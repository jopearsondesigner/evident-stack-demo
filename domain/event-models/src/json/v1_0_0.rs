use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::{
    api::errors::EventModelError,
    types::{InterfaceConfig, Schema},
};

use super::as_string;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
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
        width: Option<u32>,
        height: Option<u32>,
    },
    Image {
        id: Uuid,
        name: String,
        description: Option<String>,
        url: Url,
        width: Option<u32>,
        height: Option<u32>,
    },
    Job {
        id: Uuid,
        name: String,
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
                InterfaceConfig::Blank,
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
                InterfaceConfig::Figma { url, width, height },
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
                InterfaceConfig::Image { url, width, height },
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
    id: Uuid,
    name: String,
    description: Option<String>,
    schema: Option<String>,
}

impl TryFrom<Command> for crate::types::Command {
    type Error = EventModelError;

    fn try_from(value: Command) -> Result<Self, Self::Error> {
        crate::types::Command::create(
            value.id,
            value.name,
            as_string(value.description),
            Schema::new(as_string(value.schema)),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    id: Uuid,
    name: String,
    description: Option<String>,
    schema: Option<String>,
}

impl TryFrom<Event> for crate::types::Event {
    type Error = EventModelError;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        crate::types::Event::create(
            value.id,
            value.name,
            as_string(value.description),
            Schema::new(as_string(value.schema)),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadModel {
    id: Uuid,
    name: String,
    description: Option<String>,
    schema: Option<String>,
}

impl TryFrom<ReadModel> for crate::types::ReadModel {
    type Error = EventModelError;

    fn try_from(value: ReadModel) -> Result<Self, Self::Error> {
        crate::types::ReadModel::create(
            value.id,
            value.name,
            as_string(value.description),
            Schema::new(as_string(value.schema)),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Audience {
    id: Uuid,
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
    id: Uuid,
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
        id: Uuid,
        index: u32,
        interface: Uuid,
        audience: Option<Uuid>,
    },
    Command {
        id: Uuid,
        index: u32,
        command: Uuid,
        schema: Option<String>,
    },
    Event {
        id: Uuid,
        index: u32,
        event: Uuid,
        stream: Option<Uuid>,
        schema: Option<String>,
    },
    ReadModel {
        id: Uuid,
        index: u32,
        #[serde(rename = "read-model")]
        read_model: Uuid,
        schema: Option<String>,
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
            Placement::Command {
                id,
                index,
                command,
                schema,
            } => Ok(crate::types::Placement::Command {
                id,
                index,
                command,
                schema: Schema::new(as_string(schema)),
            }),
            Placement::Event {
                id,
                index,
                event,
                stream,
                schema,
            } => Ok(crate::types::Placement::Event {
                id,
                index,
                event,
                stream,
                schema: Schema::new(as_string(schema)),
            }),
            Placement::ReadModel {
                id,
                index,
                read_model,
                schema,
            } => Ok(crate::types::Placement::ReadModel {
                id,
                index,
                read_model,
                schema: Schema::new(as_string(schema)),
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JsonV1_0_0Transfer {}
