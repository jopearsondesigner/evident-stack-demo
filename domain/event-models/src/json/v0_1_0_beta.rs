use std::collections::HashMap;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::{Entity, EventModelDataTransfer, EventModelError, InterfaceConfig};

use super::option_as_string;

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

impl TryFrom<Interface> for crate::Interface {
    type Error = EventModelError;

    fn try_from(value: Interface) -> Result<Self, Self::Error> {
        match value {
            Interface::Blank {
                id,
                name,
                description,
            } => crate::Interface::create(
                id,
                name,
                option_as_string(description),
                InterfaceConfig::Blank,
            ),
            Interface::Rest {
                id,
                name,
                description,
            } => crate::Interface::create(
                id,
                name,
                option_as_string(description),
                InterfaceConfig::Blank,
            ),
            Interface::Html {
                id,
                name,
                description,
            } => crate::Interface::create(
                id,
                name,
                option_as_string(description),
                InterfaceConfig::Blank,
            ),
            Interface::Figma {
                id,
                name,
                description,
                url,
            } => crate::Interface::create(
                id,
                name,
                option_as_string(description),
                InterfaceConfig::Figma { url },
            ),
            Interface::Image {
                id,
                name,
                description,
                url,
            } => crate::Interface::create(
                id,
                name,
                option_as_string(description),
                InterfaceConfig::Image { url },
            ),
            Interface::Job {
                id,
                name,
                description,
            } => crate::Interface::create(
                id,
                name,
                option_as_string(description),
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

impl TryFrom<Command> for crate::Command {
    type Error = EventModelError;

    fn try_from(value: Command) -> Result<Self, Self::Error> {
        crate::Command::create(
            value.id,
            value.name,
            option_as_string(value.description),
            Default::default(),
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

impl TryFrom<Event> for crate::Event {
    type Error = EventModelError;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        crate::Event::create(
            value.id,
            value.name,
            option_as_string(value.description),
            Default::default(),
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

impl TryFrom<ReadModel> for crate::ReadModel {
    type Error = EventModelError;

    fn try_from(value: ReadModel) -> Result<Self, Self::Error> {
        crate::ReadModel::create(
            value.id,
            value.name,
            option_as_string(value.description),
            Default::default(),
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

impl TryFrom<Audience> for crate::Audience {
    type Error = EventModelError;

    fn try_from(value: Audience) -> Result<Self, Self::Error> {
        crate::Audience::create(value.id, value.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stream {
    #[serde(rename = "stream/id")]
    id: Uuid,
    #[serde(rename = "stream/name")]
    name: String,
}

impl TryFrom<Stream> for crate::Stream {
    type Error = EventModelError;

    fn try_from(value: Stream) -> Result<Self, Self::Error> {
        crate::Stream::create(value.id, value.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Placement {
    Interface {
        #[serde(rename = "placement/id")]
        id: Uuid,
        #[serde(rename = "placement/index")]
        index: usize,
        #[serde(rename = "interface/id")]
        interface: Uuid,
        #[serde(rename = "interface/audience")]
        audience: Option<Uuid>,
    },
    Command {
        #[serde(rename = "placement/id")]
        id: Uuid,
        #[serde(rename = "placement/index")]
        index: usize,
        #[serde(rename = "command/id")]
        command: Uuid,
    },
    Event {
        #[serde(rename = "placement/id")]
        id: Uuid,
        #[serde(rename = "placement/index")]
        index: usize,
        #[serde(rename = "event/id")]
        event: Uuid,
        #[serde(rename = "event/stream")]
        stream: Option<Uuid>,
    },
    ReadModel {
        #[serde(rename = "placement/id")]
        id: Uuid,
        #[serde(rename = "placement/index")]
        index: usize,
        #[serde(rename = "read-model/id")]
        read_model: Uuid,
    },
}

impl TryFrom<Placement> for crate::Placement {
    type Error = EventModelError;

    fn try_from(value: Placement) -> Result<Self, Self::Error> {
        match value {
            Placement::Interface {
                id,
                index,
                interface,
                audience,
            } => Ok(crate::Placement::Interface {
                id,
                index,
                interface,
                audience,
            }),
            Placement::Command { id, index, command } => Ok(crate::Placement::Command {
                id,
                index,
                command,
                data: Default::default(),
            }),
            Placement::Event {
                id,
                index,
                event,
                stream,
            } => Ok(crate::Placement::Event {
                id,
                index,
                event,
                stream,
                data: Default::default(),
            }),
            Placement::ReadModel {
                id,
                index,
                read_model,
            } => Ok(crate::Placement::ReadModel {
                id,
                index,
                read_model,
                data: Default::default(),
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

impl From<Option<Anchor>> for crate::Anchor {
    fn from(value: Option<Anchor>) -> Self {
        match value {
            Some(Anchor::Top) => crate::Anchor::Top,
            Some(Anchor::Left) => crate::Anchor::Left,
            Some(Anchor::Bottom) => crate::Anchor::Bottom,
            Some(Anchor::Right) => crate::Anchor::Right,
            None => crate::Anchor::None,
        }
    }
}

impl TryFrom<FlowArrow> for crate::FlowArrow {
    type Error = EventModelError;

    fn try_from(value: FlowArrow) -> Result<Self, Self::Error> {
        crate::FlowArrow::create(
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
    #[serde(skip)]
    data: String,
}

impl TryFrom<JsonV0_1_0BetaTransfer> for EventModelDataTransfer {
    type Error = EventModelError;

    fn try_from(value: JsonV0_1_0BetaTransfer) -> Result<Self, Self::Error> {
        Ok(EventModelDataTransfer {
            data: value.data,
            interfaces: value
                .interfaces
                .into_values()
                .map(|i| crate::Interface::try_from(i).map(|i| (i.id(), i)))
                .try_collect()?,
            commands: value
                .commands
                .into_values()
                .map(|c| crate::Command::try_from(c).map(|c| (c.id().to_owned(), c)))
                .try_collect()?,
            events: value
                .events
                .into_values()
                .map(|e| crate::Event::try_from(e).map(|e| (e.id().to_owned(), e)))
                .try_collect()?,
            read_models: value
                .read_models
                .into_values()
                .map(|r| crate::ReadModel::try_from(r).map(|r| (r.id().to_owned(), r)))
                .try_collect()?,
            audiences: value
                .audiences
                .into_iter()
                .map(crate::Audience::try_from)
                .try_collect()?,
            streams: value
                .streams
                .into_iter()
                .map(crate::Stream::try_from)
                .try_collect()?,
            placements: value
                .placements
                .into_values()
                .map(|p| crate::Placement::try_from(p).map(|p| (p.id().to_owned(), p)))
                .try_collect()?,
            flows: value
                .flows
                .into_values()
                .map(|p| crate::FlowArrow::try_from(p).map(|p| (p.id().to_owned(), p)))
                .try_collect()?,
        })
    }
}
