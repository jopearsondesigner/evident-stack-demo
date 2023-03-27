use crate::shared::{creating_event_model_succeeds, renaming_event_model_succeeds};
use event_models::api::EventModelState;
use event_models::implementation::in_memory::{InMemoryCreator, InMemoryEventModel};
use event_models::{EventModelData, ModifiableEventModel};
use uuid::Uuid;

use event_models::types::ComponentId::{
    CommandComponentId, EventComponentId, InterfaceComponentId, ReadModelComponentId,
};
use event_models::types::{
    Command, Component, Described, Entity, Event, Interface, InterfaceConfig, ModifiablyDescribed,
    Named, ReadModel, Schema,
};

#[test]
fn creation() {
    creating_event_model_succeeds(
        EventModelState::<InMemoryEventModel, InMemoryCreator>::BeforeCreation(
            InMemoryCreator::default(),
        ),
    );
}

#[test]
fn renaming() {
    renaming_event_model_succeeds(
        EventModelState::<InMemoryEventModel, InMemoryCreator>::EventModel(
            InMemoryEventModel::new(Uuid::new_v4(), "foo".to_string()),
        ),
    );
}

#[test]
fn adding_to_description() {
    let id = Uuid::new_v4();
    let mut model = InMemoryEventModel::new(id, "foo".to_string());
    assert_eq!(model.description(), "");
    model.add_to_description(0, "foo bar");
    assert_eq!(model.description(), "foo bar");
}

#[test]
fn deleting_from_description() {
    let id = Uuid::new_v4();
    let mut model = InMemoryEventModel::new(id, "foo".to_string());
    model.add_to_description(0, "foo bar");
    model.delete_from_description(2);
    assert_eq!(model.description(), "fo bar");
}

#[test]
fn defining_components() {
    let id = Uuid::new_v4();
    let mut model = InMemoryEventModel::new(id, "foo".to_string());

    let interface = Interface::create(
        Uuid::new_v4(),
        "an interface".into(),
        "".into(),
        InterfaceConfig::default(),
    )
    .expect("failed to create interface");
    let command = Command::create(
        Uuid::new_v4(),
        "a command".into(),
        "".into(),
        Schema::default(),
    )
    .expect("failed to create command");
    let event = Event::create(
        Uuid::new_v4(),
        "an event".into(),
        "".into(),
        Schema::default(),
    )
    .expect("failed to create event");
    let read_model = ReadModel::create(
        Uuid::new_v4(),
        "a read model".into(),
        "".into(),
        Schema::default(),
    )
    .expect("failed to create event");
    model.component_defined(Component::InterfaceComponent(interface.to_owned()));
    model.component_defined(Component::CommandComponent(command.to_owned()));
    model.component_defined(Component::EventComponent(event.to_owned()));
    model.component_defined(Component::ReadModelComponent(read_model.to_owned()));
    assert_eq!(model.interfaces().get(interface.id()), Some(&interface));
    assert_eq!(model.commands().get(command.id()), Some(&command));
    assert_eq!(model.events().get(event.id()), Some(&event));
    assert_eq!(model.read_models().get(read_model.id()), Some(&read_model));
}

#[test]
fn renaming_components() {
    let id = Uuid::new_v4();
    let interface = Interface::create(
        Uuid::new_v4(),
        "an interface".into(),
        "".into(),
        InterfaceConfig::default(),
    )
    .expect("failed to create interface");
    let command = Command::create(
        Uuid::new_v4(),
        "a command".into(),
        "".into(),
        Schema::default(),
    )
    .expect("failed to create command");
    let event = Event::create(
        Uuid::new_v4(),
        "an event".into(),
        "".into(),
        Schema::default(),
    )
    .expect("failed to create event");
    let read_model = ReadModel::create(
        Uuid::new_v4(),
        "a read model".into(),
        "".into(),
        Schema::default(),
    )
    .expect("failed to create event");

    let mut model = InMemoryEventModel::new(id, "foo".to_string());
    model.component_defined(Component::InterfaceComponent(interface.to_owned()));
    model.component_defined(Component::CommandComponent(command.to_owned()));
    model.component_defined(Component::EventComponent(event.to_owned()));
    model.component_defined(Component::ReadModelComponent(read_model.to_owned()));

    let new_interface_name = "interface foo";
    let new_command_name = "command foo";
    let new_event_name = "event foo";
    let new_read_model_name = "read model foo";

    model.component_renamed(
        &InterfaceComponentId(interface.id().to_owned()),
        new_interface_name,
    );
    model.component_renamed(
        &CommandComponentId(command.id().to_owned()),
        new_command_name,
    );
    model.component_renamed(&EventComponentId(event.id().to_owned()), new_event_name);
    model.component_renamed(
        &ReadModelComponentId(read_model.id().to_owned()),
        new_read_model_name,
    );
    assert_eq!(
        model.interfaces().get(interface.id()).unwrap().name(),
        new_interface_name
    );
    assert_eq!(
        model.commands().get(command.id()).unwrap().name(),
        new_command_name
    );
    assert_eq!(
        model.events().get(event.id()).unwrap().name(),
        new_event_name
    );
    assert_eq!(
        model.read_models().get(read_model.id()).unwrap().name(),
        new_read_model_name
    );
}

#[test]
fn adding_to_component_description() {
    let id = Uuid::new_v4();
    let interface = Interface::create(
        Uuid::new_v4(),
        "an interface".into(),
        "".into(),
        InterfaceConfig::default(),
    )
    .expect("failed to create interface");
    let command = Command::create(
        Uuid::new_v4(),
        "a command".into(),
        "".into(),
        Schema::default(),
    )
    .expect("failed to create command");
    let event = Event::create(
        Uuid::new_v4(),
        "an event".into(),
        "".into(),
        Schema::default(),
    )
    .expect("failed to create event");
    let read_model = ReadModel::create(
        Uuid::new_v4(),
        "a read model".into(),
        "".into(),
        Schema::default(),
    )
    .expect("failed to create event");

    let mut model = InMemoryEventModel::new(id, "foo".to_string());
    model.component_defined(Component::InterfaceComponent(interface.to_owned()));
    model.component_defined(Component::CommandComponent(command.to_owned()));
    model.component_defined(Component::EventComponent(event.to_owned()));
    model.component_defined(Component::ReadModelComponent(read_model.to_owned()));

    model.added_to_component_description(
        &InterfaceComponentId(*interface.id()),
        0,
        "a really nice interface",
    );
    model.added_to_component_description(
        &CommandComponentId(*command.id()),
        0,
        "a really nice command",
    );
    model.added_to_component_description(&EventComponentId(*event.id()), 0, "a really nice event");
    model.added_to_component_description(
        &ReadModelComponentId(*read_model.id()),
        0,
        "a really nice read model",
    );
    assert_eq!(
        model
            .interfaces()
            .get(interface.id())
            .unwrap()
            .description(),
        "a really nice interface"
    );
    assert_eq!(
        model.commands().get(command.id()).unwrap().description(),
        "a really nice command"
    );
    assert_eq!(
        model.events().get(event.id()).unwrap().description(),
        "a really nice event"
    );
    assert_eq!(
        model
            .read_models()
            .get(read_model.id())
            .unwrap()
            .description(),
        "a really nice read model"
    );
}

#[test]
fn deleting_from_component_description() {
    let id = Uuid::new_v4();
    let interface = Interface::create(
        Uuid::new_v4(),
        "an interface".into(),
        "".into(),
        InterfaceConfig::default(),
    )
    .expect("failed to create interface");
    let command = Command::create(
        Uuid::new_v4(),
        "a command".into(),
        "".into(),
        Schema::default(),
    )
    .expect("failed to create command");
    let event = Event::create(
        Uuid::new_v4(),
        "an event".into(),
        "".into(),
        Schema::default(),
    )
    .expect("failed to create event");
    let read_model = ReadModel::create(
        Uuid::new_v4(),
        "a read model".into(),
        "".into(),
        Schema::default(),
    )
    .expect("failed to create event");

    let mut model = InMemoryEventModel::new(id, "foo".to_string());
    model.component_defined(Component::InterfaceComponent(interface.to_owned()));
    model.component_defined(Component::CommandComponent(command.to_owned()));
    model.component_defined(Component::EventComponent(event.to_owned()));
    model.component_defined(Component::ReadModelComponent(read_model.to_owned()));

    model.added_to_component_description(
        &InterfaceComponentId(*interface.id()),
        0,
        "a really nice interface",
    );
    model.added_to_component_description(
        &CommandComponentId(*command.id()),
        0,
        "a really nice command",
    );
    model.added_to_component_description(&EventComponentId(*event.id()), 0, "a really nice event");
    model.added_to_component_description(
        &ReadModelComponentId(*read_model.id()),
        0,
        "a really nice read model",
    );
    model.deleted_from_component_description(&InterfaceComponentId(*interface.id()), 2);
    model.deleted_from_component_description(&CommandComponentId(*command.id()), 2);
    model.deleted_from_component_description(&EventComponentId(*event.id()), 2);
    model.deleted_from_component_description(&ReadModelComponentId(*read_model.id()), 2);

    assert_eq!(
        model
            .interfaces()
            .get(interface.id())
            .unwrap()
            .description(),
        "a eally nice interface"
    );
    assert_eq!(
        model.commands().get(command.id()).unwrap().description(),
        "a eally nice command"
    );
    assert_eq!(
        model.events().get(event.id()).unwrap().description(),
        "a eally nice event"
    );
    assert_eq!(
        model
            .read_models()
            .get(read_model.id())
            .unwrap()
            .description(),
        "a eally nice read model"
    );
}
