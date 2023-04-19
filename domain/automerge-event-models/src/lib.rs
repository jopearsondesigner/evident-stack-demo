use std::collections::HashMap;

use automerge::{AutoCommit, ROOT};
use autosurgeon::{hydrate, reconcile, reconcile_prop, Hydrate, Reconcile, Text};
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

#[derive(Reconcile, Hydrate, Debug)]
pub enum AutoInterfaceConfig {
    Blank,
    Figma {
        url: String,
        width: Option<u32>,
        height: Option<u32>,
    },
    Image {
        url: String,
        width: Option<u32>,
        height: Option<u32>,
    },
    Job,
}

#[derive(Reconcile, Hydrate, Debug)]
struct AutoInterface {
    #[key]
    id: Uuid,
    name: String,
    description: Text,
    config: AutoInterfaceConfig,
}

#[derive(Reconcile, Hydrate, Debug)]
struct AutoCommand {
    #[key]
    id: Uuid,
    name: String,
    description: Text,
    schema: Text,
}

#[derive(Reconcile, Hydrate, Debug)]
struct AutoEvent {
    #[key]
    id: Uuid,
    name: String,
    description: Text,
    schema: Text,
}

#[derive(Reconcile, Hydrate, Debug)]
struct AutoReadModel {
    #[key]
    id: Uuid,
    name: String,
    description: Text,
    schema: Text,
}

#[derive(Reconcile, Hydrate, Debug)]
pub struct AutoAudience {
    #[key]
    id: Uuid,
    name: String,
}

#[derive(Reconcile, Hydrate, Debug)]
pub struct AutoStream {
    #[key]
    id: Uuid,
    name: String,
}

#[derive(Reconcile, Hydrate, Debug)]
pub enum AutoPlacement {
    Interface {
        #[key]
        id: Uuid,
        index: u32,
        interface: Uuid,
        audience: Option<Uuid>,
    },
    Command {
        #[key]
        id: Uuid,
        index: u32,
        command: Uuid,
        schema: Text,
    },
    Event {
        #[key]
        id: Uuid,
        index: u32,
        event: Uuid,
        stream: Option<Uuid>,
        schema: Text,
    },
    ReadModel {
        #[key]
        id: Uuid,
        index: u32,
        read_model: Uuid,
        schema: Text,
    },
}

impl Entity for AutoPlacement {
    fn id(&self) -> &Uuid {
        match self {
            AutoPlacement::Interface { id, .. } => id,
            AutoPlacement::Command { id, .. } => id,
            AutoPlacement::Event { id, .. } => id,
            AutoPlacement::ReadModel { id, .. } => id,
        }
    }
}

#[derive(Reconcile, Hydrate, Debug)]
pub enum Anchor {
    None,
    Top,
    Left,
    Bottom,
    Right,
}

#[derive(Reconcile, Hydrate, Debug)]
pub struct AutoFlowArrow {
    #[key]
    id: Uuid,
    from_placement: Uuid,
    from_anchor: Anchor,
    to_placement: Uuid,
    to_anchor: Anchor,
}

#[derive(Reconcile, Hydrate, Debug)]
struct AutoSurgeonEventModel {
    #[key]
    id: Uuid,
    name: String,
    description: Text,
    schema: Text,
    interfaces: HashMap<String, AutoInterface>,
    commands: HashMap<String, AutoCommand>,
    events: HashMap<String, AutoEvent>,
    read_models: HashMap<String, AutoReadModel>,
    audiences: Vec<AutoAudience>,
    streams: Vec<AutoStream>,
    placements: HashMap<String, AutoPlacement>,
    flows: HashMap<String, AutoFlowArrow>,
}

impl AutoSurgeonEventModel {
    pub(crate) fn new(id: Uuid, name: String) -> Self {
        AutoSurgeonEventModel {
            id,
            name,
            description: Default::default(),
            schema: Default::default(),
            interfaces: Default::default(),
            commands: Default::default(),
            events: Default::default(),
            read_models: Default::default(),
            audiences: Default::default(),
            streams: Default::default(),
            placements: Default::default(),
            flows: Default::default(),
        }
    }
}

impl From<&AutoInterface> for Interface {
    fn from(interface: &AutoInterface) -> Self {
        todo!()
    }
}

impl From<&AutoCommand> for Command {
    fn from(command: &AutoCommand) -> Self {
        todo!()
    }
}

impl From<&AutoEvent> for Event {
    fn from(event: &AutoEvent) -> Self {
        todo!()
    }
}

impl From<&AutoReadModel> for ReadModel {
    fn from(read_model: &AutoReadModel) -> Self {
        todo!()
    }
}

impl From<AutoAudience> for Audience {
    fn from(audience: AutoAudience) -> Self {
        todo!()
    }
}

impl From<&AutoStream> for Stream {
    fn from(stream: &AutoStream) -> Self {
        todo!()
    }
}

impl From<&AutoPlacement> for Placement {
    fn from(placement: &AutoPlacement) -> Self {
        todo!()
    }
}

impl From<&AutoFlowArrow> for FlowArrow {
    fn from(flow: &AutoFlowArrow) -> Self {
        todo!()
    }
}

//  Event Model Implementation

static NAME: &str = "name";
static DESCRIPTION: &str = "description";
static SCHEMA: &str = "schema";
static EMPTY_STR: &str = "";

#[derive(Debug)]
pub struct AutomergeEventModel {
    crdt: AutoCommit,
    value: AutoSurgeonEventModel,
}

impl AutomergeEventModel {
    pub fn new(id: EventModelId, name: String) -> Self {
        let mut crdt = AutoCommit::new();
        let value = AutoSurgeonEventModel::new(id, name);
        reconcile(&mut crdt, &value).unwrap();
        AutomergeEventModel { crdt, value }
    }

    pub fn load(data: &[u8]) -> Self {
        let crdt = AutoCommit::load(data).unwrap();
        let value: AutoSurgeonEventModel = hydrate(&crdt).unwrap();
        AutomergeEventModel { crdt, value }
    }

    pub fn load_incremental(&mut self, data: &[u8]) {
        self.crdt.load_incremental(data).unwrap();
        self.value = hydrate(&self.crdt).unwrap();
    }
}

impl Entity for AutomergeEventModel {
    fn id(&self) -> &Uuid {
        &self.value.id
    }
}

impl Named for AutomergeEventModel {
    fn name(&self) -> &str {
        &self.value.name
    }
}

impl Described for AutomergeEventModel {
    fn description(&self) -> &str {
        self.value.description.as_str()
    }
}

impl HasSchema for AutomergeEventModel {
    fn schema(&self) -> &Schema {
        let schema = &self.value.schema;
        &Schema(schema.as_str().to_string())
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
        &self
            .value
            .interfaces
            .values()
            .map(|i| (i.id, i.into()))
            .collect()
    }

    fn commands(&self) -> &HashMap<CommandId, Command> {
        &self
            .value
            .commands
            .values()
            .map(|c| (c.id, c.into()))
            .collect()
    }

    fn events(&self) -> &HashMap<EventId, Event> {
        &self
            .value
            .events
            .values()
            .map(|e| (e.id, e.into()))
            .collect()
    }

    fn read_models(&self) -> &HashMap<ReadModelId, ReadModel> {
        &self
            .value
            .read_models
            .values()
            .map(|r| (r.id, r.into()))
            .collect()
    }

    fn audiences(&self) -> &Vec<Audience> {
        &self.value.audiences.into_iter().map(|x| x.into()).collect()
    }

    fn streams(&self) -> &Vec<Stream> {
        &self.value.streams.iter().map(|x| x.into()).collect()
    }

    fn placements(&self) -> &HashMap<PlacementId, Placement> {
        &self
            .value
            .placements
            .values()
            .map(|p| (*p.id(), p.into()))
            .collect()
    }

    fn flows(&self) -> &HashMap<FlowId, FlowArrow> {
        &self
            .value
            .flows
            .values()
            .map(|f| (f.id, f.into()))
            .collect()
    }
}

impl Renamable for AutomergeEventModel {
    fn rename(&mut self, name: &str) {
        reconcile_prop(&mut self.crdt, ROOT, NAME, name).unwrap();
        self.value.name = name.to_string();
    }
}

impl ModifiablyDescribed for AutomergeEventModel {
    // TODO: refactor this out of trait and into InMemoryEventModel
    fn description_mut(&mut self) -> &mut String {
        todo!()
    }

    fn set_description(&mut self, description: &str) {
        self.value.description = Text::with_value(description);
        reconcile_prop(&mut self.crdt, ROOT, DESCRIPTION, self.value.description).unwrap();
    }

    fn add_to_description(&mut self, index: usize, addition: &str) {
        self.value.description.splice(index, 0, addition);
        reconcile_prop(&mut self.crdt, ROOT, DESCRIPTION, self.value.description).unwrap();
    }

    fn delete_from_description(&mut self, index: usize, count: usize) {
        self.value.description.splice(index, count, EMPTY_STR);
        reconcile_prop(&mut self.crdt, ROOT, DESCRIPTION, self.value.description).unwrap();
    }
}

impl HasModifiableSchema for AutomergeEventModel {
    // TODO: refactor this out of trait and into InMemoryEventModel
    fn schema_mut(&mut self) -> &mut Schema {
        todo!()
    }

    fn set_schema(&mut self, schema: Schema) {
        self.value.schema = Text::with_value(schema.0);
        reconcile_prop(&mut self.crdt, ROOT, SCHEMA, self.value.schema).unwrap();
    }

    fn add_to_schema(&mut self, index: usize, addition: &str) {
        self.value.schema.splice(index, 0, addition);
        reconcile_prop(&mut self.crdt, ROOT, SCHEMA, self.value.schema).unwrap();
    }

    fn delete_from_schema(&mut self, index: usize, count: usize) {
        self.value.schema.splice(index, count, EMPTY_STR);
        reconcile_prop(&mut self.crdt, ROOT, SCHEMA, self.value.schema).unwrap();
    }
}

// impl ModifiableEventModel for AutomergeEventModel {
//     fn component_defined(&mut self, component: Component) {
//         match component {
//             Component::Interface(i) => {
//                 let Ok(Some((Value::Object(ObjType::Map), interfaces))) =
//                     self.crdt.get(ROOT, INTERFACES) else {
//                         panic!("No interfaces Map found");
//                     };
//                 let interface_doc = self
//                     .crdt
//                     .put_object(interfaces, i.id().to_string(), ObjType::Map)
//                     .unwrap();
//                 self.crdt
//                     .put(&interface_doc, ID, i.id().to_string())
//                     .unwrap();
//                 self.crdt.put(&interface_doc, NAME, i.name()).unwrap();
//                 let description = self
//                     .crdt
//                     .put_object(&interface_doc, DESCRIPTION, ObjType::Text)
//                     .unwrap();
//                 self.crdt
//                     .splice_text(&description, 0, 0, i.description())
//                     .unwrap();
//                 let config = self
//                     .crdt
//                     .put_object(&interface_doc, CONFIG, ObjType::Map)
//                     .unwrap();
//                 match i.config() {
//                     event_models::types::InterfaceConfig::Blank => {
//                         self.crdt.put(&config, TYPE, "Blank").unwrap();
//                     }
//                     event_models::types::InterfaceConfig::Figma { url, width, height } => {
//                         self.crdt.put(&config, TYPE, "Figma").unwrap();
//                         self.crdt.put(&config, URL, url.to_string()).unwrap();
//                         if let Some(w) = width {
//                             self.crdt.put(&config, WIDTH, *w as u32).unwrap();
//                         }
//                         if let Some(h) = height {
//                             self.crdt.put(&config, HEIGHT, *h as u32).unwrap();
//                         }
//                     }
//                     event_models::types::InterfaceConfig::Image { url, width, height } => {
//                         self.crdt.put(&config, TYPE, "Image").unwrap();
//                         self.crdt.put(&config, URL, url.to_string()).unwrap();
//                         if let Some(w) = width {
//                             self.crdt.put(&config, WIDTH, *w as u32).unwrap();
//                         }
//                         if let Some(h) = height {
//                             self.crdt.put(&config, HEIGHT, *h as u32).unwrap();
//                         }
//                     }
//                     event_models::types::InterfaceConfig::Job => {
//                         self.crdt.put(&config, TYPE, "Job").unwrap();
//                     }
//                 }
//                 self.value.interfaces.insert(*i.id(), i.to_owned());
//             }
//             Component::Command(c) => {
//                 let Ok(Some((Value::Object(ObjType::Map), commands))) =
//                     self.crdt.get(ROOT, COMMANDS) else {
//                         panic!("No commands Map found");
//                     };
//                 let command_doc = self
//                     .crdt
//                     .put_object(commands, c.id().to_string(), ObjType::Map)
//                     .unwrap();
//                 self.crdt.put(&command_doc, ID, c.id().to_string()).unwrap();
//                 self.crdt.put(&command_doc, NAME, c.name()).unwrap();
//                 let description = self
//                     .crdt
//                     .put_object(&command_doc, DESCRIPTION, ObjType::Text)
//                     .unwrap();
//                 self.crdt
//                     .splice_text(&description, 0, 0, c.description())
//                     .unwrap();
//                 let schema = self
//                     .crdt
//                     .put_object(&command_doc, SCHEMA, ObjType::Text)
//                     .unwrap();
//                 self.crdt.splice_text(&schema, 0, 0, &c.schema().0).unwrap();
//                 self.value.commands.insert(*c.id(), c.to_owned());
//             }
//             Component::Event(e) => {
//                 let Ok(Some((Value::Object(ObjType::Map), events))) =
//                     self.crdt.get(ROOT, EVENTS) else {
//                         panic!("No events Map found");
//                     };
//                 let event_doc = self
//                     .crdt
//                     .put_object(events, e.id().to_string(), ObjType::Map)
//                     .unwrap();
//                 self.crdt.put(&event_doc, ID, e.id().to_string()).unwrap();
//                 self.crdt.put(&event_doc, NAME, e.name()).unwrap();
//                 let description = self
//                     .crdt
//                     .put_object(&event_doc, DESCRIPTION, ObjType::Text)
//                     .unwrap();
//                 self.crdt
//                     .splice_text(&description, 0, 0, e.description())
//                     .unwrap();
//                 let schema = self
//                     .crdt
//                     .put_object(&event_doc, SCHEMA, ObjType::Text)
//                     .unwrap();
//                 self.crdt.splice_text(&schema, 0, 0, &e.schema().0).unwrap();
//                 self.value.events.insert(*e.id(), e.to_owned());
//             }
//             Component::ReadModel(r) => {
//                 let Ok(Some((Value::Object(ObjType::Map), read_models))) =
//                     self.crdt.get(ROOT, READ_MODELS) else {
//                         panic!("No read models Map found");
//                     };
//                 let read_model_doc = self
//                     .crdt
//                     .put_object(read_models, r.id().to_string(), ObjType::Map)
//                     .unwrap();
//                 self.crdt
//                     .put(&read_model_doc, ID, r.id().to_string())
//                     .unwrap();
//                 self.crdt.put(&read_model_doc, NAME, r.name()).unwrap();
//                 let description = self
//                     .crdt
//                     .put_object(&read_model_doc, DESCRIPTION, ObjType::Text)
//                     .unwrap();
//                 self.crdt
//                     .splice_text(&description, 0, 0, r.description())
//                     .unwrap();
//                 let schema = self
//                     .crdt
//                     .put_object(&read_model_doc, SCHEMA, ObjType::Text)
//                     .unwrap();
//                 self.crdt.splice_text(&schema, 0, 0, &r.schema().0).unwrap();
//                 self.value.read_models.insert(*r.id(), r.to_owned());
//             }
//         }
//     }

//     fn component_renamed(&mut self, component_id: &ComponentId, name: &str) {
//         match self.value.component_mut_by_id(component_id) {
//             Some(ComponentMut::Interface(i)) => {
//                 let Ok(Some((Value::Object(ObjType::Map), interfaces))) =
//                     self.crdt.get(ROOT, INTERFACES) else {
//                         panic!("No interfaces Map found");
//                     };

//                 let Ok(Some((Value::Object(ObjType::Map), interface))) =
//                     self.crdt.get(interfaces, i.id().to_string()) else {
//                         panic!("No interface with ID={:?} found", i.id());
//                     };

//                 self.crdt.put(interface, NAME, name).unwrap();
//                 i.rename(name);
//             }
//             Some(ComponentMut::Command(c)) => {
//                 let Ok(Some((Value::Object(ObjType::Map), commands))) =
//                     self.crdt.get(ROOT, COMMANDS) else {
//                         panic!("No commands Map found");
//                     };

//                 let Ok(Some((Value::Object(ObjType::Map), command))) =
//                     self.crdt.get(commands, c.id().to_string()) else {
//                         panic!("No command with ID={:?} found", c.id());
//                     };

//                 self.crdt.put(command, NAME, name).unwrap();
//                 c.rename(name);
//             }
//             Some(ComponentMut::Event(e)) => {
//                 let Ok(Some((Value::Object(ObjType::Map), events))) =
//                     self.crdt.get(ROOT, EVENTS) else {
//                         panic!("No events Map found");
//                     };

//                 let Ok(Some((Value::Object(ObjType::Map), event))) =
//                     self.crdt.get(events, e.id().to_string()) else {
//                         panic!("No event with ID={:?} found", e.id());
//                     };

//                 self.crdt.put(event, NAME, name).unwrap();
//                 e.rename(name);
//             }
//             Some(ComponentMut::ReadModel(r)) => {
//                 let Ok(Some((Value::Object(ObjType::Map), read_models))) =
//                     self.crdt.get(ROOT, READ_MODELS) else {
//                         panic!("No read models Map found");
//                     };

//                 let Ok(Some((Value::Object(ObjType::Map), read_model))) =
//                     self.crdt.get(read_models, r.id().to_string()) else {
//                         panic!("No read model with ID={:?} found", r.id());
//                     };

//                 self.crdt.put(read_model, NAME, name).unwrap();
//                 r.rename(name);
//             }
//             None => panic!("Component with id {:?} not found", component_id),
//         }
//     }

//     fn component_removed(&mut self, component_id: &ComponentId) {
//         match component_id {
//             ComponentId::InterfaceComponentId(id) => {
//                 let Ok(Some((Value::Object(ObjType::Map), interfaces))) =
//                     self.crdt.get(ROOT, INTERFACES) else {
//                         panic!("No interfaces Map found");
//                     };
//                 self.crdt.delete(interfaces, id.to_string()).unwrap();
//                 self.value.interfaces.remove(id);
//             }
//             ComponentId::CommandComponentId(id) => {
//                 let Ok(Some((Value::Object(ObjType::Map), commands))) =
//                     self.crdt.get(ROOT, COMMANDS) else {
//                         panic!("No commands Map found");
//                     };
//                 self.crdt.delete(commands, id.to_string()).unwrap();
//                 self.value.commands.remove(id);
//             }
//             ComponentId::EventComponentId(id) => {
//                 let Ok(Some((Value::Object(ObjType::Map), events))) =
//                     self.crdt.get(ROOT, EVENTS) else {
//                         panic!("No events Map found");
//                     };
//                 self.crdt.delete(events, id.to_string()).unwrap();
//                 self.value.events.remove(id);
//             }
//             ComponentId::ReadModelComponentId(id) => {
//                 let Ok(Some((Value::Object(ObjType::Map), read_models))) =
//                     self.crdt.get(ROOT, READ_MODELS) else {
//                         panic!("No read models Map found");
//                     };
//                 self.crdt.delete(read_models, id.to_string()).unwrap();
//                 self.value.read_models.remove(id);
//             }
//         }
//     }

//     fn added_to_component_description(
//         &mut self,
//         component_id: &ComponentId,
//         index: usize,
//         addition: &str,
//     ) {
//         match self.value.component_mut_by_id(component_id) {
//             Some(ComponentMut::Interface(i)) => todo!(),
//             Some(ComponentMut::Command(c)) => todo!(),
//             Some(ComponentMut::Event(e)) => todo!(),
//             Some(ComponentMut::ReadModel(r)) => todo!(),
//             None => todo!(),
//         }
//     }

//     fn deleted_from_component_description(
//         &mut self,
//         component_id: &ComponentId,
//         index: usize,
//         count: usize,
//     ) {
//         match self.value.component_mut_by_id(component_id) {
//             Some(ComponentMut::Interface(i)) => todo!(),
//             Some(ComponentMut::Command(c)) => todo!(),
//             Some(ComponentMut::Event(e)) => todo!(),
//             Some(ComponentMut::ReadModel(r)) => todo!(),
//             None => todo!(),
//         }
//     }

//     fn added_to_component_schema(
//         &mut self,
//         component_id: &ComponentId,
//         index: usize,
//         addition: &str,
//     ) {
//         match self.value.component_mut_by_id(component_id) {
//             Some(ComponentMut::Interface(i)) => todo!(),
//             Some(ComponentMut::Command(c)) => todo!(),
//             Some(ComponentMut::Event(e)) => todo!(),
//             Some(ComponentMut::ReadModel(r)) => todo!(),
//             None => todo!(),
//         }
//     }

//     fn deleted_from_component_schema(
//         &mut self,
//         component_id: &ComponentId,
//         index: usize,
//         count: usize,
//     ) {
//         match self.value.component_mut_by_id(component_id) {
//             Some(ComponentMut::Interface(i)) => todo!(),
//             Some(ComponentMut::Command(c)) => todo!(),
//             Some(ComponentMut::Event(e)) => todo!(),
//             Some(ComponentMut::ReadModel(r)) => todo!(),
//             None => todo!(),
//         }
//     }

//     fn component_placed(&mut self, placement: &Placement) {
//         let Ok(Some((Value::Object(ObjType::Map), obj_id))) = self.crdt.get(ROOT, PLACEMENTS) else { todo!() };
//         let placement_doc = self
//             .crdt
//             .put_object(&obj_id, placement.id().to_string(), ObjType::Map)
//             .unwrap();
//         match placement {
//             Placement::Interface {
//                 id,
//                 index,
//                 interface,
//                 audience,
//             } => {
//                 self.crdt.put(&placement_doc, ID, id.to_string()).unwrap();
//                 self.crdt.put(&placement_doc, INDEX, *index as u32).unwrap();
//                 self.crdt
//                     .put(&placement_doc, INTERFACE, interface.to_string())
//                     .unwrap();
//                 if let Some(a) = audience {
//                     self.crdt
//                         .put(&placement_doc, AUDIENCE, a.to_string())
//                         .unwrap();
//                 }
//             }
//             Placement::Command {
//                 id,
//                 index,
//                 command,
//                 schema,
//             } => {
//                 self.crdt.put(&placement_doc, ID, id.to_string()).unwrap();
//                 self.crdt.put(&placement_doc, INDEX, *index as u32).unwrap();
//                 self.crdt
//                     .put(&placement_doc, COMMAND, command.to_string())
//                     .unwrap();
//                 self.crdt
//                     .put(&placement_doc, COMMAND, command.to_string())
//                     .unwrap();
//                 let schema_doc = self
//                     .crdt
//                     .put_object(&placement_doc, SCHEMA, ObjType::Text)
//                     .unwrap();
//                 self.crdt.splice_text(&schema_doc, 0, 0, &schema.0).unwrap();
//             }
//             Placement::Event {
//                 id,
//                 index,
//                 event,
//                 stream,
//                 schema,
//             } => {
//                 self.crdt.put(&placement_doc, ID, id.to_string()).unwrap();
//                 self.crdt.put(&placement_doc, INDEX, *index as u32).unwrap();
//                 self.crdt
//                     .put(&placement_doc, EVENT, event.to_string())
//                     .unwrap();

//                 let schema_doc = self
//                     .crdt
//                     .put_object(&placement_doc, SCHEMA, ObjType::Text)
//                     .unwrap();
//                 self.crdt.splice_text(&schema_doc, 0, 0, &schema.0).unwrap();

//                 if let Some(a) = stream {
//                     self.crdt
//                         .put(&placement_doc, STREAM, a.to_string())
//                         .unwrap();
//                 }
//             }
//             Placement::ReadModel {
//                 id,
//                 index,
//                 read_model,
//                 schema,
//             } => {
//                 self.crdt.put(&placement_doc, ID, id.to_string()).unwrap();
//                 self.crdt.put(&placement_doc, INDEX, *index as u32).unwrap();
//                 self.crdt
//                     .put(&placement_doc, READ_MODEL, read_model.to_string())
//                     .unwrap();

//                 let schema_doc = self
//                     .crdt
//                     .put_object(&placement_doc, SCHEMA, ObjType::Text)
//                     .unwrap();
//                 self.crdt.splice_text(&schema_doc, 0, 0, &schema.0).unwrap();
//             }
//         }

//         self.value
//             .placements
//             .insert(placement.id().to_owned(), placement.to_owned());
//     }

//     fn placement_moved(&mut self, position: &PlacementPosition) {
//         if let Some(ref mut placement) = self.value.placements.get_mut(position.id()) {
//             let PlacementPosition(_, index, lane) = position;
//             let Ok(Some((Value::Object(ObjType::Map), placements))) =
//                 self.crdt.get(ROOT, PLACEMENTS) else { todo!() };
//             let Ok(Some((Value::Object(ObjType::Map), placement_doc))) =
//                 self.crdt.get(placements, placement.id().to_string()) else { todo!() };
//             self.crdt.put(&placement_doc, INDEX, *index as u32).unwrap();
//             match lane {
//                 LaneId::Audience(id) => self
//                     .crdt
//                     .put(&placement_doc, AUDIENCE, id.to_string())
//                     .unwrap(),
//                 LaneId::Stream(id) => self
//                     .crdt
//                     .put(&placement_doc, STREAM, id.to_string())
//                     .unwrap(),
//                 _ => (),
//             }
//             placement.relocate(index.to_owned(), lane.to_owned());
//         };
//     }

//     fn placement_removed(&mut self, placement_id: &PlacementId) {
//         let Ok(Some((Value::Object(ObjType::Map), placements))) =
//             self.crdt.get(ROOT, PLACEMENTS) else { todo!() };
//         self.crdt
//             .delete(placements, placement_id.to_string())
//             .unwrap();
//         self.value.placements.remove(placement_id);
//     }

//     fn placements_shifted(&mut self, offset: &usize, width: &usize) {
//         let Ok(Some((Value::Object(ObjType::Map), placements))) =
//             self.crdt.get(ROOT, PLACEMENTS) else { todo!() };
//         self.value.placements.iter_mut().for_each(|(_, placement)| {
//             if placement.index() >= offset {
//                 let Ok(Some((Value::Object(ObjType::Map), placement_doc))) =
//                     self.crdt.get(&placements, placement.id().to_string()) else { todo!() };
//                 let Ok(Some((Value::Scalar(Cow::Owned(ScalarValue::Uint(old_index))), _index_op_id))) =
//                     self.crdt.get(&placements, placement.id().to_string()) else { todo!() };
//                 // TODO: re-implement in terms of looking up existing
//                 // index in placement, shifting, and then setting in
//                 // CRDT.  Current impl leaves room for commutativity error?
//                 self.crdt.put(placement_doc, INDEX, old_index as u32 + *width as u32).unwrap();
//                 placement.shift_right(*width);
//             }
//         })
//     }

//     fn added_to_placement_schema(
//         &mut self,
//         placement_id: &PlacementId,
//         index: usize,
//         addition: &str,
//     ) {
//         todo!()
//     }

//     fn deleted_from_placement_schema(
//         &mut self,
//         placement_id: &PlacementId,
//         index: usize,
//         count: usize,
//     ) {
//         todo!()
//     }

//     fn lane_added(&mut self, lane: Lane, index: LaneIndex) {
//         match lane {
//             Lane::Audience(audience) => {
//                 let Ok(Some((Value::Object(ObjType::List), audiences))) =
//                     self.crdt.get(ROOT, AUDIENCES) else { todo!() };
//                 let audience_doc = self
//                     .crdt
//                     .insert_object(audiences, index, ObjType::Map)
//                     .unwrap();
//                 self.crdt
//                     .put(&audience_doc, ID, audience.id().to_string())
//                     .unwrap();
//                 self.crdt.put(&audience_doc, NAME, audience.name()).unwrap();
//                 self.value.audiences.insert(index, audience);
//             }
//             Lane::Stream(stream) => {
//                 let Ok(Some((Value::Object(ObjType::List), streams))) =
//                     self.crdt.get(ROOT, STREAMS) else { todo!() };
//                 let stream_doc = self
//                     .crdt
//                     .insert_object(streams, index, ObjType::Map)
//                     .unwrap();
//                 self.crdt
//                     .put(&stream_doc, ID, stream.id().to_string())
//                     .unwrap();
//                 self.crdt.put(&stream_doc, NAME, stream.name()).unwrap();
//                 self.value.streams.insert(index, stream);
//             }
//         }
//     }

//     fn lane_renamed(&mut self, lane_id: LaneId, name: &str) {
//         todo!()
//     }

//     fn lane_reordered(&mut self, lane_id: LaneId, index: LaneIndex) {
//         todo!()
//     }

//     fn lane_removed(&mut self, lane_id: LaneId) {
//         todo!()
//     }

//     fn plus_flow(&mut self, flow_arrow: FlowArrow) {
//         let Ok(Some((Value::Object(ObjType::Map), flows))) =
//             self.crdt.get(ROOT, FLOWS) else { todo!() };
//         let flow_doc = self
//             .crdt
//             .put_object(flows, flow_arrow.id().to_string(), ObjType::Map)
//             .unwrap();
//         self.crdt
//             .put(
//                 &flow_doc,
//                 FROM_PLACEMENT,
//                 flow_arrow.from().placement_id().to_string(),
//             )
//             .unwrap();
//         self.crdt
//             .put(
//                 &flow_doc,
//                 FROM_ANCHOR,
//                 flow_arrow.from().anchor().to_string(),
//             )
//             .unwrap();
//         self.crdt
//             .put(
//                 &flow_doc,
//                 TO_PLACEMENT,
//                 flow_arrow.to().placement_id().to_string(),
//             )
//             .unwrap();
//         self.crdt
//             .put(&flow_doc, TO_ANCHOR, flow_arrow.to().anchor().to_string())
//             .unwrap();
//         self.value
//             .flows
//             .insert(flow_arrow.id().to_owned(), flow_arrow);
//     }

//     fn minus_flow(&mut self, flow_id: &FlowId) {
//         todo!()
//     }
// }
