use crate::api::commands::EventModelCommand::*;
use crate::api::events::EventModelEvent::*;
use crate::api::EventModelState;
use crate::{EventModel, EventModelCreator, ModifiableEventModel};
use epoch::decider::{DeciderWithContext, Evolver};
use std::fmt::Debug;
use uuid::Uuid;

pub fn creating_event_model_succeeds<C, T>(initial: EventModelState<T, C>)
where
    T: EventModel + Debug + ModifiableEventModel + Clone,
    C: EventModelCreator<T> + Clone,
{
    let command = Create("New Event Model".to_string());
    let events = EventModelState::decide(&(), &initial, &command).unwrap();
    assert_eq!(events.len(), 1);

    match &events[0] {
        Created(_id, name) => {
            assert_eq!(name, "New Event Model");
        }
        _ => panic!("Wrong Event Type {:?}", &events[0]),
    };

    match events.iter().fold(initial.clone(), EventModelState::evolve) {
        EventModelState::EventModel(event_model) => {
            assert_eq!(event_model.name(), "New Event Model");
        }
        EventModelState::BeforeCreation(_) => {
            panic!("State failed to evolve: {:?}", initial)
        }
    };
}

pub fn renaming_event_model_succeeds<T, C>(initial: EventModelState<T, C>)
where
    T: EventModel + Debug + ModifiableEventModel + Clone,
    C: EventModelCreator<T> + Debug + Clone,
{
    // let mut state = initial;
    let id = Uuid::new_v4();

    let given_events = vec![Created(id.to_owned(), "Model".to_string())];

    let state = given_events
        .iter()
        .fold(initial.clone(), EventModelState::evolve);

    let when_command = Rename(id.to_owned(), "Another Name".to_string());
    let then_events = EventModelState::decide(&(), &state, &when_command).unwrap();

    assert_eq!(then_events.len(), 1);

    match &then_events[0] {
        Renamed(_id, name) => {
            assert_eq!(name, "Another Name");
        }
        _ => panic!("Wrong Event Type {:?}", &then_events[0]),
    };

    let state = then_events
        .iter()
        .fold(initial.clone(), EventModelState::evolve);

    match state {
        EventModelState::EventModel(event_model) => {
            assert_eq!(event_model.name(), "Another Name");
        }
        EventModelState::BeforeCreation(_) => {
            panic!("State failed to evolve: {:?}", state)
        }
    };
}
