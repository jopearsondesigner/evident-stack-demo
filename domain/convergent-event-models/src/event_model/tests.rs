use crate::converge::opset::InMemoryOpSet;
use crate::converge::Node;
use crate::event_model::ConvergentEventModel;
// use event_models::api::tests::{creating_event_model_succeeds, renaming_event_model_succeeds};
use event_models::api::EventModelState;
use uuid::Uuid;

// #[test]
// fn creation() {
//     creating_event_model_succeeds(EventModelState::<>::BeforeCreation);
// }

// #[test]
// fn renaming() {
//     let initial = ConvergentEventModel::new(
//         Uuid::new_v4(),
//         "foo",
//         Node::default(),
//         InMemoryOpSet::default(),
//     );

//     renaming_event_model_succeeds(EventModelState::EventModel(initial));
// }
