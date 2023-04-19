use std::collections::HashMap;

use crate::{
    Audience, Command, CommandId, Component, ComponentId, Described, Entity, Event, EventId,
    EventModel, EventModelData, EventModelId, EventModelState, FlowArrow, FlowId, HasSchema,
    Interface, InterfaceId, Lane, LaneId, LaneIndex, ModifiableEventModel, Name, Named, Placement,
    PlacementId, PlacementPosition, ReadModel, ReadModelId, Stream,
};
use autosurgeon::{Hydrate, Reconcile, Text};

use uuid::Uuid;

const EMPTY_STR: &str = "";

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
pub struct AutoInterface {
    #[key]
    id: Uuid,
    name: String,
    description: Text,
    config: AutoInterfaceConfig,
}

#[derive(Reconcile, Hydrate, Debug)]
pub struct AutoCommand {
    #[key]
    id: Uuid,
    name: String,
    description: Text,
    schema: Text,
}

#[derive(Reconcile, Hydrate, Debug)]
pub struct AutoEvent {
    #[key]
    id: Uuid,
    name: String,
    description: Text,
    schema: Text,
}

#[derive(Reconcile, Hydrate, Debug)]
pub struct AutoReadModel {
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
    fn id(&self) -> Uuid {
        match self {
            AutoPlacement::Interface { id, .. } => *id,
            AutoPlacement::Command { id, .. } => *id,
            AutoPlacement::Event { id, .. } => *id,
            AutoPlacement::ReadModel { id, .. } => *id,
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

impl From<&AutoAudience> for Audience {
    fn from(audience: &AutoAudience) -> Self {
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

#[derive(Reconcile, Hydrate, Debug)]
pub struct AutomergeEventModel {
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

impl AutomergeEventModel {
    pub fn new(id: &Uuid, name: &String) -> Self {
        AutomergeEventModel {
            id: *id,
            name: name.to_owned(),
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

impl Entity for AutomergeEventModel {
    fn id(&self) -> Uuid {
        self.id.to_owned()
    }
}

impl Named for AutomergeEventModel {
    fn name(&self) -> Name {
        Name::create(&self.name).unwrap()
    }
}

impl Described for AutomergeEventModel {
    fn description(&self) -> &str {
        self.description.as_str()
    }
}

impl HasSchema for AutomergeEventModel {
    fn schema(&self) -> &str {
        self.schema.as_str()
    }
}

impl EventModel for AutomergeEventModel {
    type CreationDetails = ();

    fn create(initial: &EventModelState<Self>, id: &EventModelId, name: &Name) -> Self {
        match initial {
            EventModelState::BeforeCreation(_) => AutomergeEventModel::new(id, name.into()),
            _ => panic!("Illegal state when creating Automerge Event Model!"),
        }
    }
}

impl EventModelData for AutomergeEventModel {
    fn interfaces(&self) -> HashMap<InterfaceId, Interface> {
        self.interfaces.values().map(|i| (i.id, i.into())).collect()
    }

    fn commands(&self) -> HashMap<CommandId, Command> {
        self.commands.values().map(|c| (c.id, c.into())).collect()
    }

    fn events(&self) -> HashMap<EventId, Event> {
        self.events.values().map(|e| (e.id, e.into())).collect()
    }

    fn read_models(&self) -> HashMap<ReadModelId, ReadModel> {
        self.read_models
            .values()
            .map(|r| (r.id, r.into()))
            .collect()
    }

    fn audiences(&self) -> Vec<Audience> {
        self.audiences.iter().map(|x| x.into()).collect()
    }

    fn streams(&self) -> Vec<Stream> {
        self.streams.iter().map(|x| x.into()).collect()
    }

    fn placements(&self) -> HashMap<PlacementId, Placement> {
        self.placements
            .values()
            .map(|p| (p.id(), p.into()))
            .collect()
    }

    fn flows(&self) -> HashMap<FlowId, FlowArrow> {
        self.flows.values().map(|f| (f.id, f.into())).collect()
    }

    fn get_placement(&self, id: &PlacementId) -> Option<Placement> {
        self.placements.get(&id.to_string()).map(|p| p.into())
    }
}

// impl Renamable for AutomergeEventModel {
//     fn rename(&mut self, name: &str) {
//         self.name = name.to_string();
//     }
// }

// impl ModifiablyDescribed for AutomergeEventModel {
//     fn set_description(&mut self, description: &str) {
//         self.description = Text::with_value(description);
//     }

//     fn add_to_description(&mut self, index: usize, addition: &str) {
//         self.description.splice(index, 0, addition);
//     }

//     fn delete_from_description(&mut self, index: usize, count: usize) {
//         self.description.splice(index, count, EMPTY_STR);
//     }
// }

// impl HasModifiableSchema for AutomergeEventModel {
//     fn set_schema(&mut self, schema: &str) {
//         self.schema = Text::with_value(schema);
//     }

//     fn add_to_schema(&mut self, index: usize, addition: &str) {
//         self.schema.splice(index, 0, addition);
//     }

//     fn delete_from_schema(&mut self, index: usize, count: usize) {
//         self.schema.splice(index, count, EMPTY_STR);
//     }
// }

impl ModifiableEventModel for AutomergeEventModel {
    fn rename(&mut self, name: &Name) {
        todo!()
    }

    fn splice_description(&mut self, index: usize, del: usize, add: &str) {
        todo!()
    }

    fn splice_schema(&mut self, index: usize, del: usize, add: &str) {
        todo!()
    }

    fn component_defined(&mut self, component: &Component) {
        todo!()
    }

    fn component_renamed(&mut self, component_id: &ComponentId, name: &Name) {
        todo!()
    }

    fn component_removed(&mut self, component_id: &ComponentId) {
        todo!()
    }

    fn splice_component_description(
        &mut self,
        component_id: &ComponentId,
        index: usize,
        del: usize,
        addition: &str,
    ) {
        todo!()
    }

    fn splice_component_schema(
        &mut self,
        component_id: &ComponentId,
        index: usize,
        del: usize,
        addition: &str,
    ) {
        todo!()
    }

    fn component_placed(&mut self, placement: &Placement) {
        todo!()
    }

    fn placement_moved(&mut self, position: &PlacementPosition) {
        todo!()
    }

    fn placement_removed(&mut self, placement_id: &PlacementId) {
        todo!()
    }

    fn placements_shifted(&mut self, offset: usize, width: usize) {
        todo!()
    }

    fn splice_placement_schema(
        &mut self,
        placement_id: &PlacementId,
        index: usize,
        del: usize,
        addition: &str,
    ) {
        todo!()
    }

    fn lane_added(&mut self, lane: &Lane, index: LaneIndex) {
        todo!()
    }

    fn lane_renamed(&mut self, lane_id: &LaneId, name: &Name) {
        todo!()
    }

    fn lane_reordered(&mut self, lane_id: &LaneId, index: LaneIndex) {
        todo!()
    }

    fn lane_removed(&mut self, lane_id: &LaneId) {
        todo!()
    }

    fn plus_flow(&mut self, flow_arrow: &FlowArrow) {
        todo!()
    }

    fn minus_flow(&mut self, flow_id: &FlowId) {
        todo!()
    }
}
