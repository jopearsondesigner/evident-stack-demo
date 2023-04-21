use epoch::decider::{DeciderWithContext, Evolver};
use event_models::api::commands::EventModelCommand::*;
use event_models::api::events::EventModelEvent::*;
use event_models::{EventModel, ModifiableEventModel};
use event_models::{EventModelState, Name};
use std::fmt::Debug;
use uuid::Uuid;

pub fn creating_event_model_succeeds<T>(initial: EventModelState<T>)
where
    T: EventModel + Debug + ModifiableEventModel + Send + Sync,
{
    // Assert test is starting in the correct state
    assert_matches!(&initial, EventModelState::BeforeCreation);

    let events =
        EventModelState::decide(&(), &initial, &Create("New Event Model".to_string())).unwrap();

    assert_eq!(events.len(), 1);
    assert_matches!(
        &events[0],
        Created(_id, name) if *name == Name::create("New Event Model").unwrap()
    );

    assert_matches!(
        events.iter().fold(initial, EventModelState::evolve),
        EventModelState::EventModel(event_model) if event_model.name() == Name::create("New Event Model").unwrap()
    )
}

pub fn renaming_event_model_succeeds<T>(initial: EventModelState<T>)
where
    T: EventModel + Debug + ModifiableEventModel + Send + Sync,
{
    // Assert test is starting in the correct state
    assert_matches!(&initial, EventModelState::EventModel(_));

    let id = Uuid::new_v4();
    let given_events = vec![Created(id.to_owned(), "Model".try_into().unwrap())];

    let state = given_events.iter().fold(initial, EventModelState::evolve);

    let when_command = Rename(id.to_owned(), "Another Name".try_into().unwrap());
    let then_events = EventModelState::decide(&(), &state, &when_command).unwrap();

    let another_name = Name::create("Another Name").unwrap();

    assert_eq!(then_events.len(), 1);
    assert_matches!(
        &then_events[0],
        Renamed(_id, name) if *name == another_name
    );

    let state = then_events.iter().fold(state, EventModelState::evolve);

    assert_matches!(
        state,
        EventModelState::EventModel(event_model) if event_model.name() == another_name
    );
}
