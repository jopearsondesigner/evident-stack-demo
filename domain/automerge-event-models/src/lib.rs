use std::{borrow::Cow, collections::HashMap};

use crate::read_only::ReadOnlyEventModel;
use automerge::{
    transaction::Transactable, AutoCommit, ObjType, ReadDoc, ScalarValue, Value, ROOT,
};
use event_models::{
    types::{
        schema::{HasModifiableSchema, HasSchema},
        Audience, Command, CommandId, Component, ComponentId, ComponentMut, Described, Entity,
        Event, EventId, FlowArrow, FlowId, Interface, InterfaceId, Lane, LaneId, LaneIndex,
        ModifiablyDescribed, Named, Placement, PlacementId, PlacementPosition, ReadModel,
        ReadModelId, Renamable, Schema, Stream,
    },
    EventModel, EventModelData, EventModelId, EventModelState, ModifiableEventModel,
};
use uuid::Uuid;

mod read_only;

#[derive(Debug, Clone)]
pub struct AutomergeEventModel {
    crdt: AutoCommit,
    value: ReadOnlyEventModel,
}

// Top-level and re-used keys
static ID: &str = "id";
static NAME: &str = "name";
static DESCRIPTION: &str = "description";
static SCHEMA: &str = "schema";
static INTERFACES: &str = "interfaces";
static COMMANDS: &str = "commands";
static EVENTS: &str = "events";
static READ_MODELS: &str = "read_models";
static AUDIENCES: &str = "audiences";
static STREAMS: &str = "streams";
static PLACEMENTS: &str = "placements";
static FLOWS: &str = "flows";

static EMPTY_STR: &str = "";

// Component and Placement keys
static CONFIG: &str = "config";
static TYPE: &str = "type";
static URL: &str = "url";
static WIDTH: &str = "width";
static HEIGHT: &str = "height";

static INDEX: &str = "index";
static INTERFACE: &str = "interface";
static COMMAND: &str = "command";
static EVENT: &str = "event";
static READ_MODEL: &str = "read_model";
static AUDIENCE: &str = "audience";
static STREAM: &str = "stream";

// Flows
static FROM_PLACEMENT: &str = "from_placement";
static FROM_ANCHOR: &str = "from_anchor";
static TO_PLACEMENT: &str = "to_placement";
static TO_ANCHOR: &str = "to_anchor";

impl AutomergeEventModel {
    pub fn new(id: EventModelId, name: String) -> Self {
        let mut crdt = AutoCommit::new();
        crdt.put(ROOT, ID, id.to_string()).unwrap();
        crdt.put(ROOT, NAME, name).unwrap();
        crdt.put_object(ROOT, DESCRIPTION, ObjType::Text).unwrap();
        crdt.put_object(ROOT, SCHEMA, ObjType::Text).unwrap();
        crdt.put_object(ROOT, INTERFACES, ObjType::Map).unwrap();
        crdt.put_object(ROOT, COMMANDS, ObjType::Map).unwrap();
        crdt.put_object(ROOT, EVENTS, ObjType::Map).unwrap();
        crdt.put_object(ROOT, READ_MODELS, ObjType::Map).unwrap();
        crdt.put_object(ROOT, AUDIENCES, ObjType::List).unwrap();
        crdt.put_object(ROOT, STREAMS, ObjType::List).unwrap();
        crdt.put_object(ROOT, PLACEMENTS, ObjType::Map).unwrap();
        crdt.put_object(ROOT, FLOWS, ObjType::Map).unwrap();
        let value: ReadOnlyEventModel = crdt.to_owned().into();
        AutomergeEventModel { crdt, value }
    }

    pub fn load(data: &[u8]) -> Self {
        let crdt = AutoCommit::load(data).unwrap();
        let value: ReadOnlyEventModel = crdt.to_owned().into();
        AutomergeEventModel { crdt, value }
    }

    pub fn load_incremental(&mut self, data: &[u8]) {
        self.crdt.load_incremental(data).unwrap();
        self.value = self.crdt.to_owned().into();
    }
}

impl Entity for AutomergeEventModel {
    fn id(&self) -> &Uuid {
        self.value.id()
    }
}

impl Named for AutomergeEventModel {
    fn name(&self) -> &str {
        &self.value.name
    }
}

impl Described for AutomergeEventModel {
    fn description(&self) -> &str {
        &self.value.description
    }
}

impl HasSchema for AutomergeEventModel {
    fn schema(&self) -> &Schema {
        &self.value.schema
    }
}

impl EventModel for AutomergeEventModel {
    type CreationDetails = (); // TODO: User ID?

    fn create(initial: EventModelState<Self>, id: EventModelId, name: String) -> Self {
        match initial {
            EventModelState::BeforeCreation(_) => AutomergeEventModel::new(id, name),
            _ => panic!("Illegal state when creating Automerge Event Model!"),
        }
    }
}

impl EventModelData for AutomergeEventModel {
    fn interfaces(&self) -> &HashMap<InterfaceId, Interface> {
        &self.value.interfaces
    }

    fn commands(&self) -> &HashMap<CommandId, Command> {
        &self.value.commands
    }

    fn events(&self) -> &HashMap<EventId, Event> {
        &self.value.events
    }

    fn read_models(&self) -> &HashMap<ReadModelId, ReadModel> {
        &self.value.read_models
    }

    fn audiences(&self) -> &Vec<Audience> {
        &self.value.audiences
    }

    fn streams(&self) -> &Vec<Stream> {
        &self.value.streams
    }

    fn placements(&self) -> &HashMap<PlacementId, Placement> {
        &self.value.placements
    }

    fn flows(&self) -> &HashMap<FlowId, FlowArrow> {
        &self.value.flows
    }
}

impl Renamable for AutomergeEventModel {
    fn rename(&mut self, name: &str) {
        self.crdt.put(ROOT, NAME, name).unwrap();
        self.value.name = name.to_string();
    }
}

impl ModifiablyDescribed for AutomergeEventModel {
    // TODO: refactor this out of trait and into InMemoryEventModel
    fn description_mut(&mut self) -> &mut String {
        todo!()
    }

    fn set_description(&mut self, description: &str) {
        let Ok(Some((Value::Object(ObjType::Text), obj_id))) = self.crdt.get(ROOT, DESCRIPTION) else {
            panic!("No description Text found");
        };
        let old_description = self.crdt.text(&obj_id).unwrap();
        self.crdt
            .splice_text(&obj_id, 0, old_description.len(), description)
            .unwrap();
        self.value.description = self.crdt.text(&obj_id).unwrap();
    }

    fn add_to_description(&mut self, index: usize, addition: &str) {
        let Ok(Some((Value::Object(ObjType::Text), obj_id))) = self.crdt.get(ROOT, DESCRIPTION) else {
            panic!("No description Text found");
        };
        self.crdt.splice_text(&obj_id, index, 0, addition).unwrap();
        self.value.description = self.crdt.text(&obj_id).unwrap();
    }

    fn delete_from_description(&mut self, index: usize, count: usize) {
        let Ok(Some((Value::Object(ObjType::Text), obj_id))) = self.crdt.get(ROOT, DESCRIPTION) else {
            panic!("No description Text found");
        };
        self.crdt
            .splice_text(&obj_id, index, count, EMPTY_STR)
            .unwrap();
        self.value.description = self.crdt.text(&obj_id).unwrap();
    }
}

impl HasModifiableSchema for AutomergeEventModel {
    // TODO: refactor this out of trait and into InMemoryEventModel
    fn schema_mut(&mut self) -> &mut Schema {
        todo!()
    }

    fn set_schema(&mut self, schema: Schema) {
        let Ok(Some((Value::Object(ObjType::Text), obj_id))) = self.crdt.get(ROOT, SCHEMA) else {
            panic!("No schema Text found");
        };
        let old_schema = self.crdt.text(&obj_id).unwrap();
        let Schema(s) = schema;
        self.crdt
            .splice_text(&obj_id, 0, old_schema.len(), &s)
            .unwrap();
        self.value.schema = Schema(self.crdt.text(&obj_id).unwrap());
    }

    fn add_to_schema(&mut self, index: usize, addition: &str) {
        let Ok(Some((Value::Object(ObjType::Text), obj_id))) = self.crdt.get(ROOT, SCHEMA) else {
            panic!("No schema Text found");
        };
        self.crdt.splice_text(&obj_id, index, 0, addition).unwrap();
        self.value.schema = Schema(self.crdt.text(&obj_id).unwrap());
    }

    fn delete_from_schema(&mut self, index: usize, count: usize) {
        let Ok(Some((Value::Object(ObjType::Text), obj_id))) = self.crdt.get(ROOT, SCHEMA) else {
            panic!("No schema Text found");
        };
        self.crdt
            .splice_text(&obj_id, index, count, EMPTY_STR)
            .unwrap();
        self.value.schema = Schema(self.crdt.text(&obj_id).unwrap());
    }
}

impl ModifiableEventModel for AutomergeEventModel {
    fn component_defined(&mut self, component: Component) {
        match component {
            Component::Interface(i) => {
                let Ok(Some((Value::Object(ObjType::Map), interfaces))) =
                    self.crdt.get(ROOT, INTERFACES) else {
                        panic!("No interfaces Map found");
                    };
                let interface_doc = self
                    .crdt
                    .put_object(interfaces, i.id().to_string(), ObjType::Map)
                    .unwrap();
                self.crdt
                    .put(&interface_doc, ID, i.id().to_string())
                    .unwrap();
                self.crdt.put(&interface_doc, NAME, i.name()).unwrap();
                let description = self
                    .crdt
                    .put_object(&interface_doc, DESCRIPTION, ObjType::Text)
                    .unwrap();
                self.crdt
                    .splice_text(&description, 0, 0, i.description())
                    .unwrap();
                let config = self
                    .crdt
                    .put_object(&interface_doc, CONFIG, ObjType::Map)
                    .unwrap();
                match i.config() {
                    event_models::types::InterfaceConfig::Blank => {
                        self.crdt.put(&config, TYPE, "Blank").unwrap();
                    }
                    event_models::types::InterfaceConfig::Figma { url, width, height } => {
                        self.crdt.put(&config, TYPE, "Figma").unwrap();
                        self.crdt.put(&config, URL, url.to_string()).unwrap();
                        if let Some(w) = width {
                            self.crdt.put(&config, WIDTH, *w as u32).unwrap();
                        }
                        if let Some(h) = height {
                            self.crdt.put(&config, HEIGHT, *h as u32).unwrap();
                        }
                    }
                    event_models::types::InterfaceConfig::Image { url, width, height } => {
                        self.crdt.put(&config, TYPE, "Image").unwrap();
                        self.crdt.put(&config, URL, url.to_string()).unwrap();
                        if let Some(w) = width {
                            self.crdt.put(&config, WIDTH, *w as u32).unwrap();
                        }
                        if let Some(h) = height {
                            self.crdt.put(&config, HEIGHT, *h as u32).unwrap();
                        }
                    }
                    event_models::types::InterfaceConfig::Job => {
                        self.crdt.put(&config, TYPE, "Job").unwrap();
                    }
                }
                self.value.interfaces.insert(*i.id(), i.to_owned());
            }
            Component::Command(c) => {
                let Ok(Some((Value::Object(ObjType::Map), commands))) =
                    self.crdt.get(ROOT, COMMANDS) else {
                        panic!("No commands Map found");
                    };
                let command_doc = self
                    .crdt
                    .put_object(commands, c.id().to_string(), ObjType::Map)
                    .unwrap();
                self.crdt.put(&command_doc, ID, c.id().to_string()).unwrap();
                self.crdt.put(&command_doc, NAME, c.name()).unwrap();
                let description = self
                    .crdt
                    .put_object(&command_doc, DESCRIPTION, ObjType::Text)
                    .unwrap();
                self.crdt
                    .splice_text(&description, 0, 0, c.description())
                    .unwrap();
                let schema = self
                    .crdt
                    .put_object(&command_doc, SCHEMA, ObjType::Text)
                    .unwrap();
                self.crdt.splice_text(&schema, 0, 0, &c.schema().0).unwrap();
                self.value.commands.insert(*c.id(), c.to_owned());
            }
            Component::Event(e) => {
                let Ok(Some((Value::Object(ObjType::Map), events))) =
                    self.crdt.get(ROOT, EVENTS) else {
                        panic!("No events Map found");
                    };
                let event_doc = self
                    .crdt
                    .put_object(events, e.id().to_string(), ObjType::Map)
                    .unwrap();
                self.crdt.put(&event_doc, ID, e.id().to_string()).unwrap();
                self.crdt.put(&event_doc, NAME, e.name()).unwrap();
                let description = self
                    .crdt
                    .put_object(&event_doc, DESCRIPTION, ObjType::Text)
                    .unwrap();
                self.crdt
                    .splice_text(&description, 0, 0, e.description())
                    .unwrap();
                let schema = self
                    .crdt
                    .put_object(&event_doc, SCHEMA, ObjType::Text)
                    .unwrap();
                self.crdt.splice_text(&schema, 0, 0, &e.schema().0).unwrap();
                self.value.events.insert(*e.id(), e.to_owned());
            }
            Component::ReadModel(r) => {
                let Ok(Some((Value::Object(ObjType::Map), read_models))) =
                    self.crdt.get(ROOT, READ_MODELS) else {
                        panic!("No read models Map found");
                    };
                let read_model_doc = self
                    .crdt
                    .put_object(read_models, r.id().to_string(), ObjType::Map)
                    .unwrap();
                self.crdt
                    .put(&read_model_doc, ID, r.id().to_string())
                    .unwrap();
                self.crdt.put(&read_model_doc, NAME, r.name()).unwrap();
                let description = self
                    .crdt
                    .put_object(&read_model_doc, DESCRIPTION, ObjType::Text)
                    .unwrap();
                self.crdt
                    .splice_text(&description, 0, 0, r.description())
                    .unwrap();
                let schema = self
                    .crdt
                    .put_object(&read_model_doc, SCHEMA, ObjType::Text)
                    .unwrap();
                self.crdt.splice_text(&schema, 0, 0, &r.schema().0).unwrap();
                self.value.read_models.insert(*r.id(), r.to_owned());
            }
        }
    }

    fn component_renamed(&mut self, component_id: &ComponentId, name: &str) {
        match self.value.component_mut_by_id(component_id) {
            Some(ComponentMut::Interface(i)) => {
                let Ok(Some((Value::Object(ObjType::Map), interfaces))) =
                    self.crdt.get(ROOT, INTERFACES) else {
                        panic!("No interfaces Map found");
                    };

                let Ok(Some((Value::Object(ObjType::Map), interface))) =
                    self.crdt.get(interfaces, i.id().to_string()) else {
                        panic!("No interface with ID={:?} found", i.id());
                    };

                self.crdt.put(interface, NAME, name).unwrap();
                i.rename(name);
            }
            Some(ComponentMut::Command(c)) => {
                let Ok(Some((Value::Object(ObjType::Map), commands))) =
                    self.crdt.get(ROOT, COMMANDS) else {
                        panic!("No commands Map found");
                    };

                let Ok(Some((Value::Object(ObjType::Map), command))) =
                    self.crdt.get(commands, c.id().to_string()) else {
                        panic!("No command with ID={:?} found", c.id());
                    };

                self.crdt.put(command, NAME, name).unwrap();
                c.rename(name);
            }
            Some(ComponentMut::Event(e)) => {
                let Ok(Some((Value::Object(ObjType::Map), events))) =
                    self.crdt.get(ROOT, EVENTS) else {
                        panic!("No events Map found");
                    };

                let Ok(Some((Value::Object(ObjType::Map), event))) =
                    self.crdt.get(events, e.id().to_string()) else {
                        panic!("No event with ID={:?} found", e.id());
                    };

                self.crdt.put(event, NAME, name).unwrap();
                e.rename(name);
            }
            Some(ComponentMut::ReadModel(r)) => {
                let Ok(Some((Value::Object(ObjType::Map), read_models))) =
                    self.crdt.get(ROOT, READ_MODELS) else {
                        panic!("No read models Map found");
                    };

                let Ok(Some((Value::Object(ObjType::Map), read_model))) =
                    self.crdt.get(read_models, r.id().to_string()) else {
                        panic!("No read model with ID={:?} found", r.id());
                    };

                self.crdt.put(read_model, NAME, name).unwrap();
                r.rename(name);
            }
            None => panic!("Component with id {:?} not found", component_id),
        }
    }

    fn component_removed(&mut self, component_id: &ComponentId) {
        match component_id {
            ComponentId::InterfaceComponentId(id) => {
                let Ok(Some((Value::Object(ObjType::Map), interfaces))) =
                    self.crdt.get(ROOT, INTERFACES) else {
                        panic!("No interfaces Map found");
                    };
                self.crdt.delete(interfaces, id.to_string()).unwrap();
                self.value.interfaces.remove(id);
            }
            ComponentId::CommandComponentId(id) => {
                let Ok(Some((Value::Object(ObjType::Map), commands))) =
                    self.crdt.get(ROOT, COMMANDS) else {
                        panic!("No commands Map found");
                    };
                self.crdt.delete(commands, id.to_string()).unwrap();
                self.value.commands.remove(id);
            }
            ComponentId::EventComponentId(id) => {
                let Ok(Some((Value::Object(ObjType::Map), events))) =
                    self.crdt.get(ROOT, EVENTS) else {
                        panic!("No events Map found");
                    };
                self.crdt.delete(events, id.to_string()).unwrap();
                self.value.events.remove(id);
            }
            ComponentId::ReadModelComponentId(id) => {
                let Ok(Some((Value::Object(ObjType::Map), read_models))) =
                    self.crdt.get(ROOT, READ_MODELS) else {
                        panic!("No read models Map found");
                    };
                self.crdt.delete(read_models, id.to_string()).unwrap();
                self.value.read_models.remove(id);
            }
        }
    }

    fn added_to_component_description(
        &mut self,
        component_id: &ComponentId,
        index: usize,
        addition: &str,
    ) {
        match self.value.component_mut_by_id(component_id) {
            Some(ComponentMut::Interface(i)) => todo!(),
            Some(ComponentMut::Command(c)) => todo!(),
            Some(ComponentMut::Event(e)) => todo!(),
            Some(ComponentMut::ReadModel(r)) => todo!(),
            None => todo!(),
        }
    }

    fn deleted_from_component_description(
        &mut self,
        component_id: &ComponentId,
        index: usize,
        count: usize,
    ) {
        match self.value.component_mut_by_id(component_id) {
            Some(ComponentMut::Interface(i)) => todo!(),
            Some(ComponentMut::Command(c)) => todo!(),
            Some(ComponentMut::Event(e)) => todo!(),
            Some(ComponentMut::ReadModel(r)) => todo!(),
            None => todo!(),
        }
    }

    fn added_to_component_schema(
        &mut self,
        component_id: &ComponentId,
        index: usize,
        addition: &str,
    ) {
        match self.value.component_mut_by_id(component_id) {
            Some(ComponentMut::Interface(i)) => todo!(),
            Some(ComponentMut::Command(c)) => todo!(),
            Some(ComponentMut::Event(e)) => todo!(),
            Some(ComponentMut::ReadModel(r)) => todo!(),
            None => todo!(),
        }
    }

    fn deleted_from_component_schema(
        &mut self,
        component_id: &ComponentId,
        index: usize,
        count: usize,
    ) {
        match self.value.component_mut_by_id(component_id) {
            Some(ComponentMut::Interface(i)) => todo!(),
            Some(ComponentMut::Command(c)) => todo!(),
            Some(ComponentMut::Event(e)) => todo!(),
            Some(ComponentMut::ReadModel(r)) => todo!(),
            None => todo!(),
        }
    }

    fn component_placed(&mut self, placement: &Placement) {
        let Ok(Some((Value::Object(ObjType::Map), obj_id))) = self.crdt.get(ROOT, PLACEMENTS) else { todo!() };
        let placement_doc = self
            .crdt
            .put_object(&obj_id, placement.id().to_string(), ObjType::Map)
            .unwrap();
        match placement {
            Placement::Interface {
                id,
                index,
                interface,
                audience,
            } => {
                self.crdt.put(&placement_doc, ID, id.to_string()).unwrap();
                self.crdt.put(&placement_doc, INDEX, *index as u32).unwrap();
                self.crdt
                    .put(&placement_doc, INTERFACE, interface.to_string())
                    .unwrap();
                if let Some(a) = audience {
                    self.crdt
                        .put(&placement_doc, AUDIENCE, a.to_string())
                        .unwrap();
                }
            }
            Placement::Command {
                id,
                index,
                command,
                schema,
            } => {
                self.crdt.put(&placement_doc, ID, id.to_string()).unwrap();
                self.crdt.put(&placement_doc, INDEX, *index as u32).unwrap();
                self.crdt
                    .put(&placement_doc, COMMAND, command.to_string())
                    .unwrap();
                self.crdt
                    .put(&placement_doc, COMMAND, command.to_string())
                    .unwrap();
                let schema_doc = self
                    .crdt
                    .put_object(&placement_doc, SCHEMA, ObjType::Text)
                    .unwrap();
                self.crdt.splice_text(&schema_doc, 0, 0, &schema.0).unwrap();
            }
            Placement::Event {
                id,
                index,
                event,
                stream,
                schema,
            } => {
                self.crdt.put(&placement_doc, ID, id.to_string()).unwrap();
                self.crdt.put(&placement_doc, INDEX, *index as u32).unwrap();
                self.crdt
                    .put(&placement_doc, EVENT, event.to_string())
                    .unwrap();

                let schema_doc = self
                    .crdt
                    .put_object(&placement_doc, SCHEMA, ObjType::Text)
                    .unwrap();
                self.crdt.splice_text(&schema_doc, 0, 0, &schema.0).unwrap();

                if let Some(a) = stream {
                    self.crdt
                        .put(&placement_doc, STREAM, a.to_string())
                        .unwrap();
                }
            }
            Placement::ReadModel {
                id,
                index,
                read_model,
                schema,
            } => {
                self.crdt.put(&placement_doc, ID, id.to_string()).unwrap();
                self.crdt.put(&placement_doc, INDEX, *index as u32).unwrap();
                self.crdt
                    .put(&placement_doc, READ_MODEL, read_model.to_string())
                    .unwrap();

                let schema_doc = self
                    .crdt
                    .put_object(&placement_doc, SCHEMA, ObjType::Text)
                    .unwrap();
                self.crdt.splice_text(&schema_doc, 0, 0, &schema.0).unwrap();
            }
        }

        self.value
            .placements
            .insert(placement.id().to_owned(), placement.to_owned());
    }

    fn placement_moved(&mut self, position: &PlacementPosition) {
        if let Some(ref mut placement) = self.value.placements.get_mut(position.id()) {
            let PlacementPosition(_, index, lane) = position;
            let Ok(Some((Value::Object(ObjType::Map), placements))) =
                self.crdt.get(ROOT, PLACEMENTS) else { todo!() };
            let Ok(Some((Value::Object(ObjType::Map), placement_doc))) =
                self.crdt.get(placements, placement.id().to_string()) else { todo!() };
            self.crdt.put(&placement_doc, INDEX, *index as u32).unwrap();
            match lane {
                LaneId::Audience(id) => self
                    .crdt
                    .put(&placement_doc, AUDIENCE, id.to_string())
                    .unwrap(),
                LaneId::Stream(id) => self
                    .crdt
                    .put(&placement_doc, STREAM, id.to_string())
                    .unwrap(),
                _ => (),
            }
            placement.relocate(index.to_owned(), lane.to_owned());
        };
    }

    fn placement_removed(&mut self, placement_id: &PlacementId) {
        let Ok(Some((Value::Object(ObjType::Map), placements))) =
            self.crdt.get(ROOT, PLACEMENTS) else { todo!() };
        self.crdt
            .delete(placements, placement_id.to_string())
            .unwrap();
        self.value.placements.remove(placement_id);
    }

    fn placements_shifted(&mut self, offset: &usize, width: &usize) {
        let Ok(Some((Value::Object(ObjType::Map), placements))) =
            self.crdt.get(ROOT, PLACEMENTS) else { todo!() };
        self.value.placements.iter_mut().for_each(|(_, placement)| {
            if placement.index() >= offset {
                let Ok(Some((Value::Object(ObjType::Map), placement_doc))) =
                    self.crdt.get(&placements, placement.id().to_string()) else { todo!() };
                let Ok(Some((Value::Scalar(Cow::Owned(ScalarValue::Uint(old_index))), _index_op_id))) =
                    self.crdt.get(&placements, placement.id().to_string()) else { todo!() };
                // TODO: re-implement in terms of looking up existing
                // index in placement, shifting, and then setting in
                // CRDT.  Current impl leaves room for commutativity error?
                self.crdt.put(placement_doc, INDEX, old_index as u32 + *width as u32).unwrap();
                placement.shift_right(*width);
            }
        })
    }

    fn added_to_placement_schema(
        &mut self,
        placement_id: &PlacementId,
        index: usize,
        addition: &str,
    ) {
        todo!()
    }

    fn deleted_from_placement_schema(
        &mut self,
        placement_id: &PlacementId,
        index: usize,
        count: usize,
    ) {
        todo!()
    }

    fn lane_added(&mut self, lane: Lane, index: LaneIndex) {
        match lane {
            Lane::Audience(audience) => {
                let Ok(Some((Value::Object(ObjType::List), audiences))) =
                    self.crdt.get(ROOT, AUDIENCES) else { todo!() };
                let audience_doc = self
                    .crdt
                    .insert_object(audiences, index, ObjType::Map)
                    .unwrap();
                self.crdt
                    .put(&audience_doc, ID, audience.id().to_string())
                    .unwrap();
                self.crdt.put(&audience_doc, NAME, audience.name()).unwrap();
                self.value.audiences.insert(index, audience);
            }
            Lane::Stream(stream) => {
                let Ok(Some((Value::Object(ObjType::List), streams))) =
                    self.crdt.get(ROOT, STREAMS) else { todo!() };
                let stream_doc = self
                    .crdt
                    .insert_object(streams, index, ObjType::Map)
                    .unwrap();
                self.crdt
                    .put(&stream_doc, ID, stream.id().to_string())
                    .unwrap();
                self.crdt.put(&stream_doc, NAME, stream.name()).unwrap();
                self.value.streams.insert(index, stream);
            }
        }
    }

    fn lane_renamed(&mut self, lane_id: LaneId, name: &str) {
        todo!()
    }

    fn lane_reordered(&mut self, lane_id: LaneId, index: LaneIndex) {
        todo!()
    }

    fn lane_removed(&mut self, lane_id: LaneId) {
        todo!()
    }

    fn plus_flow(&mut self, flow_arrow: FlowArrow) {
        let Ok(Some((Value::Object(ObjType::Map), flows))) =
            self.crdt.get(ROOT, FLOWS) else { todo!() };
        let flow_doc = self
            .crdt
            .put_object(flows, flow_arrow.id().to_string(), ObjType::Map)
            .unwrap();
        self.crdt
            .put(
                &flow_doc,
                FROM_PLACEMENT,
                flow_arrow.from().placement_id().to_string(),
            )
            .unwrap();
        self.crdt
            .put(
                &flow_doc,
                FROM_ANCHOR,
                flow_arrow.from().anchor().to_string(),
            )
            .unwrap();
        self.crdt
            .put(
                &flow_doc,
                TO_PLACEMENT,
                flow_arrow.to().placement_id().to_string(),
            )
            .unwrap();
        self.crdt
            .put(&flow_doc, TO_ANCHOR, flow_arrow.to().anchor().to_string())
            .unwrap();
        self.value
            .flows
            .insert(flow_arrow.id().to_owned(), flow_arrow);
    }

    fn minus_flow(&mut self, flow_id: &FlowId) {
        todo!()
    }
}

fn 

impl From<AutoCommit> for ReadOnlyEventModel {
    fn from(_: AutoCommit) -> Self {
        todo!()
    }
}
