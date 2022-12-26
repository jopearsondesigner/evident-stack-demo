use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub type EventModelId = Uuid;

// TODO: CBOR-ish DTO types for all subcomponents w/ serialization functions
// TODO: From<EventModel> trait implementation (both directions)

#[derive(Debug, Serialize, Deserialize)]
pub struct EventModel {
    // pub id: EventModelId,
    // pub name: String,
    // pub description: String,
    // pub interfaces: HashMap<InterfaceId, Interface>,
    // pub commands: HashMap<CommandId, Command>,
    // pub events: HashMap<EventId, Event>,
    // pub read_models: HashMap<ReadModelId, ReadModel>,
    // pub audiences: Vec<Audience>,
    // pub streams: Vec<Stream>,
    // pub placements: HashMap<PlacementId, Placement>,
    // pub flows: HashMap<FlowId, FlowArrow>,
    // pub shared_schema: Schema,
}
