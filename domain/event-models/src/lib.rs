extern crate core;
extern crate serde;
extern crate url;
extern crate uuid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use url::Url;
use uuid::Uuid;

pub mod api;
pub mod implementation;
pub mod json;

pub type EventModelId = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventModelState<T: EventModel> {
    BeforeCreation,
    EventModel(T),
    Deleted(EventModelId),
}

pub trait EventModel: Described + EventModelData + Sized {
    fn create(initial: &EventModelState<Self>, id: &EventModelId, name: &Name) -> Self;
}

pub trait EventModelData: HasSchema + Debug {
    fn interfaces(&self) -> HashMap<InterfaceId, Interface>;
    fn commands(&self) -> HashMap<CommandId, Command>;
    fn events(&self) -> HashMap<EventId, Event>;
    fn read_models(&self) -> HashMap<ReadModelId, ReadModel>;
    fn audiences(&self) -> Vec<Audience>;
    fn streams(&self) -> Vec<Stream>;
    fn placements(&self) -> HashMap<PlacementId, Placement>;
    fn flows(&self) -> HashMap<FlowId, FlowArrow>;

    fn get_placement(&self, id: &PlacementId) -> Option<Placement>;
}

pub trait ModifiableEventModel: EventModel {
    // ***** Metadata *****
    fn rename(&mut self, name: &Name);
    fn splice_description(&mut self, index: usize, del: usize, add: &str);
    fn splice_schema(&mut self, index: usize, del: usize, add: &str);

    // ***** Components *****
    fn component_defined(&mut self, component: &Component);

    // Validation of presence of component_id must be performed
    //  by `decide` prior to this step
    fn component_renamed(&mut self, component_id: &ComponentId, name: &Name);

    // Validation of presence of component_id must be performed
    //  by `decide` prior to this step
    fn component_removed(&mut self, component_id: &ComponentId);

    // Validation of presence of component_id must be performed
    //  by `decide` prior to this step
    fn splice_component_description(
        &mut self,
        component_id: &ComponentId,
        index: usize,
        del: usize,
        addition: &str,
    );

    // Validation of presence of component_id must be performed
    //  by `decide` prior to this step
    fn splice_component_schema(
        &mut self,
        component_id: &ComponentId,
        index: usize,
        del: usize,
        addition: &str,
    );

    // ***** Placements *****

    fn component_placed(&mut self, placement: &Placement);

    // Validation of presence of placement given by position must be performed
    //  by `decide` prior to this step
    fn placement_moved(&mut self, position: &PlacementPosition);

    // Validation of presence of placement_id must be performed
    //  by `decide` prior to this step
    fn placement_removed(&mut self, placement_id: &PlacementId);

    fn placements_shifted(&mut self, offset: usize, width: usize);

    // Validation of presence of placement_id must be performed
    //  by `decide` prior to this step
    fn splice_placement_schema(
        &mut self,
        placement_id: &PlacementId,
        index: usize,
        del: usize,
        addition: &str,
    );

    // ***** Lanes *****

    fn lane_added(&mut self, lane: &Lane, index: LaneIndex);
    fn lane_renamed(&mut self, lane_id: &LaneId, name: &Name);
    fn lane_reordered(&mut self, lane_id: &LaneId, index: LaneIndex);
    fn lane_removed(&mut self, lane_id: &LaneId);

    // ***** Flows *****
    fn plus_flow(&mut self, flow_arrow: &FlowArrow);
    fn minus_flow_by_placement_ids(&mut self, from: &PlacementId, to: &PlacementId) {
        self.minus_flow(&flow_id(from, to));
    }
    fn minus_flow(&mut self, flow_id: &FlowId);
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldError {
    FlowAnchorString(Option<String>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventModelError {
    IllegalState(String),
    InvalidNameError(String),
    CreationError(String),
    ModificationError(String),
    IllegalFlowArrow(String),
    SerializationError(String),
    LaneNotFound(LaneId),
    LaneIndexOutOfBounds(LaneId, usize),
    DescriptionTextOutOfBounds(String, usize, usize),
    FieldError(FieldError),
}

//// ***** Types *****

pub trait Entity {
    fn id(&self) -> Uuid;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Name(String);

pub(crate) fn validate_name(name: &str) -> Result<String, EventModelError> {
    if !name.is_empty() {
        Ok(name.to_string())
    } else {
        Err(EventModelError::InvalidNameError(
            "Name cannot be empty".to_string(),
        ))
    }
}

impl Name {
    pub fn create(value: &str) -> Result<Self, EventModelError> {
        let valid_name = validate_name(value)?;
        Ok(Name(valid_name))
    }
}

impl TryFrom<&str> for Name {
    type Error = EventModelError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Name::create(value)
    }
}

impl From<Name> for String {
    fn from(val: Name) -> Self {
        val.0
    }
}

impl From<&Name> for String {
    fn from(val: &Name) -> Self {
        val.0.to_owned()
    }
}

impl<'a> From<&'a Name> for &'a String {
    fn from(val: &'a Name) -> Self {
        &val.0
    }
}

pub trait Named: Entity {
    fn name(&self) -> Name;
}

pub trait Described: Named {
    fn description(&self) -> &str;
}

pub trait HasSchema {
    fn schema(&self) -> &str;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextEdit {
    position: usize,
    deletions: usize,
    addition: String,
}

pub type LaneIndex = usize;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum LaneId {
    DefaultAudience,
    Audience(AudienceId),
    Timeline,
    Stream(StreamId),
    DefaultStream,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Lane {
    Audience(Audience),
    Stream(Stream),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentId {
    InterfaceComponentId(InterfaceId),
    CommandComponentId(CommandId),
    EventComponentId(EventId),
    ReadModelComponentId(ReadModelId),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Component {
    Interface(Interface),
    Command(Command),
    Event(Event),
    ReadModel(ReadModel),
}

#[derive(Debug, Clone)]
pub(crate) struct EventModelDataTransfer {
    pub(crate) schema: String,
    pub(crate) interfaces: HashMap<InterfaceId, Interface>,
    pub(crate) commands: HashMap<CommandId, Command>,
    pub(crate) events: HashMap<EventId, Event>,
    pub(crate) read_models: HashMap<ReadModelId, ReadModel>,
    pub(crate) audiences: Vec<Audience>,
    pub(crate) streams: Vec<Stream>,
    pub(crate) placements: HashMap<PlacementId, Placement>,
    pub(crate) flows: HashMap<FlowId, FlowArrow>,
}

pub type InterfaceId = Uuid;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterfaceConfig {
    #[default]
    Blank,
    Figma {
        url: Url,
        width: Option<usize>,
        height: Option<usize>,
    },
    Image {
        url: Url,
        width: Option<usize>,
        height: Option<usize>,
    },
    Job,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Interface {
    id: InterfaceId,
    name: Name,
    description: String,
    config: InterfaceConfig,
}

impl Interface {
    pub fn create(
        id: Uuid,
        name: String,
        description: String,
        config: InterfaceConfig,
    ) -> Result<Self, EventModelError> {
        let name = Name::create(&name)?;
        Ok(Interface {
            id,
            name,
            description,
            config,
        })
    }

    pub fn config(&self) -> InterfaceConfig {
        self.config.to_owned()
    }
}

impl Entity for Interface {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl Named for Interface {
    fn name(&self) -> Name {
        self.name.to_owned()
    }
}

impl Described for Interface {
    fn description(&self) -> &str {
        &self.description
    }
}

pub type CommandId = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Command {
    id: CommandId,
    name: Name,
    description: String,
    schema: String,
}

impl Command {
    pub fn create(
        id: Uuid,
        name: String,
        description: String,
        schema: String,
    ) -> Result<Self, EventModelError> {
        let name = Name::create(&name)?;
        Ok(Command {
            id,
            name,
            description,
            schema,
        })
    }
}

impl Entity for Command {
    fn id(&self) -> Uuid {
        self.id.to_owned()
    }
}

impl Named for Command {
    fn name(&self) -> Name {
        self.name.to_owned()
    }
}

impl Described for Command {
    fn description(&self) -> &str {
        &self.description
    }
}

impl HasSchema for Command {
    fn schema(&self) -> &str {
        &self.schema
    }
}

pub type EventId = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    id: EventId,
    name: Name,
    description: String,
    schema: String,
}

impl Event {
    pub fn create(
        id: Uuid,
        name: String,
        description: String,
        schema: String,
    ) -> Result<Self, EventModelError> {
        let name = Name::create(&name)?;
        Ok(Event {
            id,
            name,
            description,
            schema,
        })
    }
}

impl Entity for Event {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl Named for Event {
    fn name(&self) -> Name {
        self.name.to_owned()
    }
}

impl Described for Event {
    fn description(&self) -> &str {
        &self.description
    }
}

impl HasSchema for Event {
    fn schema(&self) -> &str {
        &self.schema
    }
}

pub type ReadModelId = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadModel {
    id: ReadModelId,
    name: Name,
    description: String,
    schema: String,
}

impl ReadModel {
    pub fn create(
        id: Uuid,
        name: String,
        description: String,
        schema: String,
    ) -> Result<Self, EventModelError> {
        let name = Name::create(&name)?;
        Ok(ReadModel {
            id,
            name,
            description,
            schema,
        })
    }
}

impl Entity for ReadModel {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl Named for ReadModel {
    fn name(&self) -> Name {
        self.name.to_owned()
    }
}

impl Described for ReadModel {
    fn description(&self) -> &str {
        &self.description
    }
}

impl HasSchema for ReadModel {
    fn schema(&self) -> &str {
        &self.schema
    }
}

pub type AudienceId = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Audience {
    id: AudienceId,
    name: Name,
}

impl Audience {
    pub fn create(id: AudienceId, name: String) -> Result<Self, EventModelError> {
        let name = Name::create(&name)?;
        Ok(Audience { id, name })
    }
}

impl Entity for Audience {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl Named for Audience {
    fn name(&self) -> Name {
        self.name.to_owned()
    }
}

pub type StreamId = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stream {
    id: StreamId,
    name: Name,
}

impl Stream {
    pub fn create(id: StreamId, name: String) -> Result<Self, EventModelError> {
        let name = Name::create(&name)?;
        Ok(Stream { id, name })
    }
}

impl Entity for Stream {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl Named for Stream {
    fn name(&self) -> Name {
        self.name.to_owned()
    }
}

pub type PlacementIndex = usize;
pub type PlacementId = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlacementPosition(pub PlacementId, pub PlacementIndex, pub LaneId);

impl Entity for PlacementPosition {
    fn id(&self) -> Uuid {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Placement {
    Interface {
        id: PlacementId,
        index: PlacementIndex,
        interface: InterfaceId,
        audience: Option<AudienceId>,
    },
    Command {
        id: PlacementId,
        index: PlacementIndex,
        command: CommandId,
        schema: String,
    },
    Event {
        id: PlacementId,
        index: PlacementIndex,
        event: EventId,
        stream: Option<StreamId>,
        schema: String,
    },
    ReadModel {
        id: PlacementId,
        index: PlacementIndex,
        read_model: ReadModelId,
        schema: String,
    },
}

pub enum PlacementKind {
    Interface,
    Command,
    Event,
    ReadModel,
}

impl Placement {
    pub fn index(&self) -> &PlacementIndex {
        match self {
            Placement::Interface { index, .. } => index,
            Placement::Command { index, .. } => index,
            Placement::Event { index, .. } => index,
            Placement::ReadModel { index, .. } => index,
        }
    }

    pub fn lane(&self) -> LaneId {
        match self {
            Placement::Interface { audience, .. } => match audience {
                Some(id) => LaneId::Audience(*id),
                None => LaneId::DefaultAudience,
            },
            Placement::Command { .. } => LaneId::Timeline,
            Placement::Event { stream, .. } => match stream {
                Some(id) => LaneId::Stream(*id),
                None => LaneId::DefaultStream,
            },
            Placement::ReadModel { .. } => LaneId::Timeline,
        }
    }

    pub fn component_id(&self) -> ComponentId {
        match self {
            Placement::Interface { interface, .. } => {
                ComponentId::InterfaceComponentId(interface.to_owned())
            }
            Placement::Command { command, .. } => {
                ComponentId::CommandComponentId(command.to_owned())
            }
            Placement::Event { event, .. } => ComponentId::EventComponentId(event.to_owned()),
            Placement::ReadModel { read_model, .. } => {
                ComponentId::ReadModelComponentId(read_model.to_owned())
            }
        }
    }
}

impl Entity for Placement {
    fn id(&self) -> Uuid {
        match self {
            Placement::Interface { id, .. } => *id,
            Placement::Command { id, .. } => *id,
            Placement::Event { id, .. } => *id,
            Placement::ReadModel { id, .. } => *id,
        }
    }
}

pub type FlowId = Uuid;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Anchor {
    #[default]
    None,
    Top,
    Left,
    Bottom,
    Right,
}

impl TryFrom<Option<String>> for Anchor {
    type Error = FieldError;

    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        match value.clone() {
            Some(str) => match str.as_str() {
                "None" => Ok(Anchor::None),
                "Top" => Ok(Anchor::Top),
                "Bottom" => Ok(Anchor::Bottom),
                "Left" => Ok(Anchor::Left),
                "Right" => Ok(Anchor::Right),
                _ => Err(FieldError::FlowAnchorString(value)),
            },
            None => Ok(Anchor::default()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Port {
    placement: PlacementId,
    anchor: Anchor, // TODO: InterfaceElement?
}

impl Port {
    pub fn placement_id(&self) -> &PlacementId {
        &self.placement
    }

    pub fn anchor(&self) -> &Anchor {
        &self.anchor
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlowArrow {
    id: FlowId,
    from: Port,
    to: Port,
}

pub fn flow_id(from: &PlacementId, to: &PlacementId) -> FlowId {
    Uuid::new_v5(from, to.as_bytes())
}

impl FlowArrow {
    pub fn create(
        from: PlacementId,
        from_anchor: Anchor,
        to: PlacementId,
        to_anchor: Anchor,
    ) -> Result<FlowArrow, EventModelError> {
        Ok(FlowArrow {
            id: flow_id(&from, &to),
            from: Port {
                placement: from,
                anchor: from_anchor,
            },
            to: Port {
                placement: to,
                anchor: to_anchor,
            },
        })
    }

    // TODO: enforce business rules!
    pub fn connect(
        model: &impl EventModel,
        from: Port,
        to: Port,
    ) -> Result<FlowArrow, EventModelError> {
        let from_placement = model.placements().get(&from.placement);
        let to_placement = model.placements().get(&to.placement);
        todo!("Validation of flow arrow");
        Ok(FlowArrow {
            id: flow_id(&from.placement, &to.placement),
            from,
            to,
        })
    }

    pub fn to(&self) -> &Port {
        &self.to
    }

    pub fn from(&self) -> &Port {
        &self.from
    }

    // fun build(
    //             model: EventModel,
    //             from: FlowPort,
    //             to: FlowPort
    //         ): FlowArrow {
    //             val fromPlacement = model.placements[from.placementId]
    //                 ?: throw IllegalArgumentException("From placement doesn't exist")
    //             val toPlacement = model.placements[to.placementId]
    //                 ?: throw IllegalArgumentException("To placement doesn't exist")
    //             if (toPlacement.index < fromPlacement.index)
    //                 throw IllegalArgumentException("Flows cannot go backward")
    //             if (model.getFlow(from.placementId, to.placementId) != null)
    //                 throw IllegalArgumentException("Flow already connects these two placements")
    //             return when (fromPlacement) {
    //                 is InterfacePlacement -> when (toPlacement) {
    //                     is CommandPlacement -> FlowArrow(from, to)
    //                     else -> throw IllegalArgumentException("Interfaces can only flow to Commands")
    //                 }
    //
    //                 is CommandPlacement -> when (toPlacement) {
    //                     is EventPlacement -> FlowArrow(from, to)
    //                     else -> throw IllegalArgumentException("Commands can only flow to Events")
    //                 }
    //
    //                 is EventPlacement -> when (toPlacement) {
    //                     is CommandPlacement, is ReadModelPlacement -> FlowArrow(from, to)
    //                     else -> throw IllegalArgumentException("Events can only flow to Commands or Read Models")
    //                 }
    //
    //                 is ReadModelPlacement -> when (toPlacement) {
    //                     is InterfacePlacement -> FlowArrow(from, to)
    //                     else -> throw IllegalArgumentException("Read Models can only flow to Interfaces")
    //                 }
    //             }
    //         }
}

impl Entity for FlowArrow {
    fn id(&self) -> FlowId {
        self.id
    }
}
