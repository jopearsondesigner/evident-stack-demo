use crate::api::commands::EventModelCommand::*;
use crate::api::events::EventModelEvent::*;
use crate::api::EventModelState;
use crate::{EventModel, EventModelCreator, ModifiableEventModel};
use epoch::decider::{Decider, Evolver};
use std::fmt::Debug;
use uuid::Uuid;

pub fn creating_event_model_succeeds<c, T>(initial: <T as Evolver>::State)
where
    T: EventModel + Debug + ModifiableEventModel + Decider,
    <T as Decider>::State: EventModelState<C, T>,
    <T as Decider>::Err: std::fmt::Debug,
    <T as Evolver>::Evt: std::fmt::Debug,
{
    let mut state = initial;
    let command = Create("New Event Model".to_string());
    let events = T::decide(&state, &command).unwrap();
    assert_eq!(events.len(), 1);
    match &events[0] {
        Created(_id, name) => {
            assert_eq!(name, "New Event Model");
        }
        _ => panic!("Wrong Event Type {:?}", &events[0]),
    };

    for event in &events {
        state = EventModelState::evolve(state, event);
    }
    match state {
        EventModelState::EventModel(event_model) => {
            assert_eq!(event_model.name(), "New Event Model");
        }
        EventModelState::BeforeCreation(_) => {
            panic!("State failed to evolve: {:?}", state)
        }
    };
}

pub fn renaming_event_model_succeeds<T, C>(initial: EventModelState<T, C>)
where
    T: EventModel + Debug + ModifiableEventModel,
    C: EventModelCreator<T> + Debug,
{
    let mut state = initial;
    let id = Uuid::new_v4();

    let given_events = vec![Created(id.to_owned(), "Model".to_string())];

    for event in &given_events {
        state = EventModelState::evolve(state, event);
    }

    let when_command = Rename(id.to_owned(), "Another Name".to_string());
    let then_events = EventModelState::decide(&state, &when_command).unwrap();

    assert_eq!(then_events.len(), 1);

    match &then_events[0] {
        Renamed(_id, name) => {
            assert_eq!(name, "Another Name");
        }
        _ => panic!("Wrong Event Type {:?}", &then_events[0]),
    };

    for event in &then_events {
        state = EventModelState::evolve(state, event);
    }
    match state {
        EventModelState::EventModel(event_model) => {
            assert_eq!(event_model.name(), "Another Name");
        }
        EventModelState::BeforeCreation(_) => {
            panic!("State failed to evolve: {:?}", state)
        }
    };
}
