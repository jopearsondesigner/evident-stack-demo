use std::collections::HashMap;
use uuid::Uuid;
use crate::domain::types::audience::Audience;
use crate::domain::types::command::{Command, CommandId};
use crate::domain::types::errors::EventModelCreationError;
use crate::domain::types::event::{Event, EventId};
use crate::domain::types::event_model::{EventModel, EventModelBuilder, EventModelComponentBuilder, EventModelFlowBuilder, EventModelId, EventModelLaneBuilder, EventModelPlacementBuilder, EventModelSchemaBuilder, validate_name};
use crate::domain::types::flow::{FlowArrow, FlowId};
use crate::domain::types::interface::{Interface, InterfaceId};
use crate::domain::types::{Component, ComponentId, Described, Entity, Lane, LaneId, LaneIndex, Named, PlacementPosition};
use crate::domain::types::placement::{Placement, PlacementId};
use crate::domain::types::read_model::{ReadModel, ReadModelId};
use crate::domain::types::schema::{Schema, SchemaId};
use crate::domain::types::stream::Stream;

#[derive(Debug, Clone, PartialEq)]
pub struct InMemoryEventModel {
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

impl InMemoryEventModel {
    pub fn new(id: &Uuid, name: &str) -> Result<InMemoryEventModel, EventModelCreationError> {
        let name = validate_name(name)?;
        Ok(InMemoryEventModel {
            id: id.to_owned(),
            name,
            description: None,
            interfaces: Default::default(),
            commands: Default::default(),
            events: Default::default(),
            read_models: Default::default(),
            audiences: vec![],
            streams: vec![],
            placements: Default::default(),
            flows: Default::default(),
            schemas: Default::default(),
        })
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

    fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

impl Described for InMemoryEventModel {
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn add_to_description(&mut self, index: u32, addition: &str) {
        match &mut self.description {
            None => self.description = Some(addition.to_string()), // Ignore index?
            Some(desc) => {
                desc.insert_str(index as usize, addition);
            }
        };
    }

    fn remove_from_description(&mut self, index: u32) {
        match &mut self.description {
            None => (),
            Some(desc) => { desc.remove(index as usize); }
        };
    }
}

impl EventModel for InMemoryEventModel {
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

impl EventModelBuilder for InMemoryEventModel {
    fn renamed(mut self, name: &str) -> Self {
        self.rename(name);
        self
    }

    fn added_to_description(mut self, index: u32, addition: &str) -> Self {
        self.add_to_description(index, addition);
        self
    }

    fn deleted_from_description(mut self, index: u32) -> Self {
        self.remove_from_description(index);
        self
    }
}
impl EventModelComponentBuilder for InMemoryEventModel {
    fn component_defined(mut self, component: Component) -> Self {
        match component {
            Component::InterfaceComponent(i) => { self.interfaces.insert(*i.id(), i.to_owned()); }
            Component::CommandComponent(c) => { self.commands.insert(*c.id(), c.to_owned()); }
            Component::EventComponent(e) => { self.events.insert(*e.id(), e.to_owned()); }
            Component::ReadModelComponent(r) => { self.read_models.insert(*r.id(), r.to_owned()); }
        }
        self
    }

    fn component_renamed(mut self, component_id: ComponentId, name: &str) -> Self {
        match component_id {
            ComponentId::InterfaceComponentId(id) => {
                match self.interfaces.get_mut(id) {
                    None => { panic!("Interface not found {:?}", id) }
                    Some(interface) => {
                        interface.rename(name);
                    }
                };
            }
            ComponentId::CommandComponentId(id) => {
                match self.commands.get_mut(id) {
                    None => { panic!("Command with id {:?} not found", id) }
                    Some(command) => {
                        command.rename(name);
                    }
                };
            }
            ComponentId::EventComponentId(id) => {
                match self.events.get_mut(id) {
                    None => { panic!("Event with id {:?} not found", id) }
                    Some(event) => {
                        event.rename(name);
                    }
                };
            }
            ComponentId::ReadModelComponentId(id) => {
                match self.read_models.get_mut(id) {
                    None => { panic!("Read model with id {:?} not found", id) }
                    Some(read_model) => {
                        read_model.rename(name);
                    }
                };
            }
        };
        self
    }

    fn component_removed(mut self, component_id: ComponentId) -> Self {
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
        self, component_id: ComponentId, index: u32, addition: &str
    ) -> Self {
        todo!()
    }

    fn deleted_from_component_description(self, component_id: ComponentId, index: u32) -> Self {
        todo!()
    }
}

impl EventModelPlacementBuilder for InMemoryEventModel {
    fn component_placed(self, placement: &Placement) -> Self {
        todo!()
    }

    fn placement_moved(self, placement_id: &PlacementId, position: &PlacementPosition) -> Self {
        todo!()
    }

    fn placement_removed(self, placement_id: &PlacementId) -> Self {
        todo!()
    }
}

impl EventModelLaneBuilder for InMemoryEventModel {
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

impl EventModelFlowBuilder for InMemoryEventModel {
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

impl EventModelSchemaBuilder for InMemoryEventModel {
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

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use crate::domain::types::{Component, ComponentId, Described, Entity, Named};
    use crate::domain::types::command::Command;
    use crate::domain::types::event::Event;
    use crate::domain::types::event_model::{EventModel, EventModelBuilder, EventModelComponentBuilder, EventModelSchemaBuilder};
    use crate::domain::types::interface::Interface;
    use crate::domain::types::read_model::ReadModel;
    use crate::in_memory::InMemoryEventModel;

    #[test]
    fn successful_creation() {
        let id = Uuid::new_v4();
        let name = "foo";
        let result = InMemoryEventModel::new(&id, name);
        assert!(result.is_ok());

        let event_model = result.unwrap();
        assert_eq!(event_model.id(), &id);
        assert_eq!(event_model.name(), "foo");
        assert_eq!(event_model.description(), None);
    }

    #[test]
    fn failing_creation_due_to_empty_name() {
        let id = Uuid::new_v4();
        let result = InMemoryEventModel::new(&id, "");
        assert!(result.is_err());
    }

    #[test]
    fn renaming() {
        let id = Uuid::new_v4();
        let initial = InMemoryEventModel::new(&id, "foo").unwrap();
        let result = initial.renamed("bar");
        assert_eq!(result.name, "bar");
    }

    #[test]
    fn adding_to_description() {
        let id = Uuid::new_v4();
        let initial = InMemoryEventModel::new(&id, "foo").unwrap();
        assert_eq!(initial.description, None);
        let result = initial.added_to_description(0, "foo bar");
        assert_eq!(result.description, Some("foo bar".to_string()));
    }

    #[test]
    fn deleting_from_description() {
        let id = Uuid::new_v4();
        let initial = InMemoryEventModel::new(&id, "foo").unwrap();
        let result = initial
            .added_to_description(0, "foo bar")
            .deleted_from_description(2);
        assert_eq!(result.description, Some("fo bar".to_string()));
    }

    #[test]
    fn defining_components() {
        let id = Uuid::new_v4();
        let initial = InMemoryEventModel::new(&id, "foo").unwrap();

        let interface = Interface::new(Uuid::new_v4(), "an interface").unwrap();
        let command = Command::new(Uuid::new_v4(), "a command").unwrap();
        let event = Event::new(Uuid::new_v4(), "an event");
        let read_model = ReadModel::new(Uuid::new_v4(), "a read model");
        let result = initial
            .component_defined(Component::InterfaceComponent(&interface))
            .component_defined(Component::CommandComponent(&command))
            .component_defined(Component::EventComponent(&event))
            .component_defined(Component::ReadModelComponent(&read_model));
        assert_eq!(result.interfaces().get(&interface.id()), Some(&interface));
        assert_eq!(result.commands().get(&command.id()), Some(&command));
        assert_eq!(result.events().get(&event.id()), Some(&event));
        assert_eq!(result.read_models().get(&read_model.id()), Some(&read_model));
    }

    #[test]
    fn renaming_components() {
        let id = Uuid::new_v4();
        let interface = Interface::new(Uuid::new_v4(), "an interface").unwrap();
        let command = Command::new(Uuid::new_v4(), "a command").unwrap();
        let event = Event::new(Uuid::new_v4(), "an event");
        let read_model = ReadModel::new(Uuid::new_v4(), "a read model");

        let initial = InMemoryEventModel::new(&id, "foo")
            .unwrap()
            .component_defined(Component::InterfaceComponent(&interface))
            .component_defined(Component::CommandComponent(&command))
            .component_defined(Component::EventComponent(&event))
            .component_defined(Component::ReadModelComponent(&read_model));

        let new_interface_name = "interface foo";
        let new_command_name = "command foo";
        let new_event_name = "event foo";
        let new_read_model_name = "read model foo";

        let result = initial
            .component_renamed(
                ComponentId::InterfaceComponentId(interface.id()),
                new_interface_name
            )
            .component_renamed(
                ComponentId::CommandComponentId(command.id()),
                new_command_name
            )
            .component_renamed(
                ComponentId::EventComponentId(event.id()),
                new_event_name
            )
            .component_renamed(
                ComponentId::ReadModelComponentId(read_model.id()),
                new_read_model_name
            );
        assert_eq!(
            result.interfaces()
                .get(&interface.id()).unwrap()
                .name(),
            new_interface_name
        );
        assert_eq!(
            result.commands()
                .get(&command.id()).unwrap()
                .name(),
            new_command_name
        );
        assert_eq!(
            result.events()
                .get(&event.id()).unwrap()
                .name(),
            new_event_name
        );
        assert_eq!(
            result.read_models()
                .get(&read_model.id()).unwrap()
                .name(),
            new_read_model_name
        );
    }
}
