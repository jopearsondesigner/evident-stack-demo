use std::collections::HashMap;
use uuid::Uuid;

use crate::{EventModel, EventModelId, EventModelModifier, EventModelComponentModifier,
            EventModelFlowModifier, EventModelLaneModifier, EventModelPlacementModifier,
            EventModelSchemaModifier,
            validate_name};
use crate::types::audience::Audience;
use crate::types::command::{Command, CommandId};
use crate::types::event::{Event, EventId};
use crate::types::flow::{FlowArrow, FlowId};
use crate::types::interface::{Interface, InterfaceId};
use crate::types::{Component, ComponentId, ComponentMut, Described, Entity,
                   Lane, LaneId, LaneIndex, Named, PlacementPosition};
use crate::types::errors::EventModelError;
use crate::types::placement::{Placement, PlacementId};
use crate::types::read_model::{ReadModel, ReadModelId};
use crate::types::schema::{Schema, SchemaId};
use crate::types::stream::Stream;

#[derive(Debug, Clone, PartialEq)]
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

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use crate::{EventModel, EventModelModifier, EventModelComponentModifier, EventModelSchemaModifier};
    use crate::default::DefaultEventModel;

    use crate::types::{Component, Described, Entity, Named};
    use crate::types::command::Command;
    use crate::types::ComponentId::{CommandComponentId, EventComponentId, InterfaceComponentId,
                                    ReadModelComponentId};
    use crate::types::event::Event;
    use crate::types::interface::Interface;
    use crate::types::read_model::ReadModel;

    #[test]
    fn successful_creation() {
        let id = Uuid::new_v4();
        let name = "foo";
        let result = DefaultEventModel::new(&id, name);
        assert!(result.is_ok());

        let event_model = result.unwrap();
        assert_eq!(event_model.id(), &id);
        assert_eq!(event_model.name(), "foo");
        assert_eq!(event_model.description(), None);
    }

    #[test]
    fn failing_creation_due_to_empty_name() {
        let id = Uuid::new_v4();
        let result = DefaultEventModel::new(&id, "");
        assert!(result.is_err());
    }

    #[test]
    fn renaming() {
        let id = Uuid::new_v4();
        let initial = DefaultEventModel::new(&id, "foo").unwrap();
        let result = initial.renamed("bar");
        assert_eq!(result.name, "bar");
    }

    #[test]
    fn adding_to_description() {
        let id = Uuid::new_v4();
        let initial = DefaultEventModel::new(&id, "foo").unwrap();
        assert_eq!(initial.description, None);
        let result = initial.added_to_description(0, "foo bar");
        assert_eq!(result.description, Some("foo bar".to_string()));
    }

    #[test]
    fn deleting_from_description() {
        let id = Uuid::new_v4();
        let initial = DefaultEventModel::new(&id, "foo").unwrap();
        let result = initial
            .added_to_description(0, "foo bar")
            .deleted_from_description(2);
        assert_eq!(result.description, Some("fo bar".to_string()));
    }

    #[test]
    fn defining_components() {
        let id = Uuid::new_v4();
        let initial = DefaultEventModel::new(&id, "foo").unwrap();

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

        let initial = DefaultEventModel::new(&id, "foo")
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
                InterfaceComponentId(interface.id()),
                new_interface_name
            )
            .component_renamed(
                CommandComponentId(command.id()),
                new_command_name
            )
            .component_renamed(
                EventComponentId(event.id()),
                new_event_name
            )
            .component_renamed(
                ReadModelComponentId(read_model.id()),
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

    #[test]
    fn adding_to_component_description() {
        let id = Uuid::new_v4();
        let interface = Interface::new(Uuid::new_v4(), "an interface").unwrap();
        let command = Command::new(Uuid::new_v4(), "a command").unwrap();
        let event = Event::new(Uuid::new_v4(), "an event");
        let read_model = ReadModel::new(Uuid::new_v4(), "a read model");

        let initial = DefaultEventModel::new(&id, "foo")
            .unwrap()
            .component_defined(Component::InterfaceComponent(&interface))
            .component_defined(Component::CommandComponent(&command))
            .component_defined(Component::EventComponent(&event))
            .component_defined(Component::ReadModelComponent(&read_model));

        let result = initial
            .added_to_component_description(
                InterfaceComponentId(interface.id()),
                0,
                "a really nice interface"
            )
            .added_to_component_description(
                CommandComponentId(command.id()),
                0,
                "a really nice command"
            )
            .added_to_component_description(
                EventComponentId(event.id()),
                0,
                "a really nice event"
            )
            .added_to_component_description(
                ReadModelComponentId(read_model.id()),
                0,
                "a really nice read model"
            );
        assert_eq!(result.interfaces
                       .get(interface.id())
                       .unwrap()
                       .description(),
                   Some("a really nice interface"));
        assert_eq!(result.commands
                       .get(command.id())
                       .unwrap()
                       .description(),
                   Some("a really nice command"));
        assert_eq!(result.events
                       .get(event.id())
                       .unwrap()
                       .description(),
                   Some("a really nice event"));
        assert_eq!(result.read_models
                       .get(read_model.id())
                       .unwrap()
                       .description(),
                   Some("a really nice read model"));
    }

    #[test]
    fn deleting_from_component_description() {
        let id = Uuid::new_v4();
        let interface = Interface::new(Uuid::new_v4(), "an interface").unwrap();
        let command = Command::new(Uuid::new_v4(), "a command").unwrap();
        let event = Event::new(Uuid::new_v4(), "an event");
        let read_model = ReadModel::new(Uuid::new_v4(), "a read model");

        let initial = DefaultEventModel::new(&id, "foo")
            .unwrap()
            .component_defined(Component::InterfaceComponent(&interface))
            .component_defined(Component::CommandComponent(&command))
            .component_defined(Component::EventComponent(&event))
            .component_defined(Component::ReadModelComponent(&read_model));

        let result = initial
            .added_to_component_description(
                InterfaceComponentId(interface.id()),
                0,
                "a really nice interface"
            )
            .added_to_component_description(
                CommandComponentId(command.id()),
                0,
                "a really nice command"
            )
            .added_to_component_description(
                EventComponentId(event.id()),
                0,
                "a really nice event"
            )
            .added_to_component_description(
                ReadModelComponentId(read_model.id()),
                0,
                "a really nice read model"
            )
            .deleted_from_component_description(
                InterfaceComponentId(interface.id()),
                2
            )
            .deleted_from_component_description(
                CommandComponentId(command.id()),
                2
            )
            .deleted_from_component_description(
                EventComponentId(event.id()),
                2
            )
            .deleted_from_component_description(
                ReadModelComponentId(read_model.id()),
                2
            );

        assert_eq!(result.interfaces
                       .get(interface.id())
                       .unwrap()
                       .description(),
                   Some("a eally nice interface"));
        assert_eq!(result.commands
                       .get(command.id())
                       .unwrap()
                       .description(),
                   Some("a eally nice command"));
        assert_eq!(result.events
                       .get(event.id())
                       .unwrap()
                       .description(),
                   Some("a eally nice event"));
        assert_eq!(result.read_models
                       .get(read_model.id())
                       .unwrap()
                       .description(),
                   Some("a eally nice read model"));
    }
}

impl DefaultEventModel {
    pub fn new(id: &Uuid, name: &str) -> Result<DefaultEventModel, EventModelError> {
        let name = validate_name(name)?;
        Ok(DefaultEventModel {
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
    fn component_mut_by_id(
        &mut self, id: &ComponentId
    ) -> Option<ComponentMut> {
        match id {
            ComponentId::InterfaceComponentId(id) =>
                self.interfaces.get_mut(id)
                    .map(|i| ComponentMut::InterfaceComponentMut(i)),
            ComponentId::CommandComponentId(id) =>
                self.commands.get_mut(id)
                    .map(|i| ComponentMut::CommandComponentMut(i)),
            ComponentId::EventComponentId(id) =>
                self.events.get_mut(id)
                    .map(|i| ComponentMut::EventComponentMut(i)),
            ComponentId::ReadModelComponentId(id) =>
                self.read_models.get_mut(id)
                    .map(|i| ComponentMut::ReadModelComponentMut(i))
        }
    }

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

fn add_to_description(described: &mut dyn Described, index: u32, addition: &str) {
    match described.description() {
        None => { described.set_description(addition); }
        Some(desc) => {
            let mut description: String = desc.to_string();
            description.insert_str(index as usize, addition);
            described.set_description(&*description);
        }
    }
}

fn delete_from_description(described: &mut dyn Described, index: u32) {
    match described.description() {
        None => { described.set_description(""); }
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
            Some(desc) => { desc.remove(index as usize); }
        };
        self
    }
}

impl EventModelComponentModifier for DefaultEventModel {
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
        match self.component_mut_by_id(&component_id) {
            None => { panic!("Component with id {:?} not found", component_id) }
            Some(component) => {
                match component {
                    ComponentMut::InterfaceComponentMut(i) => { i.rename(name); }
                    ComponentMut::CommandComponentMut(c) => { c.rename(name); }
                    ComponentMut::EventComponentMut(e) => { e.rename(name); }
                    ComponentMut::ReadModelComponentMut(r) => { r.rename(name) }
                }
            }
        }
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
        mut self, component_id: ComponentId, index: u32, addition: &str
    ) -> Self {
        match self.component_mut_by_id(&component_id) {
            None => { panic!("Component with id {:?} not found", component_id) }
            Some(component) => {
                match component {
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
                }
            }
        }
        self
    }

    fn deleted_from_component_description(mut self, component_id: ComponentId, index: u32) -> Self {
        match self.component_mut_by_id(&component_id) {
            None => { panic!("Component with id {:?} not found", component_id) }
            Some(component) => {
                match component {
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
                }
            }
        }
        self
    }
}

impl EventModelPlacementModifier for DefaultEventModel {
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
