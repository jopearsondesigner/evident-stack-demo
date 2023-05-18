use crate::shared::{creating_event_model_succeeds, renaming_event_model_succeeds};
use event_models::implementation::automerge::AutomergeEventModel;

use event_models::EventModelState;
use uuid::Uuid;

#[test]
fn creation() {
    creating_event_model_succeeds(EventModelState::<AutomergeEventModel>::BeforeCreation);
}

#[test]
fn renaming() {
    let initial = AutomergeEventModel::new(&Uuid::new_v4(), &"foo".try_into().unwrap());

    renaming_event_model_succeeds(EventModelState::<AutomergeEventModel>::EventModel(initial));
}
