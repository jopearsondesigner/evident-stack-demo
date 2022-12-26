use std::collections::HashMap;
use uuid::Uuid;
use crate::domain::interface::{Interface, InterfaceId};
use crate::domain::command::{Command, CommandId};
use crate::domain::event::{Event, EventId};
use crate::domain::read_model::{ReadModel, ReadModelId};
use crate::domain::audience::Audience;
use crate::domain::common::{Described, Entity, Named};
use crate::domain::stream::Stream;
use crate::domain::placement::{Placement, PlacementId};
use crate::domain::flow::{FlowArrow, FlowId};
use crate::domain::schema::Schema;

pub type EventModelId = Uuid;

#[derive(Debug)]
pub struct EventModel {
    pub id: EventModelId,
    pub name: String,
    pub description: String,
    pub interfaces: HashMap<InterfaceId, Interface>,
    pub commands: HashMap<CommandId, Command>,
    pub events: HashMap<EventId, Event>,
    pub read_models: HashMap<ReadModelId, ReadModel>,
    pub audiences: Vec<Audience>,
    pub streams: Vec<Stream>,
    pub placements: HashMap<PlacementId, Placement>,
    pub flows: HashMap<FlowId, FlowArrow>,
    pub shared_schema: Schema,
}

impl Entity for EventModel {
    fn id(&self) -> &Uuid {
        &self.id
    }
}

impl Named for EventModel {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Described for EventModel {
    fn description(&self) -> &str {
        &self.description
    }
}
