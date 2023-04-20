use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    Audience, Command, CommandId, Component, ComponentId, Described, Entity, Event, EventId,
    EventModel, EventModelData, EventModelId, EventModelState, FlowArrow, FlowId, HasSchema,
    Interface, InterfaceId, Lane, LaneId, LaneIndex, ModifiableEventModel, Name, Named, Placement,
    PlacementId, PlacementIndex, PlacementPosition, ReadModel, ReadModelId, Stream,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InMemoryEventModel {
    id: EventModelId,
    name: Name,
    description: String,
    schema: String,
    interfaces: HashMap<InterfaceId, Interface>,
    commands: HashMap<CommandId, Command>,
    events: HashMap<EventId, Event>,
    read_models: HashMap<ReadModelId, ReadModel>,
    audiences: Vec<Audience>,
    streams: Vec<Stream>,
    placements: HashMap<PlacementId, Placement>,
    flows: HashMap<FlowId, FlowArrow>,
}

#[derive(Debug)]
pub enum ComponentMut<'a> {
    Interface(&'a mut Interface),
    Command(&'a mut Command),
    Event(&'a mut Event),
    ReadModel(&'a mut ReadModel),
}

impl Placement {
    pub fn shift_right(&mut self, offset: usize) {
        match self {
            Placement::Interface { index, .. } => *index += offset,
            Placement::Command { index, .. } => *index += offset,
            Placement::Event { index, .. } => *index += offset,
            Placement::ReadModel { index, .. } => *index += offset,
        }
    }

    pub fn relocate(&mut self, idx: PlacementIndex, lane: LaneId) {
        match self {
            Placement::Interface {
                index, audience, ..
            } => {
                *index = idx;
                match lane {
                    LaneId::DefaultAudience => *audience = None,
                    LaneId::Audience(id) => *audience = Some(id),
                    _ => (),
                };
            }
            Placement::Command { index, .. } => *index = idx,
            Placement::Event { index, stream, .. } => {
                *index = idx;
                match lane {
                    LaneId::Stream(id) => *stream = Some(id),
                    LaneId::DefaultStream => *stream = None,
                    _ => (),
                }
            }
            Placement::ReadModel { index, .. } => *index = idx,
        }
    }
}

impl InMemoryEventModel {
    pub fn new(id: &EventModelId, name: &Name) -> InMemoryEventModel {
        InMemoryEventModel {
            id: *id,
            name: name.to_owned(),
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

    fn component_mut_by_id(&mut self, id: &ComponentId) -> Option<ComponentMut> {
        match id {
            ComponentId::InterfaceComponentId(id) => {
                self.interfaces.get_mut(id).map(ComponentMut::Interface)
            }
            ComponentId::CommandComponentId(id) => {
                self.commands.get_mut(id).map(ComponentMut::Command)
            }
            ComponentId::EventComponentId(id) => self.events.get_mut(id).map(ComponentMut::Event),
            ComponentId::ReadModelComponentId(id) => {
                self.read_models.get_mut(id).map(ComponentMut::ReadModel)
            }
        }
    }
}

impl Default for InMemoryEventModel {
    fn default() -> Self {
        Self::new(&Uuid::new_v4(), &Name::create("Default").unwrap())
    }
}

impl Entity for InMemoryEventModel {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl Named for InMemoryEventModel {
    fn name(&self) -> Name {
        self.name.to_owned()
    }
}

impl Described for InMemoryEventModel {
    fn description(&self) -> &str {
        &self.description
    }
}

impl HasSchema for InMemoryEventModel {
    fn schema(&self) -> &str {
        &self.schema
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InMemoryCreationDetails;

impl EventModel for InMemoryEventModel {
    type CreationDetails = InMemoryCreationDetails;

    fn create(initial: &EventModelState<Self>, id: &EventModelId, name: &Name) -> Self {
        match initial {
            EventModelState::BeforeCreation(_) => InMemoryEventModel::new(id, name),
            _ => panic!("Illegal state when creating Event Model!"),
        }
    }
}

impl EventModelData for InMemoryEventModel {
    fn interfaces(&self) -> HashMap<InterfaceId, Interface> {
        self.interfaces.to_owned()
    }

    fn commands(&self) -> HashMap<CommandId, Command> {
        self.commands.to_owned()
    }

    fn events(&self) -> HashMap<EventId, Event> {
        self.events.to_owned()
    }

    fn read_models(&self) -> HashMap<ReadModelId, ReadModel> {
        self.read_models.to_owned()
    }

    fn audiences(&self) -> Vec<Audience> {
        self.audiences.to_owned()
    }

    fn streams(&self) -> Vec<Stream> {
        self.streams.to_owned()
    }

    fn placements(&self) -> HashMap<PlacementId, Placement> {
        self.placements.to_owned()
    }

    fn flows(&self) -> HashMap<FlowId, FlowArrow> {
        self.flows.to_owned()
    }

    fn get_placement(&self, id: &PlacementId) -> Option<Placement> {
        self.placements.get(id).cloned()
    }
}

fn splice_string(s: &mut String, pos: usize, del: usize, add: &str) {
    (0..del).for_each(|_i| {
        s.remove(pos);
    });
    s.insert_str(pos, add);
}

impl ModifiableEventModel for InMemoryEventModel {
    fn rename(&mut self, name: &Name) {
        self.name = name.to_owned();
    }

    fn splice_description(&mut self, index: usize, del: usize, add: &str) {
        let s = &mut self.description;
        splice_string(s, index, del, add);
    }

    fn splice_schema(&mut self, index: usize, del: usize, add: &str) {
        let s = &mut self.schema;
        splice_string(s, index, del, add);
    }

    fn component_defined(&mut self, component: &Component) {
        match component {
            Component::Interface(i) => {
                self.interfaces.insert(i.id(), i.to_owned());
            }
            Component::Command(c) => {
                self.commands.insert(c.id(), c.to_owned());
            }
            Component::Event(e) => {
                self.events.insert(e.id(), e.to_owned());
            }
            Component::ReadModel(r) => {
                self.read_models.insert(r.id(), r.to_owned());
            }
        }
    }

    fn component_renamed(&mut self, component_id: &ComponentId, name: &Name) {
        match self.component_mut_by_id(component_id) {
            None => {
                panic!("Component with id {:?} not found", component_id)
            }
            Some(ComponentMut::Interface(i)) => i.name = name.to_owned(),
            Some(ComponentMut::Command(c)) => c.name = name.to_owned(),
            Some(ComponentMut::Event(e)) => e.name = name.to_owned(),
            Some(ComponentMut::ReadModel(r)) => r.name = name.to_owned(),
        }
    }

    fn component_removed(&mut self, component_id: &ComponentId) {
        match component_id {
            ComponentId::InterfaceComponentId(id) => {
                self.interfaces.remove(id);
            }
            ComponentId::CommandComponentId(id) => {
                self.commands.remove(id);
            }
            ComponentId::EventComponentId(id) => {
                self.events.remove(id);
            }
            ComponentId::ReadModelComponentId(id) => {
                self.read_models.remove(id);
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
            Some(ComponentMut::Interface(i)) => {
                let s = &mut i.description;
                splice_string(s, index, del, addition);
            }
            Some(ComponentMut::Command(c)) => {
                let s = &mut c.description;
                splice_string(s, index, del, addition);
            }
            Some(ComponentMut::Event(e)) => {
                let s = &mut e.description;
                splice_string(s, index, del, addition);
            }
            Some(ComponentMut::ReadModel(r)) => {
                let s = &mut r.description;
                splice_string(s, index, del, addition);
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
            Some(ComponentMut::Interface(_)) => (),
            Some(ComponentMut::Command(c)) => {
                let s = &mut c.schema;
                splice_string(s, index, del, addition);
            }
            Some(ComponentMut::Event(e)) => {
                let s = &mut e.schema;
                splice_string(s, index, del, addition);
            }
            Some(ComponentMut::ReadModel(r)) => {
                let s = &mut r.schema;
                splice_string(s, index, del, addition);
            }
            None => {
                panic!("Component with id {:?} not found", component_id)
            }
        }
    }

    fn component_placed(&mut self, placement: &Placement) {
        self.placements
            .insert(placement.id().to_owned(), placement.to_owned());
    }

    fn placement_moved(&mut self, position: &PlacementPosition) {
        if let Some(ref mut placement) = self.placements.get_mut(&position.id()) {
            let PlacementPosition(_, index, lane) = position;
            placement.relocate(index.to_owned(), lane.to_owned());
        };
    }

    fn placement_removed(&mut self, placement_id: &PlacementId) {
        self.placements.remove(placement_id);
    }

    fn placements_shifted(&mut self, offset: usize, width: usize) {
        self.placements.iter_mut().for_each(|(_, placement)| {
            if placement.index() >= &offset {
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
        match self.placements.get_mut(placement_id) {
            Some(Placement::Interface { .. }) => (),
            Some(Placement::Command { schema, .. }) => {
                splice_string(schema, index, del, addition);
            }
            Some(Placement::Event { schema, .. }) => {
                splice_string(schema, index, del, addition);
            }
            Some(Placement::ReadModel { schema, .. }) => {
                splice_string(schema, index, del, addition);
            }
            None => {
                panic!("Placement with id {:?} not found", placement_id)
            }
        }
    }

    fn lane_added(&mut self, lane: &Lane, index: LaneIndex) {
        match lane {
            Lane::Audience(audience) => self.audiences.insert(index, audience.to_owned()),
            Lane::Stream(stream) => self.streams.insert(index, stream.to_owned()),
        }
    }

    fn lane_renamed(&mut self, lane_id: &LaneId, name: &Name) {
        match lane_id {
            LaneId::Audience(id) => {
                if let Some(a) = self
                    .audiences
                    .iter_mut()
                    .find(|audience| id == &audience.id())
                {
                    a.name = name.to_owned();
                }
            }
            LaneId::Stream(id) => {
                if let Some(s) = self.streams.iter_mut().find(|stream| id == &stream.id()) {
                    s.name = name.to_owned();
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
                    .position(|audience| id == &audience.id())
                {
                    let audience = self.audiences.remove(idx);
                    self.audiences.insert(index, audience);
                }
            }
            LaneId::Stream(id) => {
                if let Some(idx) = self
                    .streams
                    .iter_mut()
                    .position(|stream| id == &stream.id())
                {
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
                    .position(|audience| id == &audience.id())
                {
                    self.audiences.remove(idx);
                }
            }
            LaneId::Stream(id) => {
                if let Some(idx) = self
                    .streams
                    .iter_mut()
                    .position(|stream| id == &stream.id())
                {
                    self.streams.remove(idx);
                }
            }
            _ => (),
        }
    }

    fn plus_flow(&mut self, flow_arrow: &FlowArrow) {
        self.flows
            .insert(flow_arrow.id().to_owned(), flow_arrow.to_owned());
    }

    fn minus_flow(&mut self, flow_id: &FlowId) {
        self.flows.remove(flow_id);
    }
}
