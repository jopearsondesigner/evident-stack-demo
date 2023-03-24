use crate::shared::{creating_event_model_succeeds, renaming_event_model_succeeds};
use converge::{Node, OpSet};
use convergent_event_models::{ConvergentCreator, ConvergentEventModel};
use event_models::api::EventModelState;
use uuid::Uuid;

#[test]
fn creation() {
    creating_event_model_succeeds(EventModelState::BeforeCreation(ConvergentCreator));
}

#[test]
fn renaming() {
    let initial =
        ConvergentEventModel::new(Uuid::new_v4(), "foo", Node::default(), OpSet::default());

    renaming_event_model_succeeds(
        EventModelState::<ConvergentEventModel, ConvergentCreator>::EventModel(initial),
    );
}
