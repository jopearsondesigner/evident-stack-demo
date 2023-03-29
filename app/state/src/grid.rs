use std::collections::HashMap;

use event_models::{
    implementation::in_memory::InMemoryEventModel,
    types::{Described, Entity, Named, Placement},
    EventModelData,
};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum InterfaceConfig {
    Blank,
    Figma {
        url: String,
        width: Option<u32>,
        height: Option<u32>,
    },
    Image {
        url: String,
        width: Option<u32>,
        height: Option<u32>,
    },
    Job,
}

impl From<&event_models::types::InterfaceConfig> for InterfaceConfig {
    fn from(config: &event_models::types::InterfaceConfig) -> Self {
        match config {
            event_models::types::InterfaceConfig::Blank => InterfaceConfig::Blank,
            event_models::types::InterfaceConfig::Figma { url, width, height } => {
                InterfaceConfig::Figma {
                    url: url.to_string(),
                    width: width.to_owned(),
                    height: height.to_owned(),
                }
            }
            event_models::types::InterfaceConfig::Image { url, width, height } => {
                InterfaceConfig::Image {
                    url: url.to_string(),
                    width: width.to_owned(),
                    height: height.to_owned(),
                }
            }
            event_models::types::InterfaceConfig::Job => InterfaceConfig::Job,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InterfacePlacement {
    id: Uuid,
    index: u32,
    interface: Uuid,
    name: String,
    description: String,
    config: InterfaceConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audience {
    id: Uuid,
    name: String,
    placements: HashMap<u32, InterfacePlacement>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimelinePlacementType {
    Command,
    ReadModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimelinePlacement {
    id: Uuid,
    index: u32,
    component: Uuid,
    name: String,
    description: String,
    #[serde(rename = "type")]
    kind: TimelinePlacementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventPlacement {
    id: Uuid,
    index: u32,
    event: Uuid,
    name: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stream {
    id: Uuid,
    name: String,
    placements: HashMap<u32, EventPlacement>,
}

pub enum GridPlacement {
    DefaultAudience(InterfacePlacement),
    Audience(Uuid, InterfacePlacement),
    Timeline(TimelinePlacement),
    Stream(Uuid, EventPlacement),
    DefaultStream(EventPlacement),
    IllegalPlacement,
}

impl From<(&InMemoryEventModel, Placement)> for GridPlacement {
    fn from((model, placement): (&InMemoryEventModel, Placement)) -> Self {
        match placement {
            Placement::Interface {
                audience,
                id,
                index,
                interface,
            } => {
                let component = &model.interfaces()[&interface];
                let placement = InterfacePlacement {
                    id,
                    interface,
                    index,
                    name: component.name().to_string(),
                    description: component.description().to_string(),
                    config: component.config().into(),
                };
                match audience {
                    Some(audience_id) => GridPlacement::Audience(audience_id, placement),
                    None => GridPlacement::DefaultAudience(placement),
                }
            }
            Placement::Command {
                id, index, command, ..
            } => {
                let component = &model.commands()[&command];
                GridPlacement::Timeline(TimelinePlacement {
                    id,
                    index,
                    component: command,
                    name: component.name().to_string(),
                    description: component.description().to_string(),
                    kind: TimelinePlacementType::Command,
                })
            }
            Placement::Event {
                id,
                index,
                event,
                stream,
                schema,
            } => {
                let component = &model.events()[&event];
                let placement = EventPlacement {
                    id,
                    index,
                    event,
                    name: component.name().to_string(),
                    description: component.description().to_string(),
                };
                match stream {
                    Some(stream_id) => GridPlacement::Stream(stream_id, placement),
                    None => GridPlacement::DefaultStream(placement),
                }
            }
            Placement::ReadModel {
                id,
                index,
                read_model,
                ..
            } => {
                let component = &model.read_models()[&read_model];
                GridPlacement::Timeline(TimelinePlacement {
                    id,
                    index,
                    component: read_model,
                    name: component.name().to_string(),
                    description: component.description().to_string(),
                    kind: TimelinePlacementType::ReadModel,
                })
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventModelGrid {
    id: Uuid,
    name: String,
    description: String,
    default_audience: HashMap<u32, InterfacePlacement>,
    audiences: Vec<Audience>,
    timeline: HashMap<u32, TimelinePlacement>,
    streams: Vec<Stream>,
    default_stream: HashMap<u32, EventPlacement>,
}

impl From<InMemoryEventModel> for EventModelGrid {
    fn from(model: InMemoryEventModel) -> Self {
        let mut grouped_audiences: HashMap<Uuid, HashMap<u32, InterfacePlacement>> = model
            .placements()
            .iter()
            .map(|(_, placement)| (&model, placement.to_owned()))
            .map_into::<GridPlacement>()
            .filter_map(|g| match g {
                GridPlacement::Audience(audience_id, placement) => {
                    Some((audience_id, placement.index, placement))
                }
                _ => None,
            })
            .into_group_map_by(|(audience_id, _, _)| *audience_id)
            .into_iter()
            .map(|(audience_id, tuples)| {
                (
                    audience_id,
                    tuples
                        .into_iter()
                        .map(|(_, index, placement)| (index, placement))
                        .collect::<HashMap<u32, InterfacePlacement>>(),
                )
            })
            .collect();

        let mut grouped_streams: HashMap<Uuid, HashMap<u32, EventPlacement>> = model
            .placements()
            .iter()
            .map(|(_, placement)| (&model, placement.to_owned()))
            .map_into::<GridPlacement>()
            .filter_map(|g| match g {
                GridPlacement::Stream(stream_id, placement) => {
                    Some((stream_id, placement.index, placement))
                }
                _ => None,
            })
            .into_group_map_by(|(stream_id, _, _)| *stream_id)
            .into_iter()
            .map(|(stream_id, tuples)| {
                (
                    stream_id,
                    tuples
                        .into_iter()
                        .map(|(_, index, placement)| (index, placement))
                        .collect::<HashMap<u32, EventPlacement>>(),
                )
            })
            .collect();
        EventModelGrid {
            id: *model.id(),
            name: model.name().to_string(),
            description: model.description().to_string(),
            default_audience: model
                .placements()
                .iter()
                .map(|(_, placement)| (&model, placement.to_owned()))
                .map_into::<GridPlacement>()
                .filter_map(|g| match g {
                    GridPlacement::DefaultAudience(placement) => Some((placement.index, placement)),
                    _ => None,
                })
                .collect(),
            audiences: model
                .audiences()
                .iter()
                .rev()
                .filter_map(|audience| {
                    let audience_id = audience.id();
                    grouped_audiences
                        .remove(audience_id)
                        .map(|placements| Audience {
                            id: audience_id.to_owned(),
                            name: audience.name().to_string(),
                            placements,
                        })
                })
                .collect(),
            timeline: model
                .placements()
                .iter()
                .map(|(_, placement)| (&model, placement.to_owned()))
                .map_into::<GridPlacement>()
                .filter_map(|g| match g {
                    GridPlacement::Timeline(placement) => Some((placement.index, placement)),
                    _ => None,
                })
                .collect(),
            streams: model
                .streams()
                .iter()
                .filter_map(|stream| {
                    let stream_id = stream.id();
                    grouped_streams.remove(stream_id).map(|placements| Stream {
                        id: stream_id.to_owned(),
                        name: stream.name().to_string(),
                        placements,
                    })
                })
                .collect(),
            default_stream: model
                .placements()
                .iter()
                .map(|(_, placement)| (&model, placement.to_owned()))
                .map_into::<GridPlacement>()
                .filter_map(|g| match g {
                    GridPlacement::DefaultStream(placement) => Some((placement.index, placement)),
                    _ => None,
                })
                .collect(),
        }
    }
}
