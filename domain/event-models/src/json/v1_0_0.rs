use std::collections::HashMap;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::{Entity, EventModelDataTransfer, EventModelError, InterfaceConfig};

use super::{as_string, JsonExport};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Interface {
    Blank {
        id: Uuid,
        name: String,
        description: Option<String>,
    },
    Figma {
        id: Uuid,
        name: String,
        description: Option<String>,
        url: Url,
    },
    Image {
        id: Uuid,
        name: String,
        description: Option<String>,
        url: Url,
    },
    Job {
        id: Uuid,
        name: String,
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
            } => crate::Interface::create(id, name, as_string(description), InterfaceConfig::Blank),
            Interface::Figma {
                id,
                name,
                description,
                url,
            } => crate::Interface::create(
                id,
                name,
                as_string(description),
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
                as_string(description),
                InterfaceConfig::Image { url },
            ),
            Interface::Job {
                id,
                name,
                description,
            } => crate::Interface::create(id, name, as_string(description), InterfaceConfig::Job),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Command {
    id: Uuid,
    name: String,
    description: Option<String>,
}

impl TryFrom<Command> for crate::Command {
    type Error = EventModelError;

    fn try_from(value: Command) -> Result<Self, Self::Error> {
        crate::Command::create(
            value.id,
            value.name,
            as_string(value.description),
            Default::default(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    id: Uuid,
    name: String,
    description: Option<String>,
}

impl TryFrom<Event> for crate::Event {
    type Error = EventModelError;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        crate::Event::create(
            value.id,
            value.name,
            as_string(value.description),
            Default::default(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadModel {
    id: Uuid,
    name: String,
    description: Option<String>,
}

impl TryFrom<ReadModel> for crate::ReadModel {
    type Error = EventModelError;

    fn try_from(value: ReadModel) -> Result<Self, Self::Error> {
        crate::ReadModel::create(
            value.id,
            value.name,
            as_string(value.description),
            Default::default(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Audience {
    id: Uuid,
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
    id: Uuid,
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
        id: Uuid,
        index: usize,
        interface: Uuid,
        audience: Option<Uuid>,
    },
    Command {
        id: Uuid,
        index: usize,
        command: Uuid,
    },
    Event {
        id: Uuid,
        index: usize,
        event: Uuid,
        stream: Option<Uuid>,
    },
    ReadModel {
        id: Uuid,
        index: usize,
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
                schema: Default::default(),
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
                schema: Default::default(),
            }),
            Placement::ReadModel {
                id,
                index,
                read_model,
            } => Ok(crate::Placement::ReadModel {
                id,
                index,
                read_model,
                schema: Default::default(),
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
    from: Uuid,
    to: Uuid,
    from_anchor: Option<Anchor>,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JsonV1_0_0Transfer {
    interfaces: HashMap<Uuid, Interface>,
    commands: HashMap<Uuid, Command>,
    events: HashMap<Uuid, Event>,
    read_models: HashMap<Uuid, ReadModel>,
    audiences: Vec<Audience>,
    streams: Vec<Stream>,
    placements: HashMap<Uuid, Placement>,
    flows: HashMap<Uuid, FlowArrow>,
    schema: String,
}

impl TryFrom<JsonV1_0_0Transfer> for EventModelDataTransfer {
    type Error = EventModelError;

    fn try_from(value: JsonV1_0_0Transfer) -> Result<Self, Self::Error> {
        Ok(EventModelDataTransfer {
            schema: value.schema,
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

impl JsonExport for JsonV1_0_0Transfer {
    fn export(&self) -> Result<String, serde_json::Error> {
        todo!()
    }
}
