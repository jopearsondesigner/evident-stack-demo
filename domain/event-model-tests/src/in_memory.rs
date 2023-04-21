use crate::shared::{creating_event_model_succeeds, renaming_event_model_succeeds};
use event_models::implementation::in_memory::InMemoryEventModel;
use event_models::{EventModelData, ModifiableEventModel};
use event_models::{EventModelState, Name};
use uuid::Uuid;

use event_models::ComponentId::{
    CommandComponentId, EventComponentId, InterfaceComponentId, ReadModelComponentId,
};
use event_models::{
    Command, Component, Described, Entity, Event, Interface, InterfaceConfig, Named, ReadModel,
};

#[test]
fn creation() {
    creating_event_model_succeeds(EventModelState::<InMemoryEventModel>::BeforeCreation);
}

#[test]
fn renaming() {
    renaming_event_model_succeeds(EventModelState::<InMemoryEventModel>::EventModel(
        InMemoryEventModel::new(&Uuid::new_v4(), &"foo".try_into().unwrap()),
    ));
}

#[test]
fn adding_to_description() {
    let id = Uuid::new_v4();
    let mut model = InMemoryEventModel::new(&id, &"foo".try_into().unwrap());
    assert_eq!(model.description(), "");
    model.splice_description(0, 0, "foo bar");
    assert_eq!(model.description(), "foo bar");
}

#[test]
fn deleting_from_description() {
    let id = Uuid::new_v4();
    let mut model = InMemoryEventModel::new(&id, &"foo".try_into().unwrap());
    model.splice_description(0, 0, "foo bar");
    model.splice_description(2, 1, "");
    assert_eq!(model.description(), "fo bar");
}

#[test]
fn defining_components() {
    let id = Uuid::new_v4();
    let mut model = InMemoryEventModel::new(&id, &"foo".try_into().unwrap());

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
        Default::default(),
    )
    .expect("failed to create command");
    let event = Event::create(
        Uuid::new_v4(),
        "an event".into(),
        "".into(),
        Default::default(),
    )
    .expect("failed to create event");
    let read_model = ReadModel::create(
        Uuid::new_v4(),
        "a read model".into(),
        "".into(),
        Default::default(),
    )
    .expect("failed to create event");
    model.component_defined(&Component::Interface(interface.to_owned()));
    model.component_defined(&Component::Command(command.to_owned()));
    model.component_defined(&Component::Event(event.to_owned()));
    model.component_defined(&Component::ReadModel(read_model.to_owned()));
    assert_eq!(model.interfaces().get(&interface.id()), Some(&interface));
    assert_eq!(model.commands().get(&command.id()), Some(&command));
    assert_eq!(model.events().get(&event.id()), Some(&event));
    assert_eq!(model.read_models().get(&read_model.id()), Some(&read_model));
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
        Default::default(),
    )
    .expect("failed to create command");
    let event = Event::create(
        Uuid::new_v4(),
        "an event".into(),
        "".into(),
        Default::default(),
    )
    .expect("failed to create event");
    let read_model = ReadModel::create(
        Uuid::new_v4(),
        "a read model".into(),
        "".into(),
        Default::default(),
    )
    .expect("failed to create event");

    let mut model = InMemoryEventModel::new(&id, &"foo".try_into().unwrap());
    model.component_defined(&Component::Interface(interface.to_owned()));
    model.component_defined(&Component::Command(command.to_owned()));
    model.component_defined(&Component::Event(event.to_owned()));
    model.component_defined(&Component::ReadModel(read_model.to_owned()));

    let new_interface_name: Name = "interface foo".try_into().unwrap();
    let new_command_name: Name = "command foo".try_into().unwrap();
    let new_event_name: Name = "event foo".try_into().unwrap();
    let new_read_model_name: Name = "read model foo".try_into().unwrap();

    model.component_renamed(
        &InterfaceComponentId(interface.id().to_owned()),
        &new_interface_name,
    );
    model.component_renamed(
        &CommandComponentId(command.id().to_owned()),
        &new_command_name,
    );
    model.component_renamed(&EventComponentId(event.id().to_owned()), &new_event_name);
    model.component_renamed(
        &ReadModelComponentId(read_model.id().to_owned()),
        &new_read_model_name,
    );
    assert_eq!(
        model.interfaces().get(&interface.id()).unwrap().name(),
        new_interface_name
    );
    assert_eq!(
        model.commands().get(&command.id()).unwrap().name(),
        new_command_name
    );
    assert_eq!(
        model.events().get(&event.id()).unwrap().name(),
        new_event_name
    );
    assert_eq!(
        model.read_models().get(&read_model.id()).unwrap().name(),
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
        Default::default(),
    )
    .expect("failed to create command");
    let event = Event::create(
        Uuid::new_v4(),
        "an event".into(),
        "".into(),
        Default::default(),
    )
    .expect("failed to create event");
    let read_model = ReadModel::create(
        Uuid::new_v4(),
        "a read model".into(),
        "".into(),
        Default::default(),
    )
    .expect("failed to create event");

    let mut model = InMemoryEventModel::new(&id, &"foo".try_into().unwrap());
    model.component_defined(&Component::Interface(interface.to_owned()));
    model.component_defined(&Component::Command(command.to_owned()));
    model.component_defined(&Component::Event(event.to_owned()));
    model.component_defined(&Component::ReadModel(read_model.to_owned()));

    model.splice_component_description(
        &InterfaceComponentId(interface.id()),
        0,
        0,
        "a really nice interface",
    );
    model.splice_component_description(
        &CommandComponentId(command.id()),
        0,
        0,
        "a really nice command",
    );
    model.splice_component_description(&EventComponentId(event.id()), 0, 0, "a really nice event");
    model.splice_component_description(
        &ReadModelComponentId(read_model.id()),
        0,
        0,
        "a really nice read model",
    );
    assert_eq!(
        model
            .interfaces()
            .get(&interface.id())
            .unwrap()
            .description(),
        "a really nice interface"
    );
    assert_eq!(
        model.commands().get(&command.id()).unwrap().description(),
        "a really nice command"
    );
    assert_eq!(
        model.events().get(&event.id()).unwrap().description(),
        "a really nice event"
    );
    assert_eq!(
        model
            .read_models()
            .get(&read_model.id())
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
        Default::default(),
    )
    .expect("failed to create command");
    let event = Event::create(
        Uuid::new_v4(),
        "an event".into(),
        "".into(),
        Default::default(),
    )
    .expect("failed to create event");
    let read_model = ReadModel::create(
        Uuid::new_v4(),
        "a read model".into(),
        "".into(),
        Default::default(),
    )
    .expect("failed to create event");

    let mut model = InMemoryEventModel::new(&id, &"foo".try_into().unwrap());
    model.component_defined(&Component::Interface(interface.to_owned()));
    model.component_defined(&Component::Command(command.to_owned()));
    model.component_defined(&Component::Event(event.to_owned()));
    model.component_defined(&Component::ReadModel(read_model.to_owned()));

    model.splice_component_description(
        &InterfaceComponentId(interface.id()),
        0,
        0,
        "a really nice interface",
    );
    model.splice_component_description(
        &CommandComponentId(command.id()),
        0,
        0,
        "a really nice command",
    );
    model.splice_component_description(&EventComponentId(event.id()), 0, 0, "a really nice event");
    model.splice_component_description(
        &ReadModelComponentId(read_model.id()),
        0,
        0,
        "a really nice read model",
    );
    model.splice_component_description(&InterfaceComponentId(interface.id()), 2, 1, "");
    model.splice_component_description(&CommandComponentId(command.id()), 2, 1, "");
    model.splice_component_description(&EventComponentId(event.id()), 2, 1, "");
    model.splice_component_description(&ReadModelComponentId(read_model.id()), 2, 1, "");

    assert_eq!(
        model
            .interfaces()
            .get(&interface.id())
            .unwrap()
            .description(),
        "a eally nice interface"
    );
    assert_eq!(
        model.commands().get(&command.id()).unwrap().description(),
        "a eally nice command"
    );
    assert_eq!(
        model.events().get(&event.id()).unwrap().description(),
        "a eally nice event"
    );
    assert_eq!(
        model
            .read_models()
            .get(&read_model.id())
            .unwrap()
            .description(),
        "a eally nice read model"
    );
}
