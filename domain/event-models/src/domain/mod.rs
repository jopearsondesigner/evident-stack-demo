use crate::default::DefaultEventModel;
use crate::domain::commands::EventModelCommand;
use crate::domain::events::EventModelEvent;
use crate::types::errors::EventModelError;
use crate::types::errors::EventModelError::IllegalState;
use crate::types::validate_name;
use crate::{
    EventModel, EventModelComponentModifier, EventModelFlowModifier, EventModelId,
    EventModelLaneModifier, EventModelModifier, EventModelPlacementModifier,
    EventModelSchemaModifier,
};
use epoch::decider::{Decider, Event, Evolver};
use std::fmt::Debug;
use uuid::Uuid;

pub mod commands;
pub mod events;
pub mod read_models;

pub struct EventModelDecider;

enum EventModelState<T>
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
    EvolvingEventModel(T),
}

impl EventModelState<DefaultEventModel> {
    pub fn new(id: EventModelId, name: String) -> DefaultEventModel {
        DefaultEventModel::new(id, name)
    }
}

impl Decider for EventModelDecider {
    type Cmd = EventModelCommand;
    type Err = EventModelError;

    fn decide(state: &Self::State, cmd: &Self::Cmd) -> Result<Vec<Self::Evt>, Self::Err> {
        match state {
            None => match cmd {
                EventModelCommand::Create(name) => {
                    let valid_name = validate_name(name)?;
                    Ok(vec![EventModelEvent::Created(Uuid::new_v4(), valid_name)])
                }
                _ => Err(IllegalState(format!(
                    "Event Model not found matching command: {:?}",
                    cmd
                ))),
            },
            Some(model) => match cmd {
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
    type State = EventModelState;
    type Evt = EventModelEvent;

    fn evolve(state: Self::State, event: &Self::Evt) -> Self::State {
        match state {
            None => match event {
                EventModelEvent::Created(id, name) => {
                    Some(Self::State::new(id.to_owned(), name.to_owned()))
                }
                _ => None,
            },
            Some(mut model) => match event {
                EventModelEvent::Created(_, _) => Some(model),
                EventModelEvent::Renamed(_, name) => Some(model.renamed(name)),
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
        None
    }
}

impl Event for EventModelEvent {
    fn event_type(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::default::DefaultEventModel;
    use crate::domain::commands::EventModelCommand::*;
    use crate::domain::events::EventModelEvent::*;
    use crate::domain::EventModelDecider;
    use crate::types::Named;
    use uuid::Uuid;

    #[test]
    fn creating_event_model() {
        let mut state: Option<DefaultEventModel> = None;

        let command = Create("New Event Model".to_string());
        let decider = EventModelDecider;
        let events = decider.decide(&command, &state).unwrap();
        assert_eq!(events.len(), 1);
        match &events[0] {
            Created(_id, name) => {
                assert_eq!(name, "New Event Model");
            }
            _ => assert!(false, "Wrong Event Type {:?}", &events[0]),
        };

        for event in &events {
            state = evolve(state, event);
        }
        match state {
            Some(event_model) => {
                assert_eq!(event_model.name(), "New Event Model");
            }
            None => assert!(false, "State failed to evolve: {:?}", state),
        };
    }

    #[test]
    fn renaming_event_model() {
        let mut state: Option<DefaultEventModel> = None;
        let id = Uuid::new_v4();

        let given_events = vec![Created(id.to_owned(), "Model".to_string())];

        for event in &given_events {
            state = evolve(state, event);
        }

        let when_command = Rename(id.to_owned(), "Another Name".to_string());
        let then_result = decide(&when_command, &state).unwrap();

        assert_eq!(then_result.len(), 1);
    }
}
