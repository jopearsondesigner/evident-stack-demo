use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "interface/type")]
pub enum Interface {
    #[serde(rename = "interface.type/blank")]
    Blank {
        #[serde(rename = "interface/id")]
        id: Uuid,
        #[serde(rename = "interface/name")]
        name: String,
        #[serde(rename = "interface/description")]
        description: Option<String>,
    },
    #[serde(rename = "interface.type/figma")]
    Figma {
        #[serde(rename = "interface/id")]
        id: Uuid,
        #[serde(rename = "interface/name")]
        name: String,
        #[serde(rename = "interface/description")]
        description: Option<String>,
        #[serde(rename = "interface.type.figma/url")]
        url: Url,
        #[serde(rename = "interface/width")]
        width: Option<u32>,
        #[serde(rename = "interface/height")]
        height: Option<u32>,
    },
    #[serde(rename = "interface.type/image")]
    Image {
        #[serde(rename = "interface/id")]
        id: Uuid,
        #[serde(rename = "interface/name")]
        name: String,
        #[serde(rename = "interface/description")]
        description: Option<String>,
        #[serde(rename = "interface.type.image/url")]
        url: Url,
        #[serde(rename = "interface/width")]
        width: Option<u32>,
        #[serde(rename = "interface/height")]
        height: Option<u32>,
    },
    #[serde(rename = "interface.type/job")]
    Job {
        #[serde(rename = "interface/id")]
        id: Uuid,
        #[serde(rename = "interface/name")]
        name: String,
        #[serde(rename = "interface/description")]
        description: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Audience {
    #[serde(rename = "audience/id")]
    id: Uuid,
    #[serde(rename = "audience/name")]
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stream {
    #[serde(rename = "stream/id")]
    id: Uuid,
    #[serde(rename = "stream/name")]
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Command {
    #[serde(rename = "command/id")]
    id: Uuid,
    #[serde(rename = "command/name")]
    name: String,
    #[serde(rename = "command/description")]
    description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "event/id")]
    id: Uuid,
    #[serde(rename = "event/name")]
    name: String,
    #[serde(rename = "event/description")]
    description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadModel {
    #[serde(rename = "read-model/id")]
    id: Uuid,
    #[serde(rename = "read-model/name")]
    name: String,
    #[serde(rename = "read-model/description")]
    description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Placement {
    Interface {
        #[serde(rename = "placement/id")]
        id: Uuid,
        #[serde(rename = "placement/index")]
        index: u32,
        #[serde(rename = "interface/id")]
        interface: Uuid,
        #[serde(rename = "interface/audience")]
        audience: Uuid,
    },
    Command {
        #[serde(rename = "placement/id")]
        id: Uuid,
        #[serde(rename = "placement/index")]
        index: u32,
        #[serde(rename = "command/id")]
        command: Uuid,
    },
    Event {
        #[serde(rename = "placement/id")]
        id: Uuid,
        #[serde(rename = "placement/index")]
        index: u32,
        #[serde(rename = "event/id")]
        event: Uuid,
        #[serde(rename = "event/stream")]
        stream: Uuid,
    },
    ReadModel {
        #[serde(rename = "placement/id")]
        id: Uuid,
        #[serde(rename = "placement/index")]
        index: u32,
        #[serde(rename = "read-model/id")]
        read_model: Uuid,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Anchor {
    Top,
    Left,
    Bottom,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Flow {
    #[serde(rename = "flow/from")]
    from: Uuid,
    #[serde(rename = "flow/to")]
    to: Uuid,
    #[serde(rename = "flow/from-anchor")]
    from_anchor: Anchor,
    #[serde(rename = "flow/to-anchor")]
    to_anchor: Anchor,
}

/// Lossy, as we don't bring over Schemas, and we downgrade
/// non-supported Interface types to blank.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JsonV0_1_0BetaTransfer {
    #[serde(rename = "event-model/interfaces")]
    interfaces: HashMap<Uuid, Interface>,
    #[serde(rename = "event-model/commands")]
    commands: HashMap<Uuid, Command>,
    #[serde(rename = "event-model/events")]
    events: HashMap<Uuid, Event>,
    #[serde(rename = "event-model/read-models")]
    read_models: HashMap<Uuid, ReadModel>,
    #[serde(rename = "event-model/audiences")]
    audiences: Vec<Audience>,
    #[serde(rename = "event-model/streams")]
    streams: Vec<Stream>,
    #[serde(rename = "event-model/placements")]
    placements: HashMap<Uuid, Placement>,
    #[serde(rename = "event-model/flows")]
    flows: HashMap<Uuid, Flow>,
}
