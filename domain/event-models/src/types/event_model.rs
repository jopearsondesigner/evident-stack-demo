use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::types::interface::{Interface, InterfaceId};
use crate::types::command::{Command, CommandId};
use crate::types::event::{Event, EventId};
use crate::types::read_model::{ReadModel, ReadModelId};
use crate::types::audience::Audience;
use crate::types::common::{Described, Entity, Named};
use crate::types::stream::Stream;
use crate::types::placement::{Placement, PlacementId};
use crate::types::flow::{FlowArrow, FlowId};
use crate::types::schema::Schema;

pub type EventModelId = Uuid;

#[derive(Debug, Serialize, Deserialize)]
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

// pub trait EventModel: Described {
//     fn interfaces(&self) -> HashMap<&InterfaceId, &Interface>;
//     fn commands(&self) -> HashMap<&CommandId, &Command>;
//     fn events(&self) -> HashMap<&EventId, &Event>;
//     fn read_models(&self) -> HashMap<&ReadModelId, &ReadModel>;
//     fn audiences(&self) -> Vec<&Audience>;
//     fn streams(&self) -> Vec<&Stream>;
//     fn placements(&self) -> HashMap<&PlacementId, &Placement>;
//     fn flows(&self) -> HashMap<&FlowId, &FlowArrow>;
//     fn shared_schema(&self) -> &Schema;
// }
//
// pub enum EventModelLifecycle<'a> {
//     BeforeCreation,
//     Active(&'a EventModel),
//     Deleted(&'a EventModelId)
// }
