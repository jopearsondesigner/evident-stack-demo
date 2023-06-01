use std::collections::{BTreeMap, HashMap};

use event_models::{
    Described, Entity, EventModel, EventModelData, EventModelState, Named, Placement,
};
use itertools::Itertools;
use js_sys::Array;
use uuid::Uuid;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::parse_uuid;

const DEFAULT_AUDIENCE_NAME: &str = "Default Audience";
const TIMELINE_NAME: &str = "Timeline";
const DEFAULT_STREAM_NAME: &str = "Default Stream";

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
pub struct InterfaceConfig {
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
#[derive(Debug, Clone)]
pub enum GridPlacementType {
    Interface = "interface",
    Command = "command",
    Event = "event",
    ReadModel = "readModel",
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct GridPlacement {
    #[wasm_bindgen(readonly)]
    pub kind: GridPlacementType,
    id: Uuid,
    component_id: Uuid,
    #[wasm_bindgen(readonly)]
    pub index: usize,
    #[wasm_bindgen(readonly)]
    pub name: String,
    #[wasm_bindgen(readonly)]
    pub description: String,
    #[wasm_bindgen(readonly)]
    pub interface_config: Option<InterfaceConfig>,
}

#[wasm_bindgen]
impl GridPlacement {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn component_id(&self) -> String {
        self.component_id.to_string()
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum CellType {
    Interface = "interface",
    Timeline = "timeline",
    Event = "event",
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct Cell {
    #[wasm_bindgen(readonly)]
    pub kind: CellType,
    #[wasm_bindgen(readonly)]
    pub row: usize,
    #[wasm_bindgen(readonly)]
    pub column: usize,
    // Sorts placements on id, which is fine, since it's stable, and multiple placements only
    // caused by collaboration
    placements: BTreeMap<Uuid, GridPlacement>,
    audience: Option<Uuid>,
    stream: Option<Uuid>,
}

#[wasm_bindgen]
impl Cell {
    #[wasm_bindgen(getter)]
    pub fn placements(&self) -> Array {
        self.placements
            .values()
            .map(|p| JsValue::from(p.to_owned()))
            .collect()
    }

    #[wasm_bindgen(getter)]
    pub fn audience(&self) -> Option<String> {
        self.audience.map(|id| id.to_string())
    }

    #[wasm_bindgen(getter)]
    pub fn stream(&self) -> Option<String> {
        self.stream.map(|id| id.to_string())
    }

    pub fn is_empty(&self) -> bool {
        self.placements.is_empty()
    }

    pub fn topmost_placement(&self) -> Option<GridPlacement> {
        self.placements.values().last().cloned()
    }
}

fn interface_placement(
    id: Uuid,
    index: usize,
    interface: event_models::Interface,
) -> GridPlacement {
    let config = match interface.config() {
        event_models::InterfaceConfig::Blank => InterfaceConfig {
            kind: InterfaceType::Blank,
            url: None,
            width: None,
            height: None,
        },
        event_models::InterfaceConfig::Figma { url, width, height } => InterfaceConfig {
            kind: InterfaceType::Figma,
            url: Some(url.to_string()),
            width,
            height,
        },
        event_models::InterfaceConfig::Image { url, width, height } => InterfaceConfig {
            kind: InterfaceType::Image,
            url: Some(url.to_string()),
            width,
            height,
        },
        event_models::InterfaceConfig::Job => InterfaceConfig {
            kind: InterfaceType::Blank,
            url: None,
            width: None,
            height: None,
        },
    };
    GridPlacement {
        kind: GridPlacementType::Interface,
        id,
        index,
        component_id: interface.id(),
        name: interface.name().into(),
        description: interface.description().to_owned(),
        interface_config: Some(config),
    }
}

fn command_placement(id: Uuid, index: usize, c: event_models::Command) -> GridPlacement {
    GridPlacement {
        kind: GridPlacementType::Command,
        id,
        index,
        component_id: c.id(),
        name: c.name().into(),
        description: c.description().to_owned(),
        interface_config: None,
    }
}

fn read_model_placement(id: Uuid, index: usize, r: event_models::ReadModel) -> GridPlacement {
    GridPlacement {
        kind: GridPlacementType::ReadModel,
        id,
        index,
        component_id: r.id(),
        name: r.name().into(),
        description: r.description().to_owned(),
        interface_config: None,
    }
}

fn event_placement(id: Uuid, index: usize, e: event_models::Event) -> GridPlacement {
    GridPlacement {
        kind: GridPlacementType::Event,
        id,
        index,
        component_id: e.id(),
        name: e.name().into(),
        description: e.description().to_owned(),
        interface_config: None,
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
    #[wasm_bindgen(getter_with_clone)]
    pub kind: String,
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
            kind: "FlowPort".to_string(),
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

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum GridLaneType {
    DefaultAudience,
    Audience,
    Timeline,
    Stream,
    DefaultStream,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct GridLane {
    id: Option<Uuid>,
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub kind: GridLaneType,
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub row: usize,
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub name: String,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl GridLane {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> Option<String> {
        self.id.map(|i| i.to_string())
    }

    #[wasm_bindgen(getter)]
    pub fn cells(&self) -> Array {
        self.cells
            .iter()
            .map(|c| JsValue::from(c.to_owned()))
            .collect()
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
    #[wasm_bindgen(readonly)]
    pub row_count: usize,

    default_audience: GridLane,
    audiences: Vec<GridLane>,
    timeline: GridLane,
    streams: Vec<GridLane>,
    default_stream: GridLane,

    flows: Vec<FlowArrow>,
    placements: HashMap<Uuid, GridPlacement>,
}

#[wasm_bindgen]
impl EventModelGrid {
    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn placement_by_id(&self, maybe_placement_id: &str) -> Option<GridPlacement> {
        if let Ok(placement_id) = parse_uuid(maybe_placement_id.to_string()) {
            self.placements.get(&placement_id).cloned()
        } else {
            None
        }
    }

    pub fn cell_by_row_col(&self, row: usize, col: usize) -> Option<Cell> {
        let default_audience_index: usize = self.default_audience.row;
        let timeline_index = self.timeline.row;
        let default_stream_index = self.default_stream.row;

        match row {
            x if x == default_audience_index => self.default_audience.cells.get(col).cloned(),
            x if x > default_audience_index && x < timeline_index => self
                .audiences
                .get(x)
                .and_then(|a| a.cells.get(col))
                .cloned(),
            x if x == timeline_index => self.timeline.cells.get(col).cloned(),
            x if x > default_stream_index && x < default_stream_index => {
                self.streams.get(x).and_then(|a| a.cells.get(col)).cloned()
            }
            x if x == default_stream_index => self.default_stream.cells.get(col).cloned(),
            _ => None,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn default_audience(&self) -> Array {
        self.default_audience
            .cells
            .iter()
            .map(|c| JsValue::from(c.to_owned()))
            .collect()
    }

    #[wasm_bindgen(getter)]
    pub fn audiences(&self) -> Array {
        self.audiences.iter().cloned().map(JsValue::from).collect()
    }

    #[wasm_bindgen(getter)]
    pub fn timeline(&self) -> Array {
        self.timeline
            .cells
            .iter()
            .map(|c| JsValue::from(c.to_owned()))
            .collect()
    }

    #[wasm_bindgen(getter)]
    pub fn streams(&self) -> Array {
        self.streams.iter().cloned().map(JsValue::from).collect()
    }

    #[wasm_bindgen(getter)]
    pub fn default_stream(&self) -> Array {
        self.default_stream
            .cells
            .iter()
            .map(|c| JsValue::from(c.to_owned()))
            .collect()
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
                column_count: Default::default(),
                row_count: Default::default(),
                default_audience: GridLane {
                    id: None,
                    kind: GridLaneType::DefaultAudience,
                    row: 0,
                    name: DEFAULT_AUDIENCE_NAME.to_string(),
                    cells: Default::default(),
                },
                audiences: Default::default(),
                timeline: GridLane {
                    id: None,
                    kind: GridLaneType::Timeline,
                    row: 1,
                    name: TIMELINE_NAME.to_string(),
                    cells: Default::default(),
                },
                streams: Default::default(),
                default_stream: GridLane {
                    id: None,
                    kind: GridLaneType::DefaultStream,
                    row: 2,
                    name: DEFAULT_STREAM_NAME.to_string(),
                    cells: Default::default(),
                },
                flows: Default::default(),
                placements: Default::default(),
            },
            EventModelState::Deleted(id) => EventModelGrid {
                state: EventModelGridState::Unavailable,
                id: *id,
                name: Default::default(),
                description: Default::default(),
                column_count: Default::default(),
                row_count: Default::default(),
                default_audience: GridLane {
                    id: None,
                    kind: GridLaneType::DefaultAudience,
                    row: 0,
                    name: DEFAULT_AUDIENCE_NAME.to_string(),
                    cells: Default::default(),
                },
                audiences: Default::default(),
                timeline: GridLane {
                    id: None,
                    kind: GridLaneType::Timeline,
                    row: 1,
                    name: TIMELINE_NAME.to_string(),
                    cells: Default::default(),
                },
                streams: Default::default(),
                default_stream: GridLane {
                    id: None,
                    kind: GridLaneType::DefaultStream,
                    row: 2,
                    name: DEFAULT_STREAM_NAME.to_string(),
                    cells: Default::default(),
                },
                flows: Default::default(),
                placements: Default::default(),
            },
            EventModelState::EventModel(model) => {
                let column_count: usize = *model
                    .placements()
                    .values()
                    .map(|p| p.index())
                    .max()
                    .unwrap_or(&0)
                    + RIGHT_BUFFER;

                // Initialize empty grid cells
                let mut row: usize = 0;
                let mut default_audience = GridLane {
                    id: None,
                    kind: GridLaneType::DefaultAudience,
                    cells: Vec::with_capacity(column_count),
                    row,
                    name: DEFAULT_AUDIENCE_NAME.to_string(),
                };
                for column in 0..column_count {
                    default_audience.cells.push(Cell {
                        kind: CellType::Interface,
                        row,
                        column,
                        placements: Default::default(),
                        audience: None,
                        stream: None,
                    })
                }
                row += 1;

                let mut audiences: HashMap<Uuid, GridLane> = HashMap::new();
                for audience in model.audiences() {
                    let mut cells = Vec::with_capacity(column_count);
                    let audience_id = audience.id();
                    for column in 0..column_count {
                        cells.push(Cell {
                            kind: CellType::Interface,
                            row,
                            column,
                            placements: Default::default(),
                            audience: Some(audience_id.to_owned()),
                            stream: None,
                        })
                    }
                    audiences.insert(
                        audience_id.to_owned(),
                        GridLane {
                            id: Some(audience_id.to_owned()),
                            kind: GridLaneType::Audience,
                            row,
                            name: audience.name().into(),
                            cells,
                        },
                    );
                    row += 1;
                }

                let mut timeline = GridLane {
                    id: None,
                    kind: GridLaneType::Timeline,
                    cells: Vec::with_capacity(column_count),
                    row,
                    name: TIMELINE_NAME.to_string(),
                };
                for column in 0..column_count {
                    timeline.cells.push(Cell {
                        kind: CellType::Timeline,
                        row,
                        column,
                        placements: Default::default(),
                        audience: None,
                        stream: None,
                    })
                }
                row += 1;

                let mut streams: HashMap<Uuid, GridLane> = HashMap::new();
                for stream in model.streams() {
                    let mut cells = Vec::with_capacity(column_count);
                    let stream_id = stream.id();
                    for column in 0..column_count {
                        cells.push(Cell {
                            kind: CellType::Event,
                            row,
                            column,
                            placements: Default::default(),
                            audience: None,
                            stream: Some(stream_id.to_owned()),
                        })
                    }
                    streams.insert(
                        stream_id.to_owned(),
                        GridLane {
                            id: Some(stream_id.to_owned()),
                            kind: GridLaneType::Stream,
                            row,
                            name: stream.name().into(),
                            cells,
                        },
                    );
                    row += 1;
                }

                let mut default_stream = GridLane {
                    id: None,
                    kind: GridLaneType::DefaultStream,
                    cells: Vec::with_capacity(column_count),
                    row,
                    name: DEFAULT_STREAM_NAME.to_string(),
                };
                for column in 0..column_count {
                    default_stream.cells.push(Cell {
                        kind: CellType::Event,
                        row,
                        column,
                        placements: Default::default(),
                        audience: None,
                        stream: None,
                    })
                }
                let row_count = row + 1;

                // Fill grid cells with placements
                let mut placements: HashMap<Uuid, GridPlacement> =
                    HashMap::with_capacity(model.placements().len());

                for placement in model.placements().values() {
                    match placement {
                        Placement::Interface {
                            id,
                            index,
                            interface,
                            audience,
                        } => {
                            if let Some(component) = model.interfaces().get(interface) {
                                let grid_placement =
                                    interface_placement(*id, *index, component.to_owned());
                                placements.insert(*id, grid_placement.to_owned());
                                match audience {
                                    Some(audience_id) => {
                                        if let Some(a) = audiences.get_mut(audience_id) {
                                            if let Some(cell) = a.cells.get_mut(*index) {
                                                cell.placements.insert(*id, grid_placement);
                                            }
                                        }
                                    }
                                    None => {
                                        if let Some(cell) = default_audience.cells.get_mut(*index) {
                                            cell.placements.insert(*id, grid_placement);
                                        }
                                    }
                                }
                            }
                        }
                        Placement::Command {
                            id, index, command, ..
                        } => {
                            if let Some(component) = model.commands().get(command) {
                                let grid_placement =
                                    command_placement(*id, *index, component.to_owned());
                                placements.insert(*id, grid_placement.to_owned());
                                if let Some(cell) = timeline.cells.get_mut(*index) {
                                    cell.placements.insert(*id, grid_placement);
                                }
                            }
                        }
                        Placement::Event {
                            id,
                            index,
                            event,
                            stream,
                            ..
                        } => {
                            if let Some(component) = model.events().get(event) {
                                let grid_placement =
                                    event_placement(*id, *index, component.to_owned());
                                placements.insert(*id, grid_placement.to_owned());
                                match stream {
                                    Some(stream_id) => {
                                        if let Some(a) = streams.get_mut(stream_id) {
                                            if let Some(cell) = a.cells.get_mut(*index) {
                                                cell.placements.insert(*id, grid_placement);
                                            }
                                        }
                                    }
                                    None => {
                                        if let Some(cell) = default_stream.cells.get_mut(*index) {
                                            cell.placements.insert(*id, grid_placement);
                                        }
                                    }
                                }
                            }
                        }
                        Placement::ReadModel {
                            id,
                            index,
                            read_model,
                            ..
                        } => {
                            if let Some(component) = model.read_models().get(read_model) {
                                let grid_placement =
                                    read_model_placement(*id, *index, component.to_owned());
                                placements.insert(*id, grid_placement.to_owned());
                                if let Some(cell) = timeline.cells.get_mut(*index) {
                                    cell.placements.insert(*id, grid_placement);
                                }
                            }
                        }
                    }
                }

                // Construct and return grid
                EventModelGrid {
                    state: EventModelGridState::Available,
                    id: model.id(),
                    name: model.name().into(),
                    description: model.description().to_string(),
                    column_count,
                    row_count,
                    default_audience,
                    audiences: audiences
                        .into_values()
                        .sorted_by(|a, b| Ord::cmp(&a.row, &b.row))
                        .collect(),
                    timeline,
                    streams: streams
                        .into_values()
                        .sorted_by(|a, b| Ord::cmp(&a.row, &b.row))
                        .collect(),
                    default_stream,
                    flows: model.flows().values().cloned().map(Into::into).collect(),
                    placements,
                }
            }
        }
    }
}
