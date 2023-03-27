use epoch::decider::{DeciderWithContext, Evolver};
use event_models::api::commands::EventModelCommand::*;
use event_models::api::events::EventModelEvent::*;
use event_models::EventModelState;
use event_models::{EventModel, ModifiableEventModel};
use std::fmt::Debug;
use uuid::Uuid;

pub fn creating_event_model_succeeds<T>(initial: EventModelState<T>)
where
    T: EventModel + Debug + ModifiableEventModel + Clone + Send + Sync,
{
    // Assert test is starting in the correct state
    assert_matches!(&initial, EventModelState::BeforeCreation(_));

    let events =
        EventModelState::decide(&(), &initial, &Create("New Event Model".to_string())).unwrap();

    assert_eq!(events.len(), 1);
    assert_matches!(
        &events[0],
        Created(_id, name) if name == "New Event Model"
    );

    assert_matches!(
        events.iter().fold(initial.clone(), EventModelState::evolve),
        EventModelState::EventModel(event_model) if event_model.name() == "New Event Model"
    )
}

pub fn renaming_event_model_succeeds<T>(initial: EventModelState<T>)
where
    T: EventModel + Debug + ModifiableEventModel + Clone + Send + Sync,
{
    // Assert test is starting in the correct state
    assert_matches!(&initial, EventModelState::EventModel(_));

    let id = Uuid::new_v4();
    let given_events = vec![Created(id.to_owned(), "Model".to_string())];

    let state = given_events
        .iter()
        .fold(initial.clone(), EventModelState::evolve);

    let when_command = Rename(id.to_owned(), "Another Name".to_string());
    let then_events = EventModelState::decide(&(), &state, &when_command).unwrap();

    assert_eq!(then_events.len(), 1);
    assert_matches!(
        &then_events[0],
        Renamed(_id, name) if name == "Another Name"
    );

    let state = then_events
        .iter()
        .fold(initial.clone(), EventModelState::evolve);

    assert_matches!(
        state,
        EventModelState::EventModel(event_model) if event_model.name() == "Another Name"
    );
}
