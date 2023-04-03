use std::collections::HashMap;

use event_models::{
    implementation::in_memory::InMemoryEventModel,
    types::{Described, Entity, Named, Placement},
    EventModelData, EventModelState,
};
use itertools::Itertools;
use js_sys::Array;
use uuid::Uuid;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum InterfaceType {
    Blank,
    Figma,
    Image,
    Job,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct InterfacePlacement {
    id: Uuid,
    #[wasm_bindgen(readonly)]
    pub index: usize,
    interface: Uuid,
    #[wasm_bindgen(readonly)]
    pub name: String,
    #[wasm_bindgen(readonly)]
    pub description: String,
    #[wasm_bindgen(readonly)]
    pub kind: InterfaceType,
    #[wasm_bindgen(readonly)]
    pub url: Option<String>,
    #[wasm_bindgen(readonly)]
    pub width: Option<usize>,
    #[wasm_bindgen(readonly)]
    pub height: Option<usize>,
}

#[wasm_bindgen]
impl InterfacePlacement {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn interface(&self) -> String {
        self.interface.to_string()
    }
}

fn interface_placement(
    id: Uuid,
    index: usize,
    interface: event_models::types::Interface,
) -> InterfacePlacement {
    let config = interface.config();
    match config {
        event_models::types::InterfaceConfig::Blank => InterfacePlacement {
            id,
            index,
            interface: *interface.id(),
            name: interface.name().to_owned(),
            description: interface.description().to_owned(),
            kind: InterfaceType::Blank,
            url: None,
            width: None,
            height: None,
        },
        event_models::types::InterfaceConfig::Figma { url, width, height } => InterfacePlacement {
            id,
            index,
            interface: *interface.id(),
            name: interface.name().to_owned(),
            description: interface.description().to_owned(),
            kind: InterfaceType::Figma,
            url: Some(url.to_string()),
            width: *width,
            height: *height,
        },
        event_models::types::InterfaceConfig::Image { url, width, height } => InterfacePlacement {
            id,
            index,
            interface: *interface.id(),
            name: interface.name().to_owned(),
            description: interface.description().to_owned(),
            kind: InterfaceType::Image,
            url: Some(url.to_string()),
            width: *width,
            height: *height,
        },
        event_models::types::InterfaceConfig::Job => InterfacePlacement {
            id,
            index,
            interface: *interface.id(),
            name: interface.name().to_owned(),
            description: interface.description().to_owned(),
            kind: InterfaceType::Blank,
            url: None,
            width: None,
            height: None,
        },
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Audience {
    id: Uuid,
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub name: String,
    placements: HashMap<usize, InterfacePlacement>,
}

#[wasm_bindgen]
impl Audience {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn placements(&self) -> Array {
        let placements = Array::new_with_length(*self.placements.keys().max().unwrap_or(&0) as u32);
        self.placements.iter().for_each(|(index, placement)| {
            placements.set(*index as u32, JsValue::from(placement.to_owned()))
        });
        placements
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum TimelinePlacementType {
    Command = "command",
    ReadModel = "readModel",
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct TimelinePlacement {
    id: Uuid,
    #[wasm_bindgen(readonly)]
    pub index: usize,
    component: Uuid,
    #[wasm_bindgen(readonly)]
    pub name: String,
    #[wasm_bindgen(readonly)]
    pub description: String,
    #[wasm_bindgen(readonly)]
    pub kind: TimelinePlacementType,
}

#[wasm_bindgen]
impl TimelinePlacement {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn component(&self) -> String {
        self.component.to_string()
    }
}

fn command_placement(id: Uuid, index: usize, c: event_models::types::Command) -> TimelinePlacement {
    TimelinePlacement {
        id,
        index,
        component: *c.id(),
        name: c.name().to_owned(),
        description: c.description().to_owned(),
        kind: TimelinePlacementType::Command,
    }
}

fn read_model_placement(
    id: Uuid,
    index: usize,
    r: event_models::types::ReadModel,
) -> TimelinePlacement {
    TimelinePlacement {
        id,
        index,
        component: *r.id(),
        name: r.name().to_owned(),
        description: r.description().to_owned(),
        kind: TimelinePlacementType::ReadModel,
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct EventPlacement {
    id: Uuid,
    #[wasm_bindgen(readonly)]
    pub index: usize,
    event: Uuid,
    #[wasm_bindgen(readonly)]
    pub name: String,
    #[wasm_bindgen(readonly)]
    pub description: String,
}

#[wasm_bindgen]
impl EventPlacement {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn event(&self) -> String {
        self.event.to_string()
    }
}

fn event_placement(id: Uuid, index: usize, e: event_models::types::Event) -> EventPlacement {
    EventPlacement {
        id,
        index,
        event: *e.id(),
        name: e.name().to_owned(),
        description: e.name().to_owned(),
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Stream {
    id: Uuid,
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub name: String,
    placements: HashMap<usize, EventPlacement>,
}

#[wasm_bindgen]
impl Stream {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn placements(&self) -> Array {
        let placements = Array::new_with_length(*self.placements.keys().max().unwrap_or(&0) as u32);
        self.placements.iter().for_each(|(index, placement)| {
            placements.set(*index as u32, JsValue::from(placement.to_owned()))
        });
        placements
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum EventModelGridState {
    Available,
    Unavailable,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct EventModelGrid {
    #[wasm_bindgen(readonly)]
    pub state: EventModelGridState,
    id: Uuid,
    #[wasm_bindgen(readonly)]
    pub name: String,
    #[wasm_bindgen(readonly)]
    pub description: String,

    default_audience: HashMap<usize, InterfacePlacement>,
    audiences: Vec<Audience>,
    timeline: HashMap<usize, TimelinePlacement>,
    streams: Vec<Stream>,
    default_stream: HashMap<usize, EventPlacement>,
}

#[wasm_bindgen]
impl EventModelGrid {
    pub fn id(&self) -> String {
        self.id.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn default_audience(&self) -> Array {
        let default_audience =
            Array::new_with_length(*self.default_audience.keys().max().unwrap_or(&0) as u32);
        self.default_audience.iter().for_each(|(index, placement)| {
            default_audience.set(*index as u32, JsValue::from(placement.to_owned()))
        });
        default_audience
    }

    #[wasm_bindgen(getter)]
    pub fn audiences(&self) -> Array {
        self.audiences.iter().cloned().map(JsValue::from).collect()
    }

    #[wasm_bindgen(getter)]
    pub fn timeline(&self) -> Array {
        let timeline = Array::new_with_length(*self.timeline.keys().max().unwrap_or(&0) as u32);
        self.timeline.iter().for_each(|(index, placement)| {
            timeline.set(*index as u32, JsValue::from(placement.to_owned()))
        });
        timeline
    }

    #[wasm_bindgen(getter)]
    pub fn streams(&self) -> Array {
        self.streams.iter().cloned().map(JsValue::from).collect()
    }

    #[wasm_bindgen(getter)]
    pub fn default_stream(&self) -> Array {
        let default_stream =
            Array::new_with_length(*self.default_stream.keys().max().unwrap_or(&0) as u32);
        self.default_stream.iter().for_each(|(index, placement)| {
            default_stream.set(*index as u32, JsValue::from(placement.to_owned()))
        });
        default_stream
    }
}

impl From<EventModelState<InMemoryEventModel>> for EventModelGrid {
    fn from(state: EventModelState<InMemoryEventModel>) -> Self {
        match state {
            EventModelState::BeforeCreation(_) => EventModelGrid {
                state: EventModelGridState::Unavailable,
                id: Uuid::new_v4(),
                name: Default::default(),
                description: Default::default(),
                default_audience: Default::default(),
                audiences: Default::default(),
                timeline: Default::default(),
                streams: Default::default(),
                default_stream: Default::default(),
            },
            EventModelState::Deleted(id) => EventModelGrid {
                state: EventModelGridState::Unavailable,
                id,
                name: Default::default(),
                description: Default::default(),
                default_audience: Default::default(),
                audiences: Default::default(),
                timeline: Default::default(),
                streams: Default::default(),
                default_stream: Default::default(),
            },
            EventModelState::EventModel(model) => {
                let mut grouped_audiences: HashMap<
                    Option<Uuid>,
                    HashMap<usize, InterfacePlacement>,
                > = model
                    .placements()
                    .iter()
                    .filter_map(|(_, g)| match g {
                        Placement::Interface {
                            index,
                            audience,
                            id,
                            interface,
                        } => model.interfaces().get(interface).map(|i| {
                            (
                                audience,
                                index,
                                interface_placement(*id, *index, i.to_owned()),
                            )
                        }),
                        _ => None,
                    })
                    .into_group_map_by(|(audience, _, _)| *audience)
                    .into_iter()
                    .map(|(audience, tuples)| {
                        (
                            *audience,
                            tuples
                                .into_iter()
                                .map(|(_, index, placement)| (*index, placement))
                                .collect::<HashMap<usize, InterfacePlacement>>(),
                        )
                    })
                    .collect();

                let mut grouped_streams: HashMap<Option<Uuid>, HashMap<usize, EventPlacement>> =
                    model
                        .placements()
                        .iter()
                        .filter_map(|(_, g)| match g {
                            Placement::Event {
                                index,
                                stream,
                                id,
                                event,
                                schema: _,
                            } => model.events().get(event).map(|e| {
                                (stream, index, event_placement(*id, *index, e.to_owned()))
                            }),
                            _ => None,
                        })
                        .into_group_map_by(|(stream, _, _)| *stream)
                        .into_iter()
                        .map(|(stream, tuples)| {
                            (
                                *stream,
                                tuples
                                    .into_iter()
                                    .map(|(_, index, placement)| (*index, placement))
                                    .collect::<HashMap<usize, EventPlacement>>(),
                            )
                        })
                        .collect();

                EventModelGrid {
                    state: EventModelGridState::Available,
                    id: *model.id(),
                    name: model.name().to_string(),
                    description: model.description().to_string(),
                    default_audience: grouped_audiences.remove(&None).unwrap_or_default(),
                    audiences: model
                        .audiences()
                        .iter()
                        .rev()
                        .filter_map(|audience| {
                            let audience_id = audience.id();
                            grouped_audiences
                                .remove(&Some(*audience_id))
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
                        .filter_map(|(_, g)| match g {
                            Placement::Command {
                                index,
                                id,
                                command,
                                schema: _,
                            } => model
                                .commands()
                                .get(command)
                                .map(|c| (*index, command_placement(*id, *index, c.to_owned()))),
                            Placement::ReadModel {
                                id,
                                index,
                                read_model,
                                schema: _,
                            } => model
                                .read_models()
                                .get(read_model)
                                .map(|r| (*index, read_model_placement(*id, *index, r.to_owned()))),
                            _ => None,
                        })
                        .collect(),
                    streams: model
                        .streams()
                        .iter()
                        .filter_map(|stream| {
                            let stream_id = stream.id();
                            grouped_streams
                                .remove(&Some(*stream_id))
                                .map(|placements| Stream {
                                    id: stream_id.to_owned(),
                                    name: stream.name().to_string(),
                                    placements,
                                })
                        })
                        .collect(),
                    default_stream: grouped_streams.remove(&None).unwrap_or_default(),
                }
            }
        }
    }
}
