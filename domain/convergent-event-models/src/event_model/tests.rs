use crate::event_model::ConvergentEventModel;
use event_models::domain::tests::{creating_event_model_succeeds, renaming_event_model_succeeds};
use event_models::domain::EventModelState;
use event_models::{EventModel, ModifiableEventModel};
use uuid::Uuid;

use event_models::types::Command;
use event_models::types::ComponentId::{
    CommandComponentId, EventComponentId, InterfaceComponentId, ReadModelComponentId,
};
use event_models::types::Event;
use event_models::types::Interface;
use event_models::types::ReadModel;
use event_models::types::{Component, Described, Entity, ModifiablyDescribed, Named};

#[test]
fn creation() {
    creating_event_model_succeeds(EventModelState::<ConvergentEventModel>::BeforeCreation);
}

#[test]
fn renaming() {
    let initial = ConvergentEventModel::new(Uuid::new_v4(), "foo");

    renaming_event_model_succeeds(EventModelState::EventModel(initial));
}
