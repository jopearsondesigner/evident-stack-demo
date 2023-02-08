use crate::default::InMemoryEventModel;
use crate::{
    EventModel, EventModelComponentModifier, EventModelModifier, EventModelSchemaModifier,
};
use uuid::Uuid;

use crate::types::command::Command;
use crate::types::event::Event;
use crate::types::interface::Interface;
use crate::types::read_model::ReadModel;
use crate::types::ComponentId::{
    CommandComponentId, EventComponentId, InterfaceComponentId, ReadModelComponentId,
};
use crate::types::{Component, Described, Entity, Named};

#[test]
fn successful_creation() {
    let id = Uuid::new_v4();
    let name = "foo".to_string();
    let result = InMemoryEventModel::new(id, name);

    let event_model = result;
    assert_eq!(event_model.id(), &id);
    assert_eq!(event_model.name(), "foo");
    assert_eq!(event_model.description(), None);
}

#[test]
fn renaming() {
    let id = Uuid::new_v4();
    let initial = InMemoryEventModel::new(id, "foo".to_string());
    initial.rename("bar");
    assert_eq!(initial.name(), "bar");
}

#[test]
fn adding_to_description() {
    let id = Uuid::new_v4();
    let initial = InMemoryEventModel::new(id, "foo".to_string());
    assert_eq!(initial.description(), None);
    let result = initial.added_to_description(0, "foo bar");
    assert_eq!(result.description, Some("foo bar".to_string()));
}

#[test]
fn deleting_from_description() {
    let id = Uuid::new_v4();
    let initial = InMemoryEventModel::new(id, "foo".to_string());
    let result = initial
        .added_to_description(0, "foo bar")
        .deleted_from_description(2);
    assert_eq!(result.description, Some("fo bar".to_string()));
}

#[test]
fn defining_components() {
    let id = Uuid::new_v4();
    let initial = InMemoryEventModel::new(id, "foo".to_string());

    let interface = Interface::new(Uuid::new_v4(), "an interface").unwrap();
    let command = Command::new(Uuid::new_v4(), "a command").unwrap();
    let event = Event::new(Uuid::new_v4(), "an event");
    let read_model = ReadModel::new(Uuid::new_v4(), "a read model");
    let result = initial
        .component_defined(Component::InterfaceComponent(interface.to_owned()))
        .component_defined(Component::CommandComponent(command.to_owned()))
        .component_defined(Component::EventComponent(event.to_owned()))
        .component_defined(Component::ReadModelComponent(read_model.to_owned()));
    assert_eq!(result.interfaces().get(&interface.id()), Some(&interface));
    assert_eq!(result.commands().get(&command.id()), Some(&command));
    assert_eq!(result.events().get(&event.id()), Some(&event));
    assert_eq!(
        result.read_models().get(&read_model.id()),
        Some(&read_model)
    );
}

#[test]
fn renaming_components() {
    let id = Uuid::new_v4();
    let interface = Interface::new(Uuid::new_v4(), "an interface").unwrap();
    let command = Command::new(Uuid::new_v4(), "a command").unwrap();
    let event = Event::new(Uuid::new_v4(), "an event");
    let read_model = ReadModel::new(Uuid::new_v4(), "a read model");

    let initial = InMemoryEventModel::new(id, "foo".to_string())
        .component_defined(Component::InterfaceComponent(interface.to_owned()))
        .component_defined(Component::CommandComponent(command.to_owned()))
        .component_defined(Component::EventComponent(event.to_owned()))
        .component_defined(Component::ReadModelComponent(read_model.to_owned()));

    let new_interface_name = "interface foo";
    let new_command_name = "command foo";
    let new_event_name = "event foo";
    let new_read_model_name = "read model foo";

    let result = initial
        .component_renamed(
            &InterfaceComponentId(interface.id().to_owned()),
            new_interface_name,
        )
        .component_renamed(
            &CommandComponentId(command.id().to_owned()),
            new_command_name,
        )
        .component_renamed(&EventComponentId(event.id().to_owned()), new_event_name)
        .component_renamed(
            &ReadModelComponentId(read_model.id().to_owned()),
            new_read_model_name,
        );
    assert_eq!(
        result.interfaces().get(&interface.id()).unwrap().name(),
        new_interface_name
    );
    assert_eq!(
        result.commands().get(&command.id()).unwrap().name(),
        new_command_name
    );
    assert_eq!(
        result.events().get(&event.id()).unwrap().name(),
        new_event_name
    );
    assert_eq!(
        result.read_models().get(&read_model.id()).unwrap().name(),
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

    let initial = InMemoryEventModel::new(id, "foo".to_string())
        .component_defined(Component::InterfaceComponent(interface.to_owned()))
        .component_defined(Component::CommandComponent(command.to_owned()))
        .component_defined(Component::EventComponent(event.to_owned()))
        .component_defined(Component::ReadModelComponent(read_model.to_owned()));

    let result = initial
        .added_to_component_description(
            &InterfaceComponentId(*interface.id()),
            0,
            "a really nice interface",
        )
        .added_to_component_description(
            &CommandComponentId(*command.id()),
            0,
            "a really nice command",
        )
        .added_to_component_description(&EventComponentId(*event.id()), 0, "a really nice event")
        .added_to_component_description(
            &ReadModelComponentId(*read_model.id()),
            0,
            "a really nice read model",
        );
    assert_eq!(
        result.interfaces.get(interface.id()).unwrap().description(),
        Some("a really nice interface")
    );
    assert_eq!(
        result.commands.get(command.id()).unwrap().description(),
        Some("a really nice command")
    );
    assert_eq!(
        result.events.get(event.id()).unwrap().description(),
        Some("a really nice event")
    );
    assert_eq!(
        result
            .read_models
            .get(read_model.id())
            .unwrap()
            .description(),
        Some("a really nice read model")
    );
}

#[test]
fn deleting_from_component_description() {
    let id = Uuid::new_v4();
    let interface = Interface::new(Uuid::new_v4(), "an interface").unwrap();
    let command = Command::new(Uuid::new_v4(), "a command").unwrap();
    let event = Event::new(Uuid::new_v4(), "an event");
    let read_model = ReadModel::new(Uuid::new_v4(), "a read model");

    let initial = InMemoryEventModel::new(id, "foo".to_string())
        .component_defined(Component::InterfaceComponent(interface.to_owned()))
        .component_defined(Component::CommandComponent(command.to_owned()))
        .component_defined(Component::EventComponent(event.to_owned()))
        .component_defined(Component::ReadModelComponent(read_model.to_owned()));

    let result = initial
        .added_to_component_description(
            &InterfaceComponentId(*interface.id()),
            0,
            "a really nice interface",
        )
        .added_to_component_description(
            &CommandComponentId(*command.id()),
            0,
            "a really nice command",
        )
        .added_to_component_description(&EventComponentId(*event.id()), 0, "a really nice event")
        .added_to_component_description(
            &ReadModelComponentId(*read_model.id()),
            0,
            "a really nice read model",
        )
        .deleted_from_component_description(&InterfaceComponentId(*interface.id()), 2)
        .deleted_from_component_description(&CommandComponentId(*command.id()), 2)
        .deleted_from_component_description(&EventComponentId(*event.id()), 2)
        .deleted_from_component_description(&ReadModelComponentId(*read_model.id()), 2);

    assert_eq!(
        result.interfaces.get(interface.id()).unwrap().description(),
        Some("a eally nice interface")
    );
    assert_eq!(
        result.commands.get(command.id()).unwrap().description(),
        Some("a eally nice command")
    );
    assert_eq!(
        result.events.get(event.id()).unwrap().description(),
        Some("a eally nice event")
    );
    assert_eq!(
        result
            .read_models
            .get(read_model.id())
            .unwrap()
            .description(),
        Some("a eally nice read model")
    );
}
