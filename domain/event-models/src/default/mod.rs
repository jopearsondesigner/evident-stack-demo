use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::types::audience::Audience;
use crate::types::command::{Command, CommandId};
use crate::types::event::{Event, EventId};
use crate::types::flow::{FlowArrow, FlowId};
use crate::types::interface::{Interface, InterfaceId};
use crate::types::placement::{Placement, PlacementId, PlacementPosition};
use crate::types::read_model::{ReadModel, ReadModelId};
use crate::types::schema::{Schema, SchemaId};
use crate::types::stream::Stream;
use crate::types::{
    Component, ComponentId, ComponentMut, Described, Entity, Lane, LaneId, LaneIndex, Named,
};
use crate::{
    EventModel, EventModelComponentModifier, EventModelFlowModifier, EventModelId,
    EventModelLaneModifier, EventModelModifier, EventModelPlacementModifier,
    EventModelSchemaModifier,
};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DefaultEventModel {
    id: EventModelId,
    name: String,
    description: Option<String>,
    interfaces: HashMap<InterfaceId, Interface>,
    commands: HashMap<CommandId, Command>,
    events: HashMap<EventId, Event>,
    read_models: HashMap<ReadModelId, ReadModel>,
    audiences: Vec<Audience>,
    streams: Vec<Stream>,
    placements: HashMap<PlacementId, Placement>,
    flows: HashMap<FlowId, FlowArrow>,
    schemas: HashMap<SchemaId, Schema>,
}

impl DefaultEventModel {
    pub fn new(id: EventModelId, name: String) -> Self {
        DefaultEventModel {
            id,
            name,
            description: None,
            interfaces: Default::default(),
            commands: Default::default(),
            events: Default::default(),
            read_models: Default::default(),
            audiences: Default::default(),
            streams: Default::default(),
            placements: Default::default(),
            flows: Default::default(),
            schemas: Default::default(),
        }
    }
}

impl Default for DefaultEventModel {
    fn default() -> Self {
        Self::new(Uuid::new_v4(), "Default".to_string())
    }
}

impl Entity for DefaultEventModel {
    fn id(&self) -> &Uuid {
        &self.id
    }
}

impl Named for DefaultEventModel {
    fn name(&self) -> &str {
        &self.name
    }

    fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

impl Described for DefaultEventModel {
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn set_description(&mut self, description: &str) {
        if description.is_empty() {
            self.description = None
        } else {
            self.description = Some(description.to_string());
        }
    }
}

impl EventModel for DefaultEventModel {
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

    fn schemas(&self) -> &HashMap<SchemaId, Schema> {
        &self.schemas
    }
}

fn add_to_description(described: &mut impl Described, index: u32, addition: &str) {
    match described.description() {
        None => {
            described.set_description(addition);
        }
        Some(desc) => {
            let mut description: String = desc.to_string();
            description.insert_str(index as usize, addition);
            described.set_description(&description);
        }
    }
}

fn delete_from_description(described: &mut impl Described, index: u32) {
    match described.description() {
        None => {
            described.set_description("");
        }
        Some(desc) => {
            let mut description: String = desc.to_string();
            description.remove(index as usize);
            described.set_description(&*description);
        }
    }
}

impl EventModelModifier for DefaultEventModel {
    fn renamed(mut self, name: &str) -> Self {
        self.rename(name);
        self
    }

    fn added_to_description(mut self, index: u32, addition: &str) -> Self {
        match &mut self.description {
            None => self.set_description(addition), // Ignore index?
            Some(desc) => {
                desc.insert_str(index as usize, addition);
            }
        };
        self
    }

    fn deleted_from_description(mut self, index: u32) -> Self {
        match &mut self.description {
            None => (),
            Some(desc) => {
                desc.remove(index as usize);
            }
        };
        self
    }
}

impl EventModelComponentModifier for DefaultEventModel {
    fn component_mut_by_id(&mut self, id: &ComponentId) -> Option<ComponentMut> {
        match id {
            ComponentId::InterfaceComponentId(id) => self
                .interfaces
                .get_mut(id)
                .map(|i| ComponentMut::InterfaceComponentMut(i)),
            ComponentId::CommandComponentId(id) => self
                .commands
                .get_mut(id)
                .map(|i| ComponentMut::CommandComponentMut(i)),
            ComponentId::EventComponentId(id) => self
                .events
                .get_mut(id)
                .map(|i| ComponentMut::EventComponentMut(i)),
            ComponentId::ReadModelComponentId(id) => self
                .read_models
                .get_mut(id)
                .map(|i| ComponentMut::ReadModelComponentMut(i)),
        }
    }

    fn component_defined(mut self, component: Component) -> Self {
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
        self
    }

    fn component_renamed(mut self, component_id: &ComponentId, name: &str) -> Self {
        match self.component_mut_by_id(&component_id) {
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
        self
    }

    fn component_removed(mut self, component_id: &ComponentId) -> Self {
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
        self
    }

    fn added_to_component_description(
        mut self,
        component_id: &ComponentId,
        index: u32,
        addition: &str,
    ) -> Self {
        match self.component_mut_by_id(&component_id) {
            None => {
                panic!("Component with id {:?} not found", component_id)
            }
            Some(component) => match component {
                ComponentMut::InterfaceComponentMut(i) => {
                    add_to_description(i, index, addition);
                }
                ComponentMut::CommandComponentMut(c) => {
                    add_to_description(c, index, addition);
                }
                ComponentMut::EventComponentMut(e) => {
                    add_to_description(e, index, addition);
                }
                ComponentMut::ReadModelComponentMut(r) => {
                    add_to_description(r, index, addition);
                }
            },
        }
        self
    }

    fn deleted_from_component_description(
        mut self,
        component_id: &ComponentId,
        index: u32,
    ) -> Self {
        match self.component_mut_by_id(&component_id) {
            None => {
                panic!("Component with id {:?} not found", component_id)
            }
            Some(component) => match component {
                ComponentMut::InterfaceComponentMut(i) => {
                    delete_from_description(i, index);
                }
                ComponentMut::CommandComponentMut(c) => {
                    delete_from_description(c, index);
                }
                ComponentMut::EventComponentMut(e) => {
                    delete_from_description(e, index);
                }
                ComponentMut::ReadModelComponentMut(r) => {
                    delete_from_description(r, index);
                }
            },
        }
        self
    }
}

impl EventModelPlacementModifier for DefaultEventModel {
    fn component_placed(self, placement: &Placement) -> Self {
        todo!()
    }

    fn placement_moved(self, position: &PlacementPosition) -> Self {
        todo!()
    }

    fn placement_removed(self, placement_id: &PlacementId) -> Self {
        todo!()
    }
}

impl EventModelLaneModifier for DefaultEventModel {
    fn lane_added(self, lane: Lane, index: LaneIndex) -> Self {
        todo!()
    }

    fn lane_renamed(self, lane_id: LaneId, name: &str) -> Self {
        todo!()
    }

    fn lane_reordered(self, lane_id: LaneId, index: LaneIndex) -> Self {
        todo!()
    }

    fn lane_removed(self, lane_id: LaneId) -> Self {
        todo!()
    }
}

impl EventModelFlowModifier for DefaultEventModel {
    fn plus_flow(self, flow_arrow: &FlowArrow) -> Self {
        todo!()
    }

    fn minus_flow_by_placement_ids(self, from: &PlacementId, to: &PlacementId) -> Self {
        todo!()
    }

    fn minus_flow(self, flow_id: &FlowId) -> Self {
        todo!()
    }
}

impl EventModelSchemaModifier for DefaultEventModel {
    fn schema_defined(self, schema: &Schema) -> Self {
        todo!()
    }

    fn added_to_schema_definition(self, schema_id: &SchemaId, index: u32, addition: &str) -> Self {
        todo!()
    }

    fn deleted_from_schema_definition(self, schema_id: &SchemaId, index: u32) -> Self {
        todo!()
    }

    fn added_to_schema_description(self, schema_id: &SchemaId, index: u32, addition: &str) -> Self {
        todo!()
    }

    fn deleted_from_schema_description(self, schema_id: &SchemaId, index: u32) -> Self {
        todo!()
    }

    fn remove_schema(self, schema_id: &SchemaId) -> Self {
        todo!()
    }
}
