use std::collections::HashMap;
use uuid::Uuid;
use crate::types::interface::{Interface, InterfaceId};
use crate::types::command::{Command, CommandId};
use crate::types::event::{Event, EventId};
use crate::types::read_model::{ReadModel, ReadModelId};
use crate::types::audience::Audience;
use crate::types::common::Described;
use crate::types::stream::Stream;
use crate::types::placement::{Placement, PlacementId};
use crate::types::flow::{FlowArrow, FlowId};
use crate::types::schema::Schema;

pub type EventModelId = Uuid;

pub trait EventModel: Described {
    fn interfaces(&self) -> HashMap<&InterfaceId, &Interface>;
    fn commands(&self) -> HashMap<&CommandId, &Command>;
    fn events(&self) -> HashMap<&EventId, &Event>;
    fn read_models(&self) -> HashMap<&ReadModelId, &ReadModel>;
    fn audiences(&self) -> Vec<&Audience>;
    fn streams(&self) -> Vec<&Stream>;
    fn placements(&self) -> HashMap<&PlacementId, &Placement>;
    fn flows(&self) -> HashMap<&FlowId, &FlowArrow>;
    fn shared_schema(&self) -> &Schema;
}

pub enum EventModelLifecycle<'a> {
    BeforeCreation,
    Active(&'a dyn EventModel),
    Deleted(&'a EventModelId)
}
