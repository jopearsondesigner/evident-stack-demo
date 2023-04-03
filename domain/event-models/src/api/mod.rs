use crate::api::commands::EventModelCommand;
use crate::api::errors::EventModelError;
use crate::api::errors::EventModelError::IllegalState;
use crate::api::events::EventModelEvent;
use crate::json::import;
use crate::types::{validate_name, Component, Lane, Schema};
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
                        events.push(placements_shifted_event);
                        Ok(events)
                    }
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
                    EventModelCommand::DefineAndPlaceInterface(_, _, _, _) => {
                        todo!()
                    }
                    EventModelCommand::DefineAndPlaceCommand(_, _, _) => {
                        todo!()
                    }
                    EventModelCommand::DefineAndPlaceEvent(_, _, _, _) => {
                        todo!()
                    }
                    EventModelCommand::DefineAndPlaceReadModel(_, _, _) => {
                        todo!()
                    }
                    EventModelCommand::RenamePlacement(_, _, _) => {
                        todo!()
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
                    EventModelCommand::RemovePlacement(_, _) => {
                        todo!()
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
                    EventModelCommand::RenameComponent(_, _, _) => {
                        todo!()
                    }
                    EventModelCommand::AddToComponentDescription(_, _, _, _) => {
                        todo!()
                    }
                    EventModelCommand::DeleteFromComponentDescription(_, _, _) => {
                        todo!()
                    }
                    EventModelCommand::ConfigureInterface(_, _) => {
                        todo!()
                    }
                    EventModelCommand::ConnectFlow(_, _, _, _, _) => {
                        todo!()
                    }
                    EventModelCommand::DisconnectFlow(_, _) => {
                        todo!()
                    }
                    EventModelCommand::SetDescription(_, _) => todo!(),
                    EventModelCommand::SetSchema(_, _) => todo!(),
                    EventModelCommand::AddToSchema(_, _, _) => todo!(),
                    EventModelCommand::DeleteFromSchema(_, _) => todo!(),
                    EventModelCommand::SetComponentDescription(_, _, _) => todo!(),
                    EventModelCommand::SetComponentSchema(_, _, _) => todo!(),
                    EventModelCommand::AddToComponentSchema(_, _, _, _) => todo!(),
                    EventModelCommand::DeleteFromComponentSchema(_, _, _) => todo!(),
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
                EventModelEvent::ComponentRenamed(_, _, _) => {
                    todo!()
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
                EventModelEvent::PlacementRemoved(_, _) => {
                    todo!()
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
