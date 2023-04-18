use std::collections::HashMap;

use event_models::{
    types::{
        Audience, Command, CommandId, ComponentId, ComponentMut, Entity, Event, EventId, FlowArrow,
        FlowId, Interface, InterfaceId, Placement, PlacementId, ReadModel, ReadModelId, Schema,
        Stream,
    },
    EventModelId,
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

impl Entity for ReadOnlyEventModel {
    fn id(&self) -> &Uuid {
        &self.id
    }
}
