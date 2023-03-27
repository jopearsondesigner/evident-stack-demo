use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::{
    api::errors::EventModelError,
    types::{interface::InterfaceConfig, Schema},
};

fn as_string(option: Option<String>) -> String {
    match option {
        Some(s) => s,
        None => String::default(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "interface/type")]
pub enum Interface {
    // TODO: Support all other interface types via Blank or some other default
    #[serde(rename = "interface.type/blank")]
    Blank {
        #[serde(rename = "interface/id")]
        id: Uuid,
        #[serde(rename = "interface/name")]
        name: String,
        #[serde(rename = "interface/description")]
        description: Option<String>,
    },
    #[serde(rename = "interface.type/rest")]
    Rest {
        #[serde(rename = "interface/id")]
        id: Uuid,
        #[serde(rename = "interface/name")]
        name: String,
        #[serde(rename = "interface/description")]
        description: Option<String>,
    },
    #[serde(rename = "interface.type/html")]
    Html {
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

impl TryFrom<Interface> for crate::types::Interface {
    type Error = EventModelError;

    fn try_from(value: Interface) -> Result<Self, Self::Error> {
        match value {
            Interface::Blank {
                id,
                name,
                description,
            } => crate::types::Interface::create(
                id,
                name,
                as_string(description),
                InterfaceConfig::None,
            ),
            Interface::Rest {
                id,
                name,
                description,
            } => crate::types::Interface::create(
                id,
                name,
                as_string(description),
                InterfaceConfig::None,
            ),
            Interface::Html {
                id,
                name,
                description,
            } => crate::types::Interface::create(
                id,
                name,
                as_string(description),
                InterfaceConfig::None,
            ),
            Interface::Figma {
                id,
                name,
                description,
                url,
                width,
                height,
            } => crate::types::Interface::create(
                id,
                name,
                as_string(description),
                InterfaceConfig::Figma(url, width, height),
            ),
            Interface::Image {
                id,
                name,
                description,
                url,
                width,
                height,
            } => crate::types::Interface::create(
                id,
                name,
                as_string(description),
                InterfaceConfig::Image(url, width, height),
            ),
            Interface::Job {
                id,
                name,
                description,
            } => crate::types::Interface::create(
                id,
                name,
                as_string(description),
                InterfaceConfig::Job,
            ),
        }
    }
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

impl TryFrom<Command> for crate::types::Command {
    type Error = EventModelError;

    fn try_from(value: Command) -> Result<Self, Self::Error> {
        crate::types::Command::create(
            value.id,
            value.name,
            as_string(value.description),
            Schema::default(),
        )
    }
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

impl TryFrom<Event> for crate::types::Event {
    type Error = EventModelError;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        crate::types::Event::create(
            value.id,
            value.name,
            as_string(value.description),
            Schema::default(),
        )
    }
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

impl TryFrom<ReadModel> for crate::types::ReadModel {
    type Error = EventModelError;

    fn try_from(value: ReadModel) -> Result<Self, Self::Error> {
        crate::types::ReadModel::create(
            value.id,
            value.name,
            as_string(value.description),
            Schema::default(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Audience {
    #[serde(rename = "audience/id")]
    id: Uuid,
    #[serde(rename = "audience/name")]
    name: String,
}

impl TryFrom<Audience> for crate::types::Audience {
    type Error = EventModelError;

    fn try_from(value: Audience) -> Result<Self, Self::Error> {
        crate::types::Audience::create(value.id, value.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stream {
    #[serde(rename = "stream/id")]
    id: Uuid,
    #[serde(rename = "stream/name")]
    name: String,
}

impl TryFrom<Stream> for crate::types::Stream {
    type Error = EventModelError;

    fn try_from(value: Stream) -> Result<Self, Self::Error> {
        crate::types::Stream::create(value.id, value.name)
    }
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
        audience: Option<Uuid>,
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
        stream: Option<Uuid>,
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

impl TryFrom<Placement> for crate::types::Placement {
    type Error = EventModelError;

    fn try_from(value: Placement) -> Result<Self, Self::Error> {
        match value {
            Placement::Interface {
                id,
                index,
                interface,
                audience,
            } => Ok(crate::types::Placement::Interface {
                id,
                index,
                interface,
                audience,
            }),
            Placement::Command { id, index, command } => Ok(crate::types::Placement::Command {
                id,
                index,
                command,
                schema: Schema::default(),
            }),
            Placement::Event {
                id,
                index,
                event,
                stream,
            } => Ok(crate::types::Placement::Event {
                id,
                index,
                event,
                stream,
                schema: Schema::default(),
            }),
            Placement::ReadModel {
                id,
                index,
                read_model,
            } => Ok(crate::types::Placement::ReadModel {
                id,
                index,
                read_model,
                schema: Schema::default(),
            }),
        }
    }
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
pub struct FlowArrow {
    #[serde(rename = "flow/from")]
    from: Uuid,
    #[serde(rename = "flow/to")]
    to: Uuid,
    #[serde(rename = "flow/from-anchor")]
    from_anchor: Option<Anchor>,
    #[serde(rename = "flow/to-anchor")]
    to_anchor: Option<Anchor>,
}

impl From<Option<Anchor>> for crate::types::flow::Anchor {
    fn from(value: Option<Anchor>) -> Self {
        match value {
            Some(Anchor::Top) => crate::types::flow::Anchor::Top,
            Some(Anchor::Left) => crate::types::flow::Anchor::Left,
            Some(Anchor::Bottom) => crate::types::flow::Anchor::Bottom,
            Some(Anchor::Right) => crate::types::flow::Anchor::Right,
            None => crate::types::flow::Anchor::None,
        }
    }
}

impl TryFrom<FlowArrow> for crate::types::FlowArrow {
    type Error = EventModelError;

    fn try_from(value: FlowArrow) -> Result<Self, Self::Error> {
        crate::types::FlowArrow::create(
            value.from,
            value.from_anchor.into(),
            value.to,
            value.to_anchor.into(),
        )
    }
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
    flows: HashMap<Uuid, FlowArrow>,
}
