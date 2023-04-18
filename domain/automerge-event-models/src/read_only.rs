use std::collections::HashMap;

use automerge::AutoCommit;
use event_models::{
    types::{
        schema::HasSchema, Audience, Command, CommandId, ComponentId, ComponentMut, Described,
        Entity, Event, EventId, FlowArrow, FlowId, Interface, InterfaceId, Named, Placement,
        PlacementId, ReadModel, ReadModelId, Schema, Stream,
    },
    EventModelData, EventModelId,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub(crate) struct ReadOnlyEventModel {
    id: EventModelId,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) interfaces: HashMap<InterfaceId, Interface>,
    pub(crate) commands: HashMap<CommandId, Command>,
    pub(crate) events: HashMap<EventId, Event>,
    pub(crate) read_models: HashMap<ReadModelId, ReadModel>,
    pub(crate) audiences: Vec<Audience>,
    pub(crate) streams: Vec<Stream>,
    pub(crate) placements: HashMap<PlacementId, Placement>,
    pub(crate) flows: HashMap<FlowId, FlowArrow>,
    pub(crate) schema: Schema,
}

impl ReadOnlyEventModel {
    pub(crate) fn component_mut_by_id(&mut self, id: &ComponentId) -> Option<ComponentMut> {
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

impl From<AutoCommit> for ReadOnlyEventModel {
    fn from(_: AutoCommit) -> Self {
        todo!()
    }
}

impl Entity for ReadOnlyEventModel {
    fn id(&self) -> &Uuid {
        &self.id
    }
}

impl Named for ReadOnlyEventModel {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Described for ReadOnlyEventModel {
    fn description(&self) -> &str {
        &self.description
    }
}

impl HasSchema for ReadOnlyEventModel {
    fn schema(&self) -> &Schema {
        &self.schema
    }
}

impl EventModelData for ReadOnlyEventModel {
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
