use crate::api::commands::EventModelCommand::*;
use crate::api::events::EventModelEvent::*;
use crate::api::EventModelState;
use crate::{EventModel, EventModelCreator, ModifiableEventModel};
use epoch::decider::{Decider, Evolver, Event};
use std::fmt::Debug;
use uuid::Uuid;

use super::commands::EventModelCommand;
use super::events::EventModelEvent;

pub fn creating_event_model_succeeds<C, T>(initial: <T as Evolver>::State)
where
    T: EventModel + Debug + ModifiableEventModel + Decider<Cmd = EventModelCommand, Evt = EventModelEvent>,
    T::State: EventModel + EventModelCreator<T>,
    <T as Decider>::Err: Debug,
    // <T as Decider>::Cmd: EventModelCommand
{
    let mut state = T::init();
    // let mut state = initial;

    // TODO: Associated type problem here - the impl of EventModel contains the impl of Decide and Evolve -
    // Generic test needs to be handed specific Decide/Evolve to assertain which Cmd State and Evt to test with
    // see below
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
        state = T::evolve(state, event);
    }

    // TODO: Decider / Evolver Needs to implemented storage/persistance agnostic otherwise we end up
    // With fanning associated types for generic tests on the event model
    // The test needs to know which specific generic <EventModel> were dealing with to take a stab at
    // decide and evolve - this makes it impossible to write one test that generalized over both

    // match state {
    //     EventModelState::EventModel(event_model) => {
    //         assert_eq!(event_model.name(), "New Event Model");
    //     }
    //     EventModelState::BeforeCreation(_) => {
    //         panic!("State failed to evolve: {:?}", state)
    //     }
    // };

    todo!()
}

pub fn renaming_event_model_succeeds<T, C>(initial: EventModelState<T, C>)
where
    T: EventModel + Debug + ModifiableEventModel,
    C: EventModelCreator<T> + Debug,
{
    // let mut state = initial;
    // let id = Uuid::new_v4();

    // let given_events = vec![Created(id.to_owned(), "Model".to_string())];

    // for event in &given_events {
    //     state = EventModelState::evolve(state, event);
    // }

    // let when_command = Rename(id.to_owned(), "Another Name".to_string());
    // let then_events = EventModelState::decide(&state, &when_command).unwrap();

    // assert_eq!(then_events.len(), 1);

    // match &then_events[0] {
    //     Renamed(_id, name) => {
    //         assert_eq!(name, "Another Name");
    //     }
    //     _ => panic!("Wrong Event Type {:?}", &then_events[0]),
    // };

    // for event in &then_events {
    //     state = EventModelState::evolve(state, event);
    // }
    // match state {
    //     EventModelState::EventModel(event_model) => {
    //         assert_eq!(event_model.name(), "Another Name");
    //     }
    //     EventModelState::BeforeCreation(_) => {
    //         panic!("State failed to evolve: {:?}", state)
    //     }
    // };
}
