use crate::api::commands::EventModelCommand;
use crate::api::errors::EventModelError;
use crate::api::errors::EventModelError::IllegalState;
use crate::api::events::EventModelEvent;
use crate::types::validate_name;
use crate::{EventModel, EventModelCreator, ModifiableEventModel};
use epoch::decider::{DeciderWithContext, Evolver};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

pub mod commands;
pub mod errors;
pub mod events;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventModelState<T: EventModel, C: EventModelCreator<T>> {
    BeforeCreation(C),
    EventModel(T),
}

impl<T: EventModel + ModifiableEventModel + Send + Sync, C: EventModelCreator<T>> DeciderWithContext
    for EventModelState<T, C>
{
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
            EventModelState::EventModel(model) => match cmd {
                EventModelCommand::Create(_) => {
                    Err(IllegalState(format!("Model already exists: {:?}", model)))
                }
                EventModelCommand::Rename(id, name) => {
                    let valid_name = validate_name(name)?;
                    Ok(vec![EventModelEvent::Renamed(id.to_owned(), valid_name)])
                }
                EventModelCommand::AddToDescription(_, _, _) => {
                    todo!()
                }
                EventModelCommand::DeleteFromDescription(_, _) => {
                    todo!()
                }
                EventModelCommand::Import(_) => todo!(),
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
                EventModelCommand::Delete(_) => {
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
            },
        }
    }
}

impl<T: EventModel + ModifiableEventModel + Debug, C: EventModelCreator<T>> Evolver
    for EventModelState<T, C>
{
    type State = EventModelState<T, C>;

    type Evt = EventModelEvent;

    fn evolve(state: Self::State, event: &Self::Evt) -> Self::State {
        match state {
            EventModelState::BeforeCreation(creator) => match event {
                EventModelEvent::Created(id, name) => {
                    EventModelState::EventModel(creator.create(id.to_owned(), name.to_owned()))
                }
                _ => EventModelState::BeforeCreation(creator),
            },
            EventModelState::EventModel(mut model) => match event {
                EventModelEvent::Created(_, _) => EventModelState::EventModel(model),
                EventModelEvent::Renamed(_, name) => {
                    model.rename(name);
                    EventModelState::EventModel(model)
                }
                EventModelEvent::AddedToDescription(_, _, _) => {
                    todo!()
                }
                EventModelEvent::DeletedFromDescription(_, _) => {
                    todo!()
                }

                EventModelEvent::LaneAdded(_, _) => {
                    todo!()
                }
                EventModelEvent::LaneRenamed(_, _, _) => {
                    todo!()
                }
                EventModelEvent::Deleted(_) => {
                    todo!()
                }
                EventModelEvent::LaneReordered(_, _, _) => {
                    todo!()
                }
                EventModelEvent::LaneRemoved(_, _) => {
                    todo!()
                }
                EventModelEvent::ComponentDefined(_, _) => {
                    todo!()
                }
                EventModelEvent::ComponentPlaced(_, _) => {
                    todo!()
                }
                EventModelEvent::ComponentRenamed(_, _, _) => {
                    todo!()
                }
                EventModelEvent::PlacementMoved(_, _) => {
                    todo!()
                }
                EventModelEvent::PlacementRemoved(_, _) => {
                    todo!()
                }
                EventModelEvent::ComponentRemoved(_, _) => {
                    todo!()
                }
                EventModelEvent::AddedToComponentDescription(_, _, _, _) => {
                    todo!()
                }
                EventModelEvent::DeletedFromComponentDescription(_, _, _) => {
                    todo!()
                }
                EventModelEvent::InterfaceConfigured(_, _) => {
                    todo!()
                }
                EventModelEvent::FlowConnected(_, _) => {
                    todo!()
                }
                EventModelEvent::FlowDisconnected(_, _) => {
                    todo!()
                }
                EventModelEvent::DescriptionSet(_, _) => todo!(),
                EventModelEvent::SchemaSet(_, _) => todo!(),
                EventModelEvent::AddedToSchema(_, _, _) => todo!(),
                EventModelEvent::DeletedFromSchema(_, _) => todo!(),
                EventModelEvent::ComponentDescriptionSet(_, _, _) => todo!(),
                EventModelEvent::ComponentSchemaSet(_, _, _) => todo!(),
                EventModelEvent::AddedToComponentSchema(_, _, _, _) => todo!(),
                EventModelEvent::DeletedFromComponentSchema(_, _, _) => todo!(),
            },
        }
    }

    fn init() -> Self::State {
        Self::State::BeforeCreation(C::default())
    }
}
