use std::collections::HashMap;

use event_models::{
    Described, Entity, EventModel, EventModelData, EventModelState, Named, Placement,
};
use itertools::Itertools;
use js_sys::Array;
use uuid::Uuid;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct EmptyCell;

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
    interface: event_models::Interface,
) -> InterfacePlacement {
    let config = interface.config();
    match config {
        event_models::InterfaceConfig::Blank => InterfacePlacement {
            id,
            index,
            interface: interface.id(),
            name: interface.name().into(),
            description: interface.description().to_owned(),
            kind: InterfaceType::Blank,
            url: None,
            width: None,
            height: None,
        },
        event_models::InterfaceConfig::Figma { url, width, height } => InterfacePlacement {
            id,
            index,
            interface: interface.id(),
            name: interface.name().into(),
            description: interface.description().to_owned(),
            kind: InterfaceType::Figma,
            url: Some(url.to_string()),
            width,
            height,
        },
        event_models::InterfaceConfig::Image { url, width, height } => InterfacePlacement {
            id,
            index,
            interface: interface.id(),
            name: interface.name().into(),
            description: interface.description().to_owned(),
            kind: InterfaceType::Image,
            url: Some(url.to_string()),
            width,
            height,
        },
        event_models::InterfaceConfig::Job => InterfacePlacement {
            id,
            index,
            interface: interface.id(),
            name: interface.name().into(),
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
    column_count: usize,
}

#[wasm_bindgen]
impl Audience {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn placements(&self) -> Array {
        let placements = Array::new_with_length(self.column_count as u32);
        for n in 0..self.column_count {
            if let Some(placement) = self.placements.get(&n) {
                placements.set(n as u32, JsValue::from(placement.to_owned()))
            } else {
                placements.set(n as u32, JsValue::from(EmptyCell))
            }
        }
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

fn command_placement(id: Uuid, index: usize, c: event_models::Command) -> TimelinePlacement {
    TimelinePlacement {
        id,
        index,
        component: c.id(),
        name: c.name().into(),
        description: c.description().to_owned(),
        kind: TimelinePlacementType::Command,
    }
}

fn read_model_placement(id: Uuid, index: usize, r: event_models::ReadModel) -> TimelinePlacement {
    TimelinePlacement {
        id,
        index,
        component: r.id(),
        name: r.name().into(),
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

fn event_placement(id: Uuid, index: usize, e: event_models::Event) -> EventPlacement {
    EventPlacement {
        id,
        index,
        event: e.id(),
        name: e.name().into(),
        description: e.description().to_owned(),
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Stream {
    id: Uuid,
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub name: String,
    placements: HashMap<usize, EventPlacement>,
    column_count: usize,
}

#[wasm_bindgen]
impl Stream {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn placements(&self) -> Array {
        let placements = Array::new_with_length(self.column_count as u32);
        for n in 0..self.column_count {
            if let Some(placement) = self.placements.get(&n) {
                placements.set(n as u32, JsValue::from(placement.to_owned()))
            } else {
                placements.set(n as u32, JsValue::from(EmptyCell))
            }
        }
        placements
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct FlowArrow {
    id: Uuid,
    #[wasm_bindgen(getter_with_clone)]
    pub from: FlowPort,
    #[wasm_bindgen(getter_with_clone)]
    pub to: FlowPort,
}

#[wasm_bindgen]
impl FlowArrow {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.to_string()
    }
}

impl Entity for FlowArrow {
    fn id(&self) -> Uuid {
        self.id.to_owned()
    }
}

impl From<event_models::FlowArrow> for FlowArrow {
    fn from(value: event_models::FlowArrow) -> Self {
        Self {
            id: value.id().to_owned(),
            from: value.from().to_owned().into(),
            to: value.to().to_owned().into(),
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct FlowPort {
    placement_id: Uuid,
    #[wasm_bindgen(getter_with_clone)]
    pub anchor: FlowAnchor,
}

#[wasm_bindgen]
impl FlowPort {
    #[wasm_bindgen(getter)]
    pub fn placement_id(&self) -> String {
        self.placement_id.to_string()
    }
}

impl From<event_models::Port> for FlowPort {
    fn from(value: event_models::Port) -> Self {
        Self {
            placement_id: value.placement_id().to_owned(),
            anchor: value.anchor().to_owned().into(),
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Default)]
pub enum FlowAnchor {
    #[default]
    None,
    Top,
    Left,
    Bottom,
    Right,
}

impl From<event_models::Anchor> for FlowAnchor {
    fn from(value: event_models::Anchor) -> Self {
        match value {
            event_models::Anchor::None => Self::None,
            event_models::Anchor::Top => Self::Top,
            event_models::Anchor::Left => Self::Left,
            event_models::Anchor::Bottom => Self::Bottom,
            event_models::Anchor::Right => Self::Right,
        }
    }
}

pub enum Lane {
    Audience,
    Stream,
}

impl TryFrom<&str> for Lane {
    type Error = JsValue;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "audience" => Ok(Self::Audience),
            "stream" => Ok(Self::Stream),
            &_ => Err(JsValue::from(format!(
                "Value {:?} is not a lane type",
                value
            ))),
        }
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
    #[wasm_bindgen(readonly)]
    pub column_count: usize,

    default_audience: HashMap<usize, InterfacePlacement>,
    audiences: Vec<Audience>,
    timeline: HashMap<usize, TimelinePlacement>,
    streams: Vec<Stream>,
    default_stream: HashMap<usize, EventPlacement>,
    flows: Vec<FlowArrow>,
}

#[wasm_bindgen]
impl EventModelGrid {
    pub fn id(&self) -> String {
        self.id.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn default_audience(&self) -> Array {
        let default_audience = Array::new_with_length(self.column_count as u32);
        for n in 0..self.column_count {
            if let Some(placement) = self.default_audience.get(&n) {
                default_audience.set(n as u32, JsValue::from(placement.to_owned()))
            } else {
                default_audience.set(n as u32, JsValue::from(EmptyCell))
            }
        }
        default_audience
    }

    #[wasm_bindgen(getter)]
    pub fn audiences(&self) -> Array {
        self.audiences.iter().cloned().map(JsValue::from).collect()
    }

    #[wasm_bindgen(getter)]
    pub fn timeline(&self) -> Array {
        let timeline = Array::new_with_length(self.column_count as u32);
        for n in 0..self.column_count {
            if let Some(placement) = self.timeline.get(&n) {
                timeline.set(n as u32, JsValue::from(placement.to_owned()))
            } else {
                timeline.set(n as u32, JsValue::from(EmptyCell))
            }
        }
        timeline
    }

    #[wasm_bindgen(getter)]
    pub fn streams(&self) -> Array {
        self.streams.iter().cloned().map(JsValue::from).collect()
    }

    #[wasm_bindgen(getter)]
    pub fn default_stream(&self) -> Array {
        let default_stream = Array::new_with_length(self.column_count as u32);
        for n in 0..self.column_count {
            if let Some(placement) = self.default_stream.get(&n) {
                default_stream.set(n as u32, JsValue::from(placement.to_owned()))
            } else {
                default_stream.set(n as u32, JsValue::from(EmptyCell))
            }
        }
        default_stream
    }

    #[wasm_bindgen(getter)]
    pub fn flows(&self) -> Array {
        self.flows.iter().cloned().map(JsValue::from).collect()
    }
}

const RIGHT_BUFFER: usize = 10;

impl<T: EventModel + EventModelData> From<&EventModelState<T>> for EventModelGrid {
    fn from(state: &EventModelState<T>) -> Self {
        match state {
            EventModelState::BeforeCreation => EventModelGrid {
                state: EventModelGridState::Unavailable,
                id: Uuid::new_v4(),
                name: Default::default(),
                description: Default::default(),
                default_audience: Default::default(),
                audiences: Default::default(),
                timeline: Default::default(),
                streams: Default::default(),
                default_stream: Default::default(),
                flows: Default::default(),
                column_count: 0,
            },
            EventModelState::Deleted(id) => EventModelGrid {
                state: EventModelGridState::Unavailable,
                id: *id,
                name: Default::default(),
                description: Default::default(),
                default_audience: Default::default(),
                audiences: Default::default(),
                timeline: Default::default(),
                streams: Default::default(),
                default_stream: Default::default(),
                flows: Default::default(),
                column_count: 0,
            },
            EventModelState::EventModel(model) => {
                let column_count: usize = *model
                    .placements()
                    .values()
                    .map(|p| p.index())
                    .max()
                    .unwrap_or(&0)
                    + RIGHT_BUFFER;
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
                    id: model.id(),
                    name: model.name().into(),
                    description: model.description().to_string(),
                    default_audience: grouped_audiences.remove(&None).unwrap_or_default(),
                    audiences: model
                        .audiences()
                        .iter()
                        .rev()
                        .filter_map(|audience| {
                            let audience_id = audience.id();
                            grouped_audiences
                                .remove(&Some(audience_id))
                                .map(|placements| Audience {
                                    id: audience_id.to_owned(),
                                    name: audience.name().into(),
                                    placements,
                                    column_count,
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
                                .remove(&Some(stream_id))
                                .map(|placements| Stream {
                                    id: stream_id.to_owned(),
                                    name: stream.name().into(),
                                    placements,
                                    column_count,
                                })
                        })
                        .collect(),
                    default_stream: grouped_streams.remove(&None).unwrap_or_default(),
                    flows: model.flows().values().cloned().map(Into::into).collect(),
                    column_count,
                }
            }
        }
    }
}
