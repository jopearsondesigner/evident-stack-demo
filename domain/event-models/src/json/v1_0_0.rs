use std::collections::HashMap;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::{
    Audience, Command, Described, Event, EventModelData, EventModelDataTransfer, EventModelError,
    FlowArrow, InterfaceConfig, Placement, ReadModel, Stream,
};

use super::JsonExport;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Interface {
    Blank {
        id: Uuid,
        name: String,
        description: String,
    },
    Figma {
        id: Uuid,
        name: String,
        description: String,
        url: Url,
    },
    Image {
        id: Uuid,
        name: String,
        description: String,
        url: Url,
    },
    Job {
        id: Uuid,
        name: String,
        description: String,
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
            } => crate::Interface::create(id, name, description, InterfaceConfig::Blank),
            Interface::Figma {
                id,
                name,
                description,
                url,
            } => crate::Interface::create(id, name, description, InterfaceConfig::Figma { url }),
            Interface::Image {
                id,
                name,
                description,
                url,
            } => crate::Interface::create(id, name, description, InterfaceConfig::Image { url }),
            Interface::Job {
                id,
                name,
                description,
            } => crate::Interface::create(id, name, description, InterfaceConfig::Job),
        }
    }
}

impl From<&crate::Interface> for Interface {
    fn from(value: &crate::Interface) -> Self {
        let id = value.id;
        let name: String = value.name.to_owned().into();
        let description: String = value.description.to_owned();
        match &value.config {
            InterfaceConfig::Blank => Interface::Blank {
                id,
                name,
                description,
            },
            InterfaceConfig::Figma { url } => Interface::Figma {
                id,
                name,
                description,
                url: url.to_owned(),
            },
            InterfaceConfig::Image { url } => Interface::Image {
                id,
                name,
                description,
                url: url.to_owned(),
            },
            InterfaceConfig::Job => Interface::Job {
                id,
                name,
                description,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JsonV1_0_0Transfer {
    id: Uuid,
    name: String,
    description: String,
    interfaces: HashMap<Uuid, Interface>,
    commands: HashMap<Uuid, Command>,
    events: HashMap<Uuid, Event>,
    read_models: HashMap<Uuid, ReadModel>,
    audiences: Vec<Audience>,
    streams: Vec<Stream>,
    placements: HashMap<Uuid, Placement>,
    flows: HashMap<Uuid, FlowArrow>,
    #[serde(default)]
    data: String,
}

impl TryFrom<JsonV1_0_0Transfer> for EventModelDataTransfer {
    type Error = EventModelError;

    fn try_from(value: JsonV1_0_0Transfer) -> Result<Self, Self::Error> {
        Ok(EventModelDataTransfer {
            data: value.data,
            interfaces: value
                .interfaces
                .into_iter()
                .map(|(id, i)| crate::Interface::try_from(i).map(|i| (id, i)))
                .try_collect()?,
            commands: value.commands,
            events: value.events,
            read_models: value.read_models,
            audiences: value.audiences,
            streams: value.streams,
            placements: value.placements,
            flows: value.flows,
        })
    }
}

impl<T: EventModelData + Described> From<T> for JsonV1_0_0Transfer {
    fn from(value: T) -> Self {
        JsonV1_0_0Transfer {
            id: value.id(),
            name: value.name().into(),
            description: value.description().to_owned(),
            interfaces: value
                .interfaces()
                .values()
                .map(|i| (i.id, i.into()))
                .collect(),
            commands: value.commands(),
            events: value.events(),
            read_models: value.read_models(),
            audiences: value.audiences(),
            streams: value.streams(),
            placements: value.placements(),
            flows: value.flows(),
            data: value.data().to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct JsonV1_0_0Export {
    #[serde(rename = "spec-version")]
    spec_version: String,
    #[serde(rename = "event-model")]
    event_model: JsonV1_0_0Transfer,
}

impl JsonExport for JsonV1_0_0Transfer {
    fn export(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&JsonV1_0_0Export {
            spec_version: "1.0.0".to_string(),
            event_model: self.to_owned(),
        })
    }
}
