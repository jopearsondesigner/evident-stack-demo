use crate::default::InMemoryEventModel;
use crate::domain::commands::EventModelCommand;
use crate::domain::events::EventModelEvent;
use crate::types::errors::EventModelError;
use crate::types::errors::EventModelError::IllegalState;
use crate::types::{validate_name, Named};
use crate::{
    EventModel, EventModelComponentModifier, EventModelFlowModifier, EventModelId,
    EventModelLaneModifier, EventModelModifier, EventModelPlacementModifier,
    EventModelSchemaModifier,
};
use epoch::decider::{Decider, Event, Evolver};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

pub mod commands;
pub mod events;
pub mod read_models;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventModelState<T>
where
    T: EventModel
        + Debug
        + EventModelModifier
        + EventModelComponentModifier
        + EventModelLaneModifier
        + EventModelPlacementModifier
        + EventModelFlowModifier
        + EventModelSchemaModifier,
{
    BeforeCreation,
    EventModel(T),
}

impl EventModelState<InMemoryEventModel> {
    pub fn new(id: EventModelId, name: String) -> Self {
        EventModelState::EventModel(InMemoryEventModel::new(id, name))
    }
}

pub struct EventModelDecider;

impl Decider for EventModelDecider {
    type Cmd = EventModelCommand;
    type Err = EventModelError;

    fn decide(state: &Self::State, cmd: &Self::Cmd) -> Result<Vec<Self::Evt>, Self::Err> {
        match state {
            EventModelState::BeforeCreation => match cmd {
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
            },
        }
    }
}

impl Evolver for EventModelDecider {
    // eagerly awaiting https://github.com/rust-lang/rust/issues/63063
    type State = EventModelState<InMemoryEventModel>;
    type Evt = EventModelEvent;

    fn evolve(mut state: Self::State, event: &Self::Evt) -> Self::State {
        match state {
            EventModelState::BeforeCreation => match event {
                EventModelEvent::Created(id, name) => {
                    EventModelState::new(id.to_owned(), name.to_owned())
                }
                _ => EventModelState::BeforeCreation,
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
            },
        }
    }

    fn init() -> Self::State {
        EventModelState::BeforeCreation
    }
}

impl Event for EventModelEvent {
    type EntityId = String;

    fn event_type(&self) -> String {
        todo!()
    }

    fn get_id(&self) -> Self::EntityId {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::commands::EventModelCommand::*;
    use crate::domain::events::EventModelEvent::*;
    use crate::domain::{EventModelDecider, EventModelState};
    use crate::{default::InMemoryEventModel, types::Named};
    use epoch::decider::{Decider, Evolver};
    use uuid::Uuid;

    #[test]
    fn creating_event_model() {
        let mut state: EventModelState<InMemoryEventModel> = EventModelState::BeforeCreation;

        let command = Create("New Event Model".to_string());
        let events = EventModelDecider::decide(&state, &command).unwrap();
        assert_eq!(events.len(), 1);
        match &events[0] {
            Created(_id, name) => {
                assert_eq!(name, "New Event Model");
            }
            _ => assert!(false, "Wrong Event Type {:?}", &events[0]),
        };

        for event in &events {
            state = EventModelDecider::evolve(state, event);
        }
        match state {
            EventModelState::EventModel(event_model) => {
                assert_eq!(event_model.name(), "New Event Model");
            }
            EventModelState::BeforeCreation => {
                assert!(false, "State failed to evolve: {:?}", state)
            }
        };
    }

    #[test]
    fn renaming_event_model() {
        let mut state: EventModelState<InMemoryEventModel> = EventModelState::BeforeCreation;
        let id = Uuid::new_v4();

        let given_events = vec![Created(id.to_owned(), "Model".to_string())];

        for event in &given_events {
            state = EventModelDecider::evolve(state, event);
        }

        let when_command = Rename(id.to_owned(), "Another Name".to_string());
        let then_result = EventModelDecider::decide(&state, &when_command).unwrap();

        assert_eq!(then_result.len(), 1);
    }
}
