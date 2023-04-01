use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::types::audience::Audience;
use crate::types::command::{Command, CommandId};
use crate::types::event::{Event, EventId};
use crate::types::flow::{FlowArrow, FlowId};
use crate::types::interface::{Interface, InterfaceId};
use crate::types::placement::{Placement, PlacementId, PlacementPosition};
use crate::types::read_model::{ReadModel, ReadModelId};
use crate::types::schema::{HasModifiableSchema, HasSchema, Schema};
use crate::types::stream::Stream;
use crate::types::{
    Component, ComponentId, ComponentMut, Described, Entity, Lane, LaneId, LaneIndex,
    ModifiablyDescribed, Named, Renamable,
};
use crate::{EventModel, EventModelData, EventModelId, EventModelState, ModifiableEventModel};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InMemoryEventModel {
    id: EventModelId,
    name: String,
    description: String,
    interfaces: HashMap<InterfaceId, Interface>,
    commands: HashMap<CommandId, Command>,
    events: HashMap<EventId, Event>,
    read_models: HashMap<ReadModelId, ReadModel>,
    audiences: Vec<Audience>,
    streams: Vec<Stream>,
    placements: HashMap<PlacementId, Placement>,
    flows: HashMap<FlowId, FlowArrow>,
    schema: Schema,
}

impl InMemoryEventModel {
    pub fn new(id: EventModelId, name: String) -> InMemoryEventModel {
        InMemoryEventModel {
            id,
            name,
            description: Default::default(),
            interfaces: Default::default(),
            commands: Default::default(),
            events: Default::default(),
            read_models: Default::default(),
            audiences: Default::default(),
            streams: Default::default(),
            placements: Default::default(),
            flows: Default::default(),
            schema: Default::default(),
        }
    }

    fn component_mut_by_id(&mut self, id: &ComponentId) -> Option<ComponentMut> {
        match id {
            ComponentId::InterfaceComponentId(id) => self
                .interfaces
                .get_mut(id)
                .map(ComponentMut::InterfaceComponentMut),
            ComponentId::CommandComponentId(id) => self
                .commands
                .get_mut(id)
                .map(ComponentMut::CommandComponentMut),
            ComponentId::EventComponentId(id) => {
                self.events.get_mut(id).map(ComponentMut::EventComponentMut)
            }
            ComponentId::ReadModelComponentId(id) => self
                .read_models
                .get_mut(id)
                .map(ComponentMut::ReadModelComponentMut),
        }
    }
}

impl Default for InMemoryEventModel {
    fn default() -> Self {
        Self::new(Uuid::new_v4(), "Default".to_string())
    }
}

impl Entity for InMemoryEventModel {
    fn id(&self) -> &Uuid {
        &self.id
    }
}

impl Named for InMemoryEventModel {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Renamable for InMemoryEventModel {
    fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

impl Described for InMemoryEventModel {
    fn description(&self) -> &str {
        &self.description
    }
}

impl ModifiablyDescribed for InMemoryEventModel {
    fn description_mut(&mut self) -> &mut String {
        &mut self.description
    }
}

impl HasSchema for InMemoryEventModel {
    fn schema(&self) -> &Schema {
        &self.schema
    }
}

impl HasModifiableSchema for InMemoryEventModel {
    fn schema_mut(&mut self) -> &mut Schema {
        &mut self.schema
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InMemoryCreationDetails;

impl EventModel for InMemoryEventModel {
    type CreationDetails = InMemoryCreationDetails;

    fn create(initial: EventModelState<Self>, id: EventModelId, name: String) -> Self {
        match initial {
            EventModelState::BeforeCreation(_) => InMemoryEventModel::new(id, name),
            EventModelState::EventModel(_) => panic!("Illegal state when creating Event Model!"),
        }
    }
}

impl EventModelData for InMemoryEventModel {
    fn interfaces(&self) -> &HashMap<InterfaceId, Interface> {
        &self.interfaces
    }

    fn commands(&self) -> &HashMap<CommandId, Command> {
        &self.commands
    }

    fn events(&self) -> &HashMap<EventId, Event> {
        &self.events
    }

    fn read_models(&self) -> &HashMap<ReadModelId, ReadModel> {
        &self.read_models
    }

    fn audiences(&self) -> &Vec<Audience> {
        &self.audiences
    }

    fn streams(&self) -> &Vec<Stream> {
        &self.streams
    }

    fn placements(&self) -> &HashMap<PlacementId, Placement> {
        &self.placements
    }

    fn flows(&self) -> &HashMap<FlowId, FlowArrow> {
        &self.flows
    }
}

impl ModifiableEventModel for InMemoryEventModel {
    fn component_defined(&mut self, component: Component) {
        match component {
            Component::InterfaceComponent(i) => {
                self.interfaces.insert(*i.id(), i.to_owned());
            }
            Component::CommandComponent(c) => {
                self.commands.insert(*c.id(), c.to_owned());
            }
            Component::EventComponent(e) => {
                self.events.insert(*e.id(), e.to_owned());
            }
            Component::ReadModelComponent(r) => {
                self.read_models.insert(*r.id(), r.to_owned());
            }
        }
    }

    fn component_renamed(&mut self, component_id: &ComponentId, name: &str) {
        match self.component_mut_by_id(component_id) {
            None => {
                panic!("Component with id {:?} not found", component_id)
            }
            Some(component) => match component {
                ComponentMut::InterfaceComponentMut(i) => {
                    i.rename(name);
                }
                ComponentMut::CommandComponentMut(c) => {
                    c.rename(name);
                }
                ComponentMut::EventComponentMut(e) => {
                    e.rename(name);
                }
                ComponentMut::ReadModelComponentMut(r) => r.rename(name),
            },
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

    fn added_to_component_description(
        &mut self,
        component_id: &ComponentId,
        index: usize,
        addition: &str,
    ) {
        match self.component_mut_by_id(component_id) {
            None => {
                panic!("Component with id {:?} not found", component_id)
            }
            Some(component) => match component {
                ComponentMut::InterfaceComponentMut(i) => {
                    i.add_to_description(index, addition);
                }
                ComponentMut::CommandComponentMut(c) => {
                    c.add_to_description(index, addition);
                }
                ComponentMut::EventComponentMut(e) => {
                    e.add_to_description(index, addition);
                }
                ComponentMut::ReadModelComponentMut(r) => {
                    r.add_to_description(index, addition);
                }
            },
        }
    }

    fn deleted_from_component_description(
        &mut self,
        component_id: &ComponentId,
        index: usize,
        count: usize,
    ) {
        match self.component_mut_by_id(component_id) {
            None => {
                panic!("Component with id {:?} not found", component_id)
            }
            Some(component) => match component {
                ComponentMut::InterfaceComponentMut(i) => {
                    i.delete_from_description(index, count);
                }
                ComponentMut::CommandComponentMut(c) => {
                    c.delete_from_description(index, count);
                }
                ComponentMut::EventComponentMut(e) => {
                    e.delete_from_description(index, count);
                }
                ComponentMut::ReadModelComponentMut(r) => {
                    r.delete_from_description(index, count);
                }
            },
        }
    }

    fn added_to_component_schema(
        &mut self,
        component_id: &ComponentId,
        index: usize,
        addition: &str,
    ) {
        match self.component_mut_by_id(component_id) {
            None => {
                panic!("Component with id {:?} not found", component_id)
            }
            Some(component) => match component {
                ComponentMut::InterfaceComponentMut(_) => (),
                ComponentMut::CommandComponentMut(c) => {
                    c.add_to_schema(index, addition);
                }
                ComponentMut::EventComponentMut(e) => {
                    e.add_to_schema(index, addition);
                }
                ComponentMut::ReadModelComponentMut(r) => {
                    r.add_to_schema(index, addition);
                }
            },
        }
    }

    fn deleted_from_component_schema(
        &mut self,
        component_id: &ComponentId,
        index: usize,
        count: usize,
    ) {
        match self.component_mut_by_id(component_id) {
            None => {
                panic!("Component with id {:?} not found", component_id)
            }
            Some(component) => match component {
                ComponentMut::InterfaceComponentMut(_) => (),
                ComponentMut::CommandComponentMut(c) => {
                    c.delete_from_schema(index, count);
                }
                ComponentMut::EventComponentMut(e) => {
                    e.delete_from_schema(index, count);
                }
                ComponentMut::ReadModelComponentMut(r) => {
                    r.delete_from_schema(index, count);
                }
            },
        }
    }

    fn component_placed(&mut self, placement: &Placement) {
        self.placements
            .insert(placement.id().to_owned(), placement.to_owned());
    }

    fn placement_moved(&mut self, position: &PlacementPosition) {
        if let Some(ref mut placement) = self.placements.get_mut(position.id()) {
            let PlacementPosition(_, index, lane) = position;
            placement.relocate(index.to_owned(), lane.to_owned());
        };
    }

    fn placement_removed(&mut self, placement_id: &PlacementId) {
        self.placements.remove(placement_id);
    }

    fn added_to_placement_schema(
        &mut self,
        placement_id: &PlacementId,
        index: usize,
        addition: &str,
    ) {
        if let Some(placement) = self.placements.get(placement_id) {
            if let Some(component_mut) = self.component_mut_by_id(&placement.component_id()) {
                match component_mut {
                    ComponentMut::InterfaceComponentMut(_) => (),
                    ComponentMut::CommandComponentMut(c) => c.add_to_schema(index, addition),
                    ComponentMut::EventComponentMut(e) => e.add_to_schema(index, addition),
                    ComponentMut::ReadModelComponentMut(r) => r.add_to_schema(index, addition),
                };
            }
        }
    }

    fn deleted_from_placement_schema(
        &mut self,
        placement_id: &PlacementId,
        index: usize,
        count: usize,
    ) {
        if let Some(placement) = self.placements.get(placement_id) {
            if let Some(component_mut) = self.component_mut_by_id(&placement.component_id()) {
                match component_mut {
                    ComponentMut::InterfaceComponentMut(_) => (),
                    ComponentMut::CommandComponentMut(c) => c.delete_from_schema(index, count),
                    ComponentMut::EventComponentMut(e) => e.delete_from_schema(index, count),
                    ComponentMut::ReadModelComponentMut(r) => r.delete_from_schema(index, count),
                };
            }
        }
    }

    fn lane_added(&mut self, lane: Lane, index: LaneIndex) {
        match lane {
            Lane::Audience(audience) => self.audiences.insert(index, audience),
            Lane::Stream(stream) => self.streams.insert(index, stream),
        }
    }

    fn lane_renamed(&mut self, lane_id: LaneId, name: &str) {
        match lane_id {
            LaneId::Audience(id) => {
                if let Some(a) = self
                    .audiences
                    .iter_mut()
                    .find(|audience| id == *audience.id())
                {
                    a.rename(name);
                }
            }
            LaneId::Stream(id) => {
                if let Some(s) = self.streams.iter_mut().find(|stream| id == *stream.id()) {
                    s.rename(name);
                }
            }
            _ => (),
        }
    }

    fn lane_reordered(&mut self, lane_id: LaneId, index: LaneIndex) {
        match lane_id {
            LaneId::Audience(id) => {
                if let Some(idx) = self
                    .audiences
                    .iter_mut()
                    .position(|audience| id == *audience.id())
                {
                    let audience = self.audiences.remove(idx);
                    self.audiences.insert(index, audience);
                }
            }
            LaneId::Stream(id) => {
                if let Some(idx) = self
                    .streams
                    .iter_mut()
                    .position(|stream| id == *stream.id())
                {
                    let stream = self.streams.remove(idx);
                    self.streams.insert(index, stream);
                }
            }
            _ => (),
        }
    }

    fn lane_removed(&mut self, lane_id: LaneId) {
        match lane_id {
            LaneId::Audience(id) => {
                if let Some(idx) = self
                    .audiences
                    .iter_mut()
                    .position(|audience| id == *audience.id())
                {
                    self.audiences.remove(idx);
                }
            }
            LaneId::Stream(id) => {
                if let Some(idx) = self
                    .streams
                    .iter_mut()
                    .position(|stream| id == *stream.id())
                {
                    self.streams.remove(idx);
                }
            }
            _ => (),
        }
    }

    fn plus_flow(&mut self, flow_arrow: FlowArrow) {
        self.flows.insert(flow_arrow.id().to_owned(), flow_arrow);
    }

    fn minus_flow(&mut self, flow_id: &FlowId) {
        self.flows.remove(flow_id);
    }
}
