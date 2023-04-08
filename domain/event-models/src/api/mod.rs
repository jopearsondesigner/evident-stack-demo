use crate::api::commands::EventModelCommand;
use crate::api::errors::EventModelError;
use crate::api::errors::EventModelError::IllegalState;
use crate::api::events::EventModelEvent;
use crate::json::import;
use crate::types::{
    validate_name, Command, Component, Event, Interface, Lane, Placement, ReadModel, Schema,
};
use crate::{EventModel, EventModelDataTransfer, EventModelState, ModifiableEventModel};
use epoch::decider::{DeciderWithContext, Evolver};
use std::fmt::Debug;
use uuid::Uuid;

pub mod commands;
pub mod errors;
pub mod events;

impl<T: EventModel + ModifiableEventModel + Send + Sync> DeciderWithContext for EventModelState<T> {
    type Ctx = ();

    type Cmd = EventModelCommand;

    type Err = EventModelError;

    fn decide(
        _ctx: &Self::Ctx,
        state: &Self::State,
        cmd: &Self::Cmd,
    ) -> Result<Vec<Self::Evt>, Self::Err> {
        match state {
            EventModelState::BeforeCreation(_) => match cmd {
                EventModelCommand::Create(name) => {
                    let valid_name = validate_name(name)?;
                    Ok(vec![EventModelEvent::Created(Uuid::new_v4(), valid_name)])
                }
                _ => Err(IllegalState(format!(
                    "Event Model not found matching command: {:?}",
                    cmd
                ))),
            },
            EventModelState::EventModel(model) => {
                match cmd {
                    // Event Model lifecyle and attributes
                    EventModelCommand::Create(_) => {
                        Err(IllegalState(format!("Model already exists: {:?}", model)))
                    }
                    EventModelCommand::Delete(id) => Ok(vec![EventModelEvent::Deleted(*id)]),

                    EventModelCommand::Rename(id, name) => {
                        let valid_name = validate_name(name)?;
                        Ok(vec![EventModelEvent::Renamed(*id, valid_name)])
                    }
                    EventModelCommand::AddToDescription(_, _index, _addition) => {
                        todo!()
                    }
                    EventModelCommand::DeleteFromDescription(_, _) => {
                        todo!()
                    }
                    EventModelCommand::SetDescription(_, _) => todo!(),
                    EventModelCommand::SetSchema(_, _) => todo!(),
                    EventModelCommand::AddToSchema(_, _, _) => todo!(),
                    EventModelCommand::DeleteFromSchema(_, _) => todo!(),

                    // Import
                    EventModelCommand::Import(model_id, offset, json) => {
                        let import = import(json).map_err(|e| {
                            EventModelError::SerializationError(format!(
                                "JSON parsing error {:?}",
                                e
                            ))
                        })?;
                        let data_transfer: EventModelDataTransfer = import.try_into()?;
                        let mut events: Vec<EventModelEvent> = vec![];

                        let Schema(schema_str) = data_transfer.schema;
                        events.push(EventModelEvent::AddedToSchema(
                            *model_id,
                            model.schema().0.len(),
                            schema_str,
                        ));

                        // Shift placements before adding new placements (so that the new placements don't also get shifted...
                        let placements_shifted_event = EventModelEvent::PlacementsShifted(
                            *model_id,
                            *offset,
                            data_transfer
                                .placements
                                .iter()
                                .map(|(_, placement)| *placement.index())
                                .max()
                                .unwrap_or(0),
                        );
                        events.push(placements_shifted_event);

                        events.extend(data_transfer.interfaces.into_iter().map(
                            |(_, interface)| {
                                // TODO: what to do when interface w/ id exists?
                                EventModelEvent::ComponentDefined(
                                    *model_id,
                                    Component::InterfaceComponent(interface),
                                )
                            },
                        ));
                        events.extend(data_transfer.commands.into_iter().map(|(_, command)| {
                            // TODO: what to do when command w/ id exists?
                            EventModelEvent::ComponentDefined(
                                *model_id,
                                Component::CommandComponent(command),
                            )
                        }));
                        events.extend(data_transfer.events.into_iter().map(|(_, event)| {
                            // TODO: what to do when event w/ id exists?
                            EventModelEvent::ComponentDefined(
                                *model_id,
                                Component::EventComponent(event),
                            )
                        }));
                        events.extend(data_transfer.read_models.into_iter().map(
                            |(_, read_model)| {
                                // TODO: what to do when read model w/ id exists?
                                EventModelEvent::ComponentDefined(
                                    *model_id,
                                    Component::ReadModelComponent(read_model),
                                )
                            },
                        ));
                        let mut audience_insertion_index = model.audiences().len() - 1;
                        events.extend(data_transfer.audiences.into_iter().map(|audience| {
                            // TODO: what to do when audience w/ id exists?
                            audience_insertion_index += 1;
                            EventModelEvent::LaneAdded(
                                *model_id,
                                Lane::Audience(audience),
                                audience_insertion_index,
                            )
                        }));
                        let mut stream_insertion_index = model.streams().len() - 1;
                        events.extend(data_transfer.streams.into_iter().map(|stream| {
                            // TODO: what to do when stream w/ id exists?
                            stream_insertion_index += 1;
                            EventModelEvent::LaneAdded(
                                *model_id,
                                Lane::Stream(stream),
                                stream_insertion_index,
                            )
                        }));

                        events.extend(data_transfer.placements.into_iter().map(
                            |(_, mut placement)| {
                                // TODO: what to do when placement w/ id and/or index,component_id exists?
                                //       currently we generate a new id for each placement, to allow multiple imports
                                placement.shift_right(*offset);
                                EventModelEvent::ComponentPlaced(*model_id, placement)
                            },
                        ));
                        events.extend(data_transfer.flows.into_iter().map(|(_, flow_arrow)| {
                            // Automatically deduped via id generation
                            EventModelEvent::FlowConnected(*model_id, flow_arrow)
                        }));

                        Ok(events)
                    }

                    // Lanes
                    EventModelCommand::AddAudience(_, _, _) => {
                        todo!()
                    }
                    EventModelCommand::RenameAudience(_, _, _) => {
                        todo!()
                    }
                    EventModelCommand::ReorderAudience(_, _, _) => {
                        todo!()
                    }
                    EventModelCommand::RemoveAudience(_, _) => {
                        todo!()
                    }
                    EventModelCommand::AddStream(_, _, _) => {
                        todo!()
                    }
                    EventModelCommand::RenameStream(_, _, _) => {
                        todo!()
                    }
                    EventModelCommand::ReorderStream(_, _, _) => {
                        todo!()
                    }
                    EventModelCommand::RemoveStream(_, _) => {
                        todo!()
                    }

                    // Placements
                    EventModelCommand::DefineAndPlaceInterface(model_id, name, index, audience) => {
                        let interface_id = Uuid::new_v4();
                        let interface = Interface::create(
                            interface_id,
                            name.to_owned(),
                            Default::default(),
                            Default::default(),
                        )?;
                        let component = Component::InterfaceComponent(interface);
                        let placement = Placement::Interface {
                            id: Uuid::new_v4(),
                            index: *index,
                            interface: interface_id,
                            audience: *audience,
                        };
                        Ok(vec![
                            EventModelEvent::ComponentDefined(*model_id, component),
                            EventModelEvent::ComponentPlaced(*model_id, placement),
                        ])
                    }
                    EventModelCommand::DefineAndPlaceCommand(model_id, name, index) => {
                        let command_id = Uuid::new_v4();
                        let command = Command::create(
                            command_id,
                            name.to_owned(),
                            Default::default(),
                            Default::default(),
                        )?;
                        let component = Component::CommandComponent(command);
                        let placement = Placement::Command {
                            id: Uuid::new_v4(),
                            index: *index,
                            command: command_id,
                            schema: Default::default(),
                        };
                        Ok(vec![
                            EventModelEvent::ComponentDefined(*model_id, component),
                            EventModelEvent::ComponentPlaced(*model_id, placement),
                        ])
                    }
                    EventModelCommand::DefineAndPlaceEvent(model_id, name, index, stream) => {
                        let event_id = Uuid::new_v4();
                        let event = Event::create(
                            event_id,
                            name.to_owned(),
                            Default::default(),
                            Default::default(),
                        )?;
                        let component = Component::EventComponent(event);
                        let placement = Placement::Event {
                            id: Uuid::new_v4(),
                            index: *index,
                            event: event_id,
                            stream: *stream,
                            schema: Default::default(),
                        };
                        Ok(vec![
                            EventModelEvent::ComponentDefined(*model_id, component),
                            EventModelEvent::ComponentPlaced(*model_id, placement),
                        ])
                    }
                    EventModelCommand::DefineAndPlaceReadModel(model_id, name, index) => {
                        let read_model_id = Uuid::new_v4();
                        let read_model = ReadModel::create(
                            read_model_id,
                            name.to_owned(),
                            Default::default(),
                            Default::default(),
                        )?;
                        let component = Component::ReadModelComponent(read_model);
                        let placement = Placement::ReadModel {
                            id: Uuid::new_v4(),
                            index: *index,
                            read_model: read_model_id,
                            schema: Default::default(),
                        };
                        Ok(vec![
                            EventModelEvent::ComponentDefined(*model_id, component),
                            EventModelEvent::ComponentPlaced(*model_id, placement),
                        ])
                    }
                    EventModelCommand::RenamePlacement(model_id, placement_id, name) => {
                        let placement = model.placements().get(placement_id).ok_or_else(|| {
                            EventModelError::ModificationError(format!(
                                "No placement found with id {:?}",
                                placement_id
                            ))
                        })?;
                        Ok(vec![EventModelEvent::ComponentRenamed(
                            *model_id,
                            placement.component_id(),
                            name.to_string(),
                        )])
                    }
                    EventModelCommand::MoveInterfacePlacement(_, _, _, _) => {
                        todo!()
                    }
                    EventModelCommand::MoveTimelinePlacement(_, _, _) => {
                        todo!()
                    }
                    EventModelCommand::MoveEventPlacement(_, _, _, _) => {
                        todo!()
                    }
                    EventModelCommand::RemovePlacement(model_id, placement_id) => {
                        Ok(vec![EventModelEvent::PlacementRemoved(
                            *model_id,
                            *placement_id,
                        )])
                    }
                    EventModelCommand::DuplicateInterfacePlacement(_, _, _, _) => {
                        todo!()
                    }
                    EventModelCommand::DuplicateTimelinePlacement(_, _, _) => {
                        todo!()
                    }
                    EventModelCommand::DuplicateEventPlacement(_, _, _, _) => {
                        todo!()
                    }

                    // Components
                    EventModelCommand::RenameComponent(_, _, _) => {
                        todo!()
                    }
                    EventModelCommand::SetComponentDescription(_, _, _) => todo!(),
                    EventModelCommand::SetComponentSchema(_, _, _) => todo!(),
                    EventModelCommand::AddToComponentSchema(_, _, _, _) => todo!(),
                    EventModelCommand::DeleteFromComponentSchema(_, _, _) => todo!(),
                    EventModelCommand::AddToComponentDescription(_, _, _, _) => {
                        todo!()
                    }
                    EventModelCommand::DeleteFromComponentDescription(_, _, _) => {
                        todo!()
                    }
                    EventModelCommand::ConfigureInterface(_, _) => {
                        todo!()
                    }

                    // Flows
                    EventModelCommand::ConnectFlow(_, _, _, _, _) => {
                        todo!()
                    }
                    EventModelCommand::DisconnectFlow(_, _) => {
                        todo!()
                    }
                }
            }
            EventModelState::Deleted(_) => Ok(vec![]),
        }
    }
}

impl<T: EventModel + ModifiableEventModel + Debug> Evolver for EventModelState<T> {
    type State = EventModelState<T>;

    type Evt = EventModelEvent;

    fn evolve(state: Self::State, event: &Self::Evt) -> Self::State {
        match state {
            EventModelState::BeforeCreation(details) => match event {
                EventModelEvent::Created(id, name) => EventModelState::EventModel(T::create(
                    EventModelState::BeforeCreation(details),
                    id.to_owned(),
                    name.to_owned(),
                )),
                _ => EventModelState::BeforeCreation(details),
            },
            EventModelState::EventModel(mut model) => match event {
                EventModelEvent::Created(_, _) => EventModelState::EventModel(model),
                EventModelEvent::Deleted(id) => EventModelState::Deleted(*id),

                EventModelEvent::Renamed(_, name) => {
                    model.rename(name);
                    EventModelState::EventModel(model)
                }
                EventModelEvent::AddedToDescription(_, index, addition) => {
                    model.add_to_description(*index, addition);
                    EventModelState::EventModel(model)
                }
                EventModelEvent::DeletedFromDescription(_, index, count) => {
                    model.delete_from_description(*index, *count);
                    EventModelState::EventModel(model)
                }
                EventModelEvent::DescriptionSet(_, _) => todo!(),
                EventModelEvent::AddedToSchema(_, index, addition) => {
                    model.add_to_schema(*index, addition);
                    EventModelState::EventModel(model)
                }
                EventModelEvent::DeletedFromSchema(_, index, count) => {
                    model.delete_from_schema(*index, *count);
                    EventModelState::EventModel(model)
                }
                EventModelEvent::SchemaSet(_, _) => todo!(),

                EventModelEvent::LaneAdded(_, lane, index) => {
                    model.lane_added(lane.to_owned(), *index);
                    EventModelState::EventModel(model)
                }
                EventModelEvent::LaneRenamed(_, _, _) => {
                    todo!()
                }
                EventModelEvent::LaneReordered(_, _, _) => {
                    todo!()
                }
                EventModelEvent::LaneRemoved(_, _) => {
                    todo!()
                }
                EventModelEvent::ComponentDefined(_, component) => {
                    model.component_defined(component.to_owned());
                    EventModelState::EventModel(model)
                }
                EventModelEvent::ComponentRenamed(_, component_id, name) => {
                    model.component_renamed(component_id, name);
                    EventModelState::EventModel(model)
                }
                EventModelEvent::AddedToComponentDescription(_, _, _, _) => {
                    todo!()
                }
                EventModelEvent::DeletedFromComponentDescription(_, _, _) => {
                    todo!()
                }
                EventModelEvent::ComponentDescriptionSet(_, _, _) => todo!(),
                EventModelEvent::AddedToComponentSchema(_, _, _, _) => todo!(),
                EventModelEvent::DeletedFromComponentSchema(_, _, _) => todo!(),
                EventModelEvent::ComponentSchemaSet(_, _, _) => todo!(),
                EventModelEvent::InterfaceConfigured(_, _) => {
                    todo!()
                }
                EventModelEvent::ComponentRemoved(_, _) => {
                    todo!()
                }

                EventModelEvent::ComponentPlaced(_, placement) => {
                    model.component_placed(placement);
                    EventModelState::EventModel(model)
                }
                EventModelEvent::PlacementMoved(_, _) => {
                    todo!()
                }
                EventModelEvent::PlacementRemoved(_, placement_id) => {
                    model.placement_removed(placement_id);
                    EventModelState::EventModel(model)
                }
                EventModelEvent::PlacementsShifted(_, offset, width) => {
                    model.placements_shifted(offset, width);
                    EventModelState::EventModel(model)
                }

                EventModelEvent::FlowConnected(_, flow_arrow) => {
                    model.plus_flow(flow_arrow.to_owned());
                    EventModelState::EventModel(model)
                }
                EventModelEvent::FlowDisconnected(_, _) => {
                    todo!()
                }
            },
            EventModelState::Deleted(_) => state,
        }
    }
}
