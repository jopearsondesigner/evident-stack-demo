use crate::shared::{creating_event_model_succeeds, renaming_event_model_succeeds};
use converge::{random_node, Node, OpSet};
use convergent_event_models::{ConvergentCreationDetails, ConvergentEventModel};
use event_models::EventModelState;
use uuid::Uuid;

#[test]
fn creation() {
    creating_event_model_succeeds(EventModelState::<ConvergentEventModel>::BeforeCreation(
        ConvergentCreationDetails::new(random_node()),
    ));
}

#[test]
fn renaming() {
    let initial =
        ConvergentEventModel::new(Uuid::new_v4(), "foo", Node::default(), OpSet::default());

    renaming_event_model_succeeds(EventModelState::<ConvergentEventModel>::EventModel(initial));
}
