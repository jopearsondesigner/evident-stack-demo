use std::collections::HashMap;

use crate::{
    validate_name, Anchor, Audience, Command, CommandId, Component, ComponentId, Described, Entity,
    Event, EventId, EventModel, EventModelData, EventModelError, EventModelId, EventModelState,
    FlowArrow, FlowId, HasSchema, Interface, InterfaceConfig, InterfaceId, Lane, LaneId, LaneIndex,
    ModifiableEventModel, Name, Named, Placement, PlacementId, PlacementIndex, PlacementPosition,
    ReadModel, ReadModelId, Stream,
};
use autosurgeon::{Hydrate, Reconcile, Text};

use uuid::Uuid;

#[derive(Reconcile, Hydrate, Debug)]
pub struct AutoName(String);

impl AutoName {
    fn create(value: &str) -> Result<Self, EventModelError> {
        let name = validate_name(value)?;
        Ok(AutoName(name))
    }
}

impl From<&Name> for AutoName {
    fn from(n: &Name) -> Self {
        AutoName(n.into())
    }
}

impl From<&AutoName> for Name {
    fn from(n: &AutoName) -> Self {
        Name::create(&n.0).unwrap()
    }
}

#[derive(Reconcile, Hydrate, Debug)]
pub enum AutoInterfaceConfig {
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

impl From<&AutoInterfaceConfig> for InterfaceConfig {
    fn from(interface_config: &AutoInterfaceConfig) -> Self {
        todo!()
    }
}

impl From<&InterfaceConfig> for AutoInterfaceConfig {
    fn from(interface_config: &InterfaceConfig) -> Self {
        todo!()
    }
}

#[derive(Reconcile, Hydrate, Debug)]
pub struct AutoInterface {
    #[key]
    id: Uuid,
    name: AutoName,
    description: Text,
    config: AutoInterfaceConfig,
}

impl From<&AutoInterface> for Interface {
    fn from(interface: &AutoInterface) -> Self {
        todo!()
    }
}

impl From<&Interface> for AutoInterface {
    fn from(interface: &Interface) -> Self {
        todo!()
    }
}

#[derive(Reconcile, Hydrate, Debug)]
pub struct AutoCommand {
    #[key]
    id: Uuid,
    name: AutoName,
    description: Text,
    schema: Text,
}

impl From<&AutoCommand> for Command {
    fn from(command: &AutoCommand) -> Self {
        todo!()
    }
}

impl From<&Command> for AutoCommand {
    fn from(command: &Command) -> Self {
        todo!()
    }
}

#[derive(Reconcile, Hydrate, Debug)]
pub struct AutoEvent {
    #[key]
    id: Uuid,
    name: AutoName,
    description: Text,
    schema: Text,
}

impl From<&AutoEvent> for Event {
    fn from(event: &AutoEvent) -> Self {
        todo!()
    }
}

impl From<&Event> for AutoEvent {
    fn from(event: &Event) -> Self {
        todo!()
    }
}

#[derive(Reconcile, Hydrate, Debug)]
pub struct AutoReadModel {
    #[key]
    id: Uuid,
    name: AutoName,
    description: Text,
    schema: Text,
}

impl From<&AutoReadModel> for ReadModel {
    fn from(readModel: &AutoReadModel) -> Self {
        todo!()
    }
}

impl From<&ReadModel> for AutoReadModel {
    fn from(readModel: &ReadModel) -> Self {
        todo!()
    }
}

#[derive(Reconcile, Hydrate, Debug)]
pub struct AutoAudience {
    #[key]
    id: Uuid,
    name: AutoName,
}

impl From<&AutoAudience> for Audience {
    fn from(audience: &AutoAudience) -> Self {
        todo!()
    }
}

impl From<&Audience> for AutoAudience {
    fn from(audience: &Audience) -> Self {
        todo!()
    }
}

#[derive(Reconcile, Hydrate, Debug)]
pub struct AutoStream {
    #[key]
    id: Uuid,
    name: AutoName,
}

impl From<&AutoStream> for Stream {
    fn from(stream: &AutoStream) -> Self {
        todo!()
    }
}

impl From<&Stream> for AutoStream {
    fn from(stream: &Stream) -> Self {
        todo!()
    }
}

#[derive(Reconcile, Hydrate, Debug)]
pub enum AutoPlacement {
    Interface {
        #[key]
        id: Uuid,
        index: u32,
        interface: Uuid,
        audience: Option<Uuid>,
    },
    Command {
        #[key]
        id: Uuid,
        index: u32,
        command: Uuid,
        schema: Text,
    },
    Event {
        #[key]
        id: Uuid,
        index: u32,
        event: Uuid,
        stream: Option<Uuid>,
        schema: Text,
    },
    ReadModel {
        #[key]
        id: Uuid,
        index: u32,
        read_model: Uuid,
        schema: Text,
    },
}

impl AutoPlacement {
    pub fn index(&self) -> PlacementIndex {
        match self {
            AutoPlacement::Interface { index, .. } => *index as usize,
            AutoPlacement::Command { index, .. } => *index as usize,
            AutoPlacement::Event { index, .. } => *index as usize,
            AutoPlacement::ReadModel { index, .. } => *index as usize,
        }
    }

    pub fn shift_right(&mut self, offset: usize) {
        match self {
            AutoPlacement::Interface { index, .. } => *index += offset as u32,
            AutoPlacement::Command { index, .. } => *index += offset as u32,
            AutoPlacement::Event { index, .. } => *index += offset as u32,
            AutoPlacement::ReadModel { index, .. } => *index += offset as u32,
        }
    }

    pub fn relocate(&mut self, idx: PlacementIndex, lane: LaneId) {
        match self {
            AutoPlacement::Interface {
                index, audience, ..
            } => {
                *index = idx as u32;
                match lane {
                    LaneId::DefaultAudience => *audience = None,
                    LaneId::Audience(id) => *audience = Some(id),
                    _ => (),
                };
            }
            AutoPlacement::Command { index, .. } => *index = idx as u32,
            AutoPlacement::Event { index, stream, .. } => {
                *index = idx as u32;
                match lane {
                    LaneId::Stream(id) => *stream = Some(id),
                    LaneId::DefaultStream => *stream = None,
                    _ => (),
                }
            }
            AutoPlacement::ReadModel { index, .. } => *index = idx as u32,
        }
    }
}

impl From<&AutoPlacement> for Placement {
    fn from(placement: &AutoPlacement) -> Self {
        todo!()
    }
}

impl From<&Placement> for AutoPlacement {
    fn from(placement: &Placement) -> Self {
        todo!()
    }
}

impl Entity for AutoPlacement {
    fn id(&self) -> Uuid {
        match self {
            AutoPlacement::Interface { id, .. } => *id,
            AutoPlacement::Command { id, .. } => *id,
            AutoPlacement::Event { id, .. } => *id,
            AutoPlacement::ReadModel { id, .. } => *id,
        }
    }
}

#[derive(Reconcile, Hydrate, Debug)]
pub enum AutoAnchor {
    None,
    Top,
    Left,
    Bottom,
    Right,
}

impl From<&AutoAnchor> for Anchor {
    fn from(flow: &AutoAnchor) -> Self {
        todo!()
    }
}

impl From<&Anchor> for AutoAnchor {
    fn from(flow: &Anchor) -> Self {
        todo!()
    }
}

#[derive(Reconcile, Hydrate, Debug)]
pub struct AutoFlowArrow {
    #[key]
    id: Uuid,
    from_placement: Uuid,
    from_anchor: AutoAnchor,
    to_placement: Uuid,
    to_anchor: AutoAnchor,
}

impl From<&AutoFlowArrow> for FlowArrow {
    fn from(flow: &AutoFlowArrow) -> Self {
        todo!()
    }
}

impl From<&FlowArrow> for AutoFlowArrow {
    fn from(flow: &FlowArrow) -> Self {
        todo!()
    }
}

//  Event Model Implementation

#[derive(Reconcile, Hydrate, Debug)]
pub struct AutomergeEventModel {
    #[key]
    id: Uuid,
    name: AutoName,
    description: Text,
    schema: Text,
    interfaces: HashMap<String, AutoInterface>,
    commands: HashMap<String, AutoCommand>,
    events: HashMap<String, AutoEvent>,
    read_models: HashMap<String, AutoReadModel>,
    audiences: Vec<AutoAudience>,
    streams: Vec<AutoStream>,
    placements: HashMap<String, AutoPlacement>,
    flows: HashMap<String, AutoFlowArrow>,
}

pub enum AutoComponentMut<'a> {
    Interface(&'a mut AutoInterface),
    Command(&'a mut AutoCommand),
    Event(&'a mut AutoEvent),
    ReadModel(&'a mut AutoReadModel),
}

impl AutomergeEventModel {
    pub fn new(id: &Uuid, name: &Name) -> Self {
        AutomergeEventModel {
            id: *id,
            name: name.into(),
            description: Default::default(),
            schema: Default::default(),
            interfaces: Default::default(),
            commands: Default::default(),
            events: Default::default(),
            read_models: Default::default(),
            audiences: Default::default(),
            streams: Default::default(),
            placements: Default::default(),
            flows: Default::default(),
        }
    }

    fn component_mut_by_id(&mut self, id: &ComponentId) -> Option<AutoComponentMut> {
        match id {
            ComponentId::InterfaceComponentId(id) => self
                .interfaces
                .get_mut(&id.to_string())
                .map(AutoComponentMut::Interface),
            ComponentId::CommandComponentId(id) => self
                .commands
                .get_mut(&id.to_string())
                .map(AutoComponentMut::Command),
            ComponentId::EventComponentId(id) => self
                .events
                .get_mut(&id.to_string())
                .map(AutoComponentMut::Event),
            ComponentId::ReadModelComponentId(id) => self
                .read_models
                .get_mut(&id.to_string())
                .map(AutoComponentMut::ReadModel),
        }
    }
}

impl Entity for AutomergeEventModel {
    fn id(&self) -> Uuid {
        self.id.to_owned()
    }
}

impl Named for AutomergeEventModel {
    fn name(&self) -> Name {
        let n = &self.name;
        n.into()
    }
}

impl Described for AutomergeEventModel {
    fn description(&self) -> &str {
        self.description.as_str()
    }
}

impl HasSchema for AutomergeEventModel {
    fn schema(&self) -> &str {
        self.schema.as_str()
    }
}

impl EventModel for AutomergeEventModel {
    fn create(initial: &EventModelState<Self>, id: &EventModelId, name: &Name) -> Self {
        match initial {
            EventModelState::BeforeCreation => AutomergeEventModel::new(id, name.into()),
            _ => panic!("Illegal state when creating Automerge Event Model!"),
        }
    }
}

impl EventModelData for AutomergeEventModel {
    fn interfaces(&self) -> HashMap<InterfaceId, Interface> {
        self.interfaces.values().map(|i| (i.id, i.into())).collect()
    }

    fn commands(&self) -> HashMap<CommandId, Command> {
        self.commands.values().map(|c| (c.id, c.into())).collect()
    }

    fn events(&self) -> HashMap<EventId, Event> {
        self.events.values().map(|e| (e.id, e.into())).collect()
    }

    fn read_models(&self) -> HashMap<ReadModelId, ReadModel> {
        self.read_models
            .values()
            .map(|r| (r.id, r.into()))
            .collect()
    }

    fn audiences(&self) -> Vec<Audience> {
        self.audiences.iter().map(|x| x.into()).collect()
    }

    fn streams(&self) -> Vec<Stream> {
        self.streams.iter().map(|x| x.into()).collect()
    }

    fn placements(&self) -> HashMap<PlacementId, Placement> {
        self.placements
            .values()
            .map(|p| (p.id(), p.into()))
            .collect()
    }

    fn flows(&self) -> HashMap<FlowId, FlowArrow> {
        self.flows.values().map(|f| (f.id, f.into())).collect()
    }

    fn get_placement(&self, id: &PlacementId) -> Option<Placement> {
        self.placements.get(&id.to_string()).map(|p| p.into())
    }
}

// impl Renamable for AutomergeEventModel {
//     fn rename(&mut self, name: &str) {
//         self.name = name.to_string();
//     }
// }

// impl ModifiablyDescribed for AutomergeEventModel {
//     fn set_description(&mut self, description: &str) {
//         self.description = Text::with_value(description);
//     }

//     fn add_to_description(&mut self, index: usize, addition: &str) {
//         self.description.splice(index, 0, addition);
//     }

//     fn delete_from_description(&mut self, index: usize, count: usize) {
//         self.description.splice(index, count, EMPTY_STR);
//     }
// }

// impl HasModifiableSchema for AutomergeEventModel {
//     fn set_schema(&mut self, schema: &str) {
//         self.schema = Text::with_value(schema);
//     }

//     fn add_to_schema(&mut self, index: usize, addition: &str) {
//         self.schema.splice(index, 0, addition);
//     }

//     fn delete_from_schema(&mut self, index: usize, count: usize) {
//         self.schema.splice(index, count, EMPTY_STR);
//     }
// }

impl ModifiableEventModel for AutomergeEventModel {
    fn rename(&mut self, name: &Name) {
        self.name = name.into();
    }

    fn splice_description(&mut self, index: usize, del: usize, add: &str) {
        self.description.splice(index, del, add);
    }

    fn splice_schema(&mut self, index: usize, del: usize, add: &str) {
        self.schema.splice(index, del, add);
    }

    fn component_defined(&mut self, component: &Component) {
        match component {
            Component::Interface(i) => {
                self.interfaces.insert(i.id().to_string(), i.into());
            }
            Component::Command(c) => {
                self.commands.insert(c.id().to_string(), c.into());
            }
            Component::Event(e) => {
                self.events.insert(e.id().to_string(), e.into());
            }
            Component::ReadModel(r) => {
                self.read_models.insert(r.id().to_string(), r.into());
            }
        }
    }

    fn component_renamed(&mut self, component_id: &ComponentId, name: &Name) {
        match self.component_mut_by_id(component_id) {
            None => {
                panic!("Component with id {:?} not found", component_id)
            }
            Some(AutoComponentMut::Interface(i)) => i.name = name.into(),
            Some(AutoComponentMut::Command(c)) => c.name = name.into(),
            Some(AutoComponentMut::Event(e)) => e.name = name.into(),
            Some(AutoComponentMut::ReadModel(r)) => r.name = name.into(),
        }
    }

    fn component_removed(&mut self, component_id: &ComponentId) {
        match component_id {
            ComponentId::InterfaceComponentId(id) => {
                self.interfaces.remove(&id.to_string());
            }
            ComponentId::CommandComponentId(id) => {
                self.commands.remove(&id.to_string());
            }
            ComponentId::EventComponentId(id) => {
                self.events.remove(&id.to_string());
            }
            ComponentId::ReadModelComponentId(id) => {
                self.read_models.remove(&id.to_string());
            }
        }
    }

    fn splice_component_description(
        &mut self,
        component_id: &ComponentId,
        index: usize,
        del: usize,
        addition: &str,
    ) {
        match self.component_mut_by_id(component_id) {
            Some(AutoComponentMut::Interface(i)) => {
                i.description.splice(index, del, addition);
            }
            Some(AutoComponentMut::Command(c)) => {
                c.description.splice(index, del, addition);
            }
            Some(AutoComponentMut::Event(e)) => {
                e.description.splice(index, del, addition);
            }
            Some(AutoComponentMut::ReadModel(r)) => {
                r.description.splice(index, del, addition);
            }
            None => {
                panic!("Component with id {:?} not found", component_id)
            }
        }
    }

    fn splice_component_schema(
        &mut self,
        component_id: &ComponentId,
        index: usize,
        del: usize,
        addition: &str,
    ) {
        match self.component_mut_by_id(component_id) {
            Some(AutoComponentMut::Interface(_)) => (),
            Some(AutoComponentMut::Command(c)) => {
                c.schema.splice(index, del, addition);
            }
            Some(AutoComponentMut::Event(e)) => {
                e.schema.splice(index, del, addition);
            }
            Some(AutoComponentMut::ReadModel(r)) => {
                r.schema.splice(index, del, addition);
            }
            None => {
                panic!("Component with id {:?} not found", component_id)
            }
        }
    }

    fn component_placed(&mut self, placement: &Placement) {
        self.placements
            .insert(placement.id().to_string(), placement.into());
    }

    fn placement_moved(&mut self, position: &PlacementPosition) {
        if let Some(ref mut placement) = self.placements.get_mut(&position.id().to_string()) {
            let PlacementPosition(_, index, lane) = position;
            placement.relocate(index.to_owned(), lane.to_owned());
        };
    }

    fn placement_removed(&mut self, placement_id: &PlacementId) {
        self.placements.remove(&placement_id.to_string());
    }

    fn placements_shifted(&mut self, offset: usize, width: usize) {
        self.placements.iter_mut().for_each(|(_, placement)| {
            if placement.index() >= offset {
                placement.shift_right(width);
            }
        })
    }

    fn splice_placement_schema(
        &mut self,
        placement_id: &PlacementId,
        index: usize,
        del: usize,
        addition: &str,
    ) {
        match self.placements.get_mut(&placement_id.to_string()) {
            Some(AutoPlacement::Interface { .. }) => (),
            Some(AutoPlacement::Command { schema, .. }) => {
                schema.splice(index, del, addition);
            }
            Some(AutoPlacement::Event { schema, .. }) => {
                schema.splice(index, del, addition);
            }
            Some(AutoPlacement::ReadModel { schema, .. }) => {
                schema.splice(index, del, addition);
            }
            None => {
                panic!("Placement with id {:?} not found", placement_id)
            }
        }
    }

    fn lane_added(&mut self, lane: &Lane, index: LaneIndex) {
        match lane {
            Lane::Audience(audience) => self.audiences.insert(index, audience.into()),
            Lane::Stream(stream) => self.streams.insert(index, stream.into()),
        }
    }

    fn lane_renamed(&mut self, lane_id: &LaneId, name: &Name) {
        match lane_id {
            LaneId::Audience(id) => {
                if let Some(a) = self
                    .audiences
                    .iter_mut()
                    .find(|audience| *id == audience.id)
                {
                    a.name = name.into();
                }
            }
            LaneId::Stream(id) => {
                if let Some(s) = self.streams.iter_mut().find(|stream| *id == stream.id) {
                    s.name = name.into();
                }
            }
            _ => (),
        }
    }

    fn lane_reordered(&mut self, lane_id: &LaneId, index: LaneIndex) {
        match lane_id {
            LaneId::Audience(id) => {
                if let Some(idx) = self
                    .audiences
                    .iter_mut()
                    .position(|audience| *id == audience.id)
                {
                    let audience = self.audiences.remove(idx);
                    self.audiences.insert(index, audience);
                }
            }
            LaneId::Stream(id) => {
                if let Some(idx) = self.streams.iter_mut().position(|stream| *id == stream.id) {
                    let stream = self.streams.remove(idx);
                    self.streams.insert(index, stream);
                }
            }
            _ => (),
        }
    }

    fn lane_removed(&mut self, lane_id: &LaneId) {
        match lane_id {
            LaneId::Audience(id) => {
                if let Some(idx) = self
                    .audiences
                    .iter_mut()
                    .position(|audience| *id == audience.id)
                {
                    self.audiences.remove(idx);
                }
            }
            LaneId::Stream(id) => {
                if let Some(idx) = self.streams.iter_mut().position(|stream| *id == stream.id) {
                    self.streams.remove(idx);
                }
            }
            _ => (),
        }
    }

    fn plus_flow(&mut self, flow_arrow: &FlowArrow) {
        self.flows
            .insert(flow_arrow.id().to_string(), flow_arrow.into());
    }

    fn minus_flow(&mut self, flow_id: &FlowId) {
        self.flows.remove(&flow_id.to_string());
    }
}
