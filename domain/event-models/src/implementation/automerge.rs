use std::{collections::HashMap, str::FromStr};

use crate::{
    validate_name, Anchor, Audience, Command, CommandId, Component, ComponentId, Described, Entity,
    Event, EventId, EventModel, EventModelData, EventModelError, EventModelId, EventModelState,
    FlowArrow, FlowId, HasSchema, Interface, InterfaceConfig, InterfaceId, Lane, LaneId, LaneIndex,
    ModifiableEventModel, Name, Named, Placement, PlacementId, PlacementIndex, PlacementPosition,
    Port, ReadModel, ReadModelId, Stream,
};
use autosurgeon::{Hydrate, Reconcile, Text};

use url::Url;
use uuid::Uuid;

#[derive(Reconcile, Hydrate, Debug, Clone)]
struct AutoName(String);

impl AutoName {
    fn create(value: &str) -> Result<Self, EventModelError> {
        let name = validate_name(value)?;
        Ok(AutoName(name))
    }
}

impl From<&Name> for AutoName {
    fn from(n: &Name) -> Self {
        AutoName(n.into())
    }
}

impl From<&AutoName> for Name {
    fn from(n: &AutoName) -> Self {
        Name::create(&n.0).unwrap()
    }
}

#[derive(Reconcile, Hydrate, Debug, Clone)]
enum AutoInterfaceConfig {
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

impl From<&AutoInterfaceConfig> for InterfaceConfig {
    fn from(interface_config: &AutoInterfaceConfig) -> Self {
        match interface_config {
            AutoInterfaceConfig::Blank => InterfaceConfig::Blank,
            AutoInterfaceConfig::Figma { url, width, height } => InterfaceConfig::Figma {
                url: Url::from_str(url).unwrap(),
                width: width.map(|w| w as usize),
                height: height.map(|h| h as usize),
            },
            AutoInterfaceConfig::Image { url, width, height } => InterfaceConfig::Image {
                url: Url::from_str(url).unwrap(),
                width: width.map(|w| w as usize),
                height: height.map(|h| h as usize),
            },
            AutoInterfaceConfig::Job => InterfaceConfig::Job,
        }
    }
}

impl From<&InterfaceConfig> for AutoInterfaceConfig {
    fn from(interface_config: &InterfaceConfig) -> Self {
        match interface_config {
            InterfaceConfig::Blank => AutoInterfaceConfig::Blank,
            InterfaceConfig::Figma { url, width, height } => AutoInterfaceConfig::Figma {
                url: url.to_string(),
                width: width.map(|w| w as u32),
                height: height.map(|h| h as u32),
            },
            InterfaceConfig::Image { url, width, height } => AutoInterfaceConfig::Image {
                url: url.to_string(),
                width: width.map(|w| w as u32),
                height: height.map(|h| h as u32),
            },
            InterfaceConfig::Job => AutoInterfaceConfig::Job,
        }
    }
}

#[derive(Reconcile, Hydrate, Debug, Clone)]
struct AutoInterface {
    #[key]
    id: Uuid,
    name: AutoName,
    description: Text,
    config: AutoInterfaceConfig,
}

impl From<&AutoInterface> for Interface {
    fn from(interface: &AutoInterface) -> Self {
        let name: &AutoName = &interface.name;
        let config: &AutoInterfaceConfig = &interface.config;
        Interface {
            id: interface.id,
            name: name.into(),
            description: interface.description.as_str().to_string(),
            config: config.into(),
        }
    }
}

impl From<&Interface> for AutoInterface {
    fn from(interface: &Interface) -> Self {
        let name: &Name = &interface.name;
        let config: &InterfaceConfig = &interface.config;
        AutoInterface {
            id: interface.id,
            name: name.into(),
            description: Text::with_value(&interface.description),
            config: config.into(),
        }
    }
}

#[derive(Reconcile, Hydrate, Debug, Clone)]
struct AutoCommand {
    #[key]
    id: Uuid,
    name: AutoName,
    description: Text,
    schema: Text,
}

impl From<&AutoCommand> for Command {
    fn from(command: &AutoCommand) -> Self {
        let name: &AutoName = &command.name;
        Command {
            id: command.id,
            name: name.into(),
            description: command.description.as_str().to_string(),
            schema: command.schema.as_str().to_string(),
        }
    }
}

impl From<&Command> for AutoCommand {
    fn from(command: &Command) -> Self {
        let name: &Name = &command.name;
        AutoCommand {
            id: command.id,
            name: name.into(),
            description: Text::with_value(&command.description),
            schema: Text::with_value(&command.schema),
        }
    }
}

#[derive(Reconcile, Hydrate, Debug, Clone)]
struct AutoEvent {
    #[key]
    id: Uuid,
    name: AutoName,
    description: Text,
    schema: Text,
}

impl From<&AutoEvent> for Event {
    fn from(event: &AutoEvent) -> Self {
        let name: &AutoName = &event.name;
        Event {
            id: event.id,
            name: name.into(),
            description: event.description.as_str().to_string(),
            schema: event.schema.as_str().to_string(),
        }
    }
}

impl From<&Event> for AutoEvent {
    fn from(event: &Event) -> Self {
        let name: &Name = &event.name;
        AutoEvent {
            id: event.id,
            name: name.into(),
            description: Text::with_value(&event.description),
            schema: Text::with_value(&event.schema),
        }
    }
}

#[derive(Reconcile, Hydrate, Debug, Clone)]
struct AutoReadModel {
    #[key]
    id: Uuid,
    name: AutoName,
    description: Text,
    schema: Text,
}

impl From<&AutoReadModel> for ReadModel {
    fn from(read_model: &AutoReadModel) -> Self {
        let name: &AutoName = &read_model.name;
        ReadModel {
            id: read_model.id,
            name: name.into(),
            description: read_model.description.as_str().to_string(),
            schema: read_model.schema.as_str().to_string(),
        }
    }
}

impl From<&ReadModel> for AutoReadModel {
    fn from(read_model: &ReadModel) -> Self {
        let name: &Name = &read_model.name;
        AutoReadModel {
            id: read_model.id,
            name: name.into(),
            description: Text::with_value(&read_model.description),
            schema: Text::with_value(&read_model.schema),
        }
    }
}

#[derive(Reconcile, Hydrate, Debug, Clone)]
struct AutoAudience {
    #[key]
    id: Uuid,
    name: AutoName,
}

impl From<&AutoAudience> for Audience {
    fn from(audience: &AutoAudience) -> Self {
        let name: &AutoName = &audience.name;
        Audience {
            id: audience.id,
            name: name.into(),
        }
    }
}

impl From<&Audience> for AutoAudience {
    fn from(audience: &Audience) -> Self {
        let name: &Name = &audience.name;
        AutoAudience {
            id: audience.id,
            name: name.into(),
        }
    }
}

#[derive(Reconcile, Hydrate, Debug, Clone)]
struct AutoStream {
    #[key]
    id: Uuid,
    name: AutoName,
}

impl From<&AutoStream> for Stream {
    fn from(stream: &AutoStream) -> Self {
        let name: &AutoName = &stream.name;
        Stream {
            id: stream.id,
            name: name.into(),
        }
    }
}

impl From<&Stream> for AutoStream {
    fn from(stream: &Stream) -> Self {
        let name: &Name = &stream.name;
        AutoStream {
            id: stream.id,
            name: name.into(),
        }
    }
}

#[derive(Reconcile, Hydrate, Debug, Clone)]
enum AutoPlacement {
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

impl AutoPlacement {
    fn index(&self) -> PlacementIndex {
        match self {
            AutoPlacement::Interface { index, .. } => *index as usize,
            AutoPlacement::Command { index, .. } => *index as usize,
            AutoPlacement::Event { index, .. } => *index as usize,
            AutoPlacement::ReadModel { index, .. } => *index as usize,
        }
    }

    fn shift_right(&mut self, offset: usize) {
        match self {
            AutoPlacement::Interface { index, .. } => *index += offset as u32,
            AutoPlacement::Command { index, .. } => *index += offset as u32,
            AutoPlacement::Event { index, .. } => *index += offset as u32,
            AutoPlacement::ReadModel { index, .. } => *index += offset as u32,
        }
    }

    fn relocate(&mut self, idx: PlacementIndex, lane: LaneId) {
        match self {
            AutoPlacement::Interface {
                index, audience, ..
            } => {
                *index = idx as u32;
                match lane {
                    LaneId::DefaultAudience => *audience = None,
                    LaneId::Audience(id) => *audience = Some(id),
                    _ => (),
                };
            }
            AutoPlacement::Command { index, .. } => *index = idx as u32,
            AutoPlacement::Event { index, stream, .. } => {
                *index = idx as u32;
                match lane {
                    LaneId::Stream(id) => *stream = Some(id),
                    LaneId::DefaultStream => *stream = None,
                    _ => (),
                }
            }
            AutoPlacement::ReadModel { index, .. } => *index = idx as u32,
        }
    }
}

impl From<&AutoPlacement> for Placement {
    fn from(placement: &AutoPlacement) -> Self {
        match placement {
            AutoPlacement::Interface {
                id,
                index,
                interface,
                audience,
            } => Placement::Interface {
                id: *id,
                index: *index as usize,
                interface: *interface,
                audience: *audience,
            },
            AutoPlacement::Command {
                id,
                index,
                command,
                schema,
            } => Placement::Command {
                id: *id,
                index: *index as usize,
                command: *command,
                schema: schema.as_str().to_string(),
            },
            AutoPlacement::Event {
                id,
                index,
                event,
                stream,
                schema,
            } => Placement::Event {
                id: *id,
                index: *index as usize,
                event: *event,
                stream: *stream,
                schema: schema.as_str().to_string(),
            },
            AutoPlacement::ReadModel {
                id,
                index,
                read_model,
                schema,
            } => Placement::ReadModel {
                id: *id,
                index: *index as usize,
                read_model: *read_model,
                schema: schema.as_str().to_string(),
            },
        }
    }
}

impl From<&Placement> for AutoPlacement {
    fn from(placement: &Placement) -> Self {
        match placement {
            Placement::Interface {
                id,
                index,
                interface,
                audience,
            } => AutoPlacement::Interface {
                id: *id,
                index: *index as u32,
                interface: *interface,
                audience: *audience,
            },
            Placement::Command {
                id,
                index,
                command,
                schema,
            } => AutoPlacement::Command {
                id: *id,
                index: *index as u32,
                command: *command,
                schema: Text::with_value(schema),
            },
            Placement::Event {
                id,
                index,
                event,
                stream,
                schema,
            } => AutoPlacement::Event {
                id: *id,
                index: *index as u32,
                event: *event,
                stream: *stream,
                schema: Text::with_value(schema),
            },
            Placement::ReadModel {
                id,
                index,
                read_model,
                schema,
            } => AutoPlacement::ReadModel {
                id: *id,
                index: *index as u32,
                read_model: *read_model,
                schema: Text::with_value(schema),
            },
        }
    }
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

#[derive(Reconcile, Hydrate, Debug, Clone)]
enum AutoAnchor {
    None,
    Top,
    Left,
    Bottom,
    Right,
}

impl From<&AutoAnchor> for Anchor {
    fn from(flow: &AutoAnchor) -> Self {
        match flow {
            AutoAnchor::None => Anchor::None,
            AutoAnchor::Top => Anchor::Top,
            AutoAnchor::Left => Anchor::Left,
            AutoAnchor::Bottom => Anchor::Bottom,
            AutoAnchor::Right => Anchor::Right,
        }
    }
}

impl From<&Anchor> for AutoAnchor {
    fn from(flow: &Anchor) -> Self {
        match flow {
            Anchor::None => AutoAnchor::None,
            Anchor::Top => AutoAnchor::Top,
            Anchor::Left => AutoAnchor::Left,
            Anchor::Bottom => AutoAnchor::Bottom,
            Anchor::Right => AutoAnchor::Right,
        }
    }
}

#[derive(Reconcile, Hydrate, Debug, Clone)]
struct AutoFlowArrow {
    #[key]
    id: Uuid,
    from_placement: Uuid,
    from_anchor: AutoAnchor,
    to_placement: Uuid,
    to_anchor: AutoAnchor,
}

impl From<&AutoFlowArrow> for FlowArrow {
    fn from(flow: &AutoFlowArrow) -> Self {
        let from_anchor: &AutoAnchor = &flow.from_anchor;
        let to_anchor: &AutoAnchor = &flow.to_anchor;
        FlowArrow {
            id: flow.id,
            from: Port {
                placement: flow.from_placement,
                anchor: from_anchor.into(),
            },
            to: Port {
                placement: flow.to_placement,
                anchor: to_anchor.into(),
            },
        }
    }
}

impl From<&FlowArrow> for AutoFlowArrow {
    fn from(flow: &FlowArrow) -> Self {
        let from_anchor: &Anchor = &flow.from.anchor;
        let to_anchor: &Anchor = &flow.to.anchor;
        AutoFlowArrow {
            id: flow.id,
            from_placement: flow.from.placement,
            from_anchor: from_anchor.into(),
            to_placement: flow.to.placement,
            to_anchor: to_anchor.into(),
        }
    }
}

enum AutoComponentMut<'a> {
    Interface(&'a mut AutoInterface),
    Command(&'a mut AutoCommand),
    Event(&'a mut AutoEvent),
    ReadModel(&'a mut AutoReadModel),
}

#[derive(Reconcile, Hydrate, Debug, Clone)]
pub struct AutomergeEventModel {
    #[key]
    id: Uuid,
    name: AutoName,
    description: Text,
    schema: Text,
    #[autosurgeon(with = "autosurgeon::map_with_parseable_keys")]
    interfaces: HashMap<Uuid, AutoInterface>,
    #[autosurgeon(with = "autosurgeon::map_with_parseable_keys")]
    commands: HashMap<Uuid, AutoCommand>,
    #[autosurgeon(with = "autosurgeon::map_with_parseable_keys")]
    events: HashMap<Uuid, AutoEvent>,
    #[autosurgeon(with = "autosurgeon::map_with_parseable_keys")]
    read_models: HashMap<Uuid, AutoReadModel>,
    audiences: Vec<AutoAudience>,
    streams: Vec<AutoStream>,
    #[autosurgeon(with = "autosurgeon::map_with_parseable_keys")]
    placements: HashMap<Uuid, AutoPlacement>,
    #[autosurgeon(with = "autosurgeon::map_with_parseable_keys")]
    flows: HashMap<Uuid, AutoFlowArrow>,
}

impl AutomergeEventModel {
    pub fn new(id: &Uuid, name: &Name) -> Self {
        AutomergeEventModel {
            id: *id,
            name: name.into(),
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

    fn component_mut_by_id(&mut self, id: &ComponentId) -> Option<AutoComponentMut> {
        match id {
            ComponentId::InterfaceComponentId(id) => {
                self.interfaces.get_mut(id).map(AutoComponentMut::Interface)
            }
            ComponentId::CommandComponentId(id) => {
                self.commands.get_mut(id).map(AutoComponentMut::Command)
            }
            ComponentId::EventComponentId(id) => {
                self.events.get_mut(id).map(AutoComponentMut::Event)
            }
            ComponentId::ReadModelComponentId(id) => self
                .read_models
                .get_mut(id)
                .map(AutoComponentMut::ReadModel),
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
        let n = &self.name;
        n.into()
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
    fn create(initial: &EventModelState<Self>, id: &EventModelId, name: &Name) -> Self {
        match initial {
            EventModelState::BeforeCreation => AutomergeEventModel::new(id, name),
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
        self.placements.get(id).map(|p| p.into())
    }
}

impl ModifiableEventModel for AutomergeEventModel {
    fn rename(&mut self, name: &Name) {
        self.name = name.into();
    }

    fn splice_description(&mut self, index: usize, del: usize, add: &str) {
        self.description.splice(index, del, add);
    }

    fn splice_schema(&mut self, index: usize, del: usize, add: &str) {
        self.schema.splice(index, del, add);
    }

    fn component_defined(&mut self, component: &Component) {
        match component {
            Component::Interface(i) => {
                self.interfaces.insert(i.id(), i.into());
            }
            Component::Command(c) => {
                self.commands.insert(c.id(), c.into());
            }
            Component::Event(e) => {
                self.events.insert(e.id(), e.into());
            }
            Component::ReadModel(r) => {
                self.read_models.insert(r.id(), r.into());
            }
        }
    }

    fn component_renamed(&mut self, component_id: &ComponentId, name: &Name) {
        match self.component_mut_by_id(component_id) {
            None => {
                panic!("Component with id {:?} not found", component_id)
            }
            Some(AutoComponentMut::Interface(i)) => i.name = name.into(),
            Some(AutoComponentMut::Command(c)) => c.name = name.into(),
            Some(AutoComponentMut::Event(e)) => e.name = name.into(),
            Some(AutoComponentMut::ReadModel(r)) => r.name = name.into(),
        }
    }

    fn component_removed(&mut self, component_id: &ComponentId) {
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
    }

    fn splice_component_description(
        &mut self,
        component_id: &ComponentId,
        index: usize,
        del: usize,
        addition: &str,
    ) {
        match self.component_mut_by_id(component_id) {
            Some(AutoComponentMut::Interface(i)) => {
                i.description.splice(index, del, addition);
            }
            Some(AutoComponentMut::Command(c)) => {
                c.description.splice(index, del, addition);
            }
            Some(AutoComponentMut::Event(e)) => {
                e.description.splice(index, del, addition);
            }
            Some(AutoComponentMut::ReadModel(r)) => {
                r.description.splice(index, del, addition);
            }
            None => {
                panic!("Component with id {:?} not found", component_id)
            }
        }
    }

    fn splice_component_schema(
        &mut self,
        component_id: &ComponentId,
        index: usize,
        del: usize,
        addition: &str,
    ) {
        match self.component_mut_by_id(component_id) {
            Some(AutoComponentMut::Interface(_)) => (),
            Some(AutoComponentMut::Command(c)) => {
                c.schema.splice(index, del, addition);
            }
            Some(AutoComponentMut::Event(e)) => {
                e.schema.splice(index, del, addition);
            }
            Some(AutoComponentMut::ReadModel(r)) => {
                r.schema.splice(index, del, addition);
            }
            None => {
                panic!("Component with id {:?} not found", component_id)
            }
        }
    }

    fn component_placed(&mut self, placement: &Placement) {
        self.placements.insert(placement.id(), placement.into());
    }

    fn placement_moved(&mut self, position: &PlacementPosition) {
        if let Some(ref mut placement) = self.placements.get_mut(&position.id()) {
            let PlacementPosition(_, index, lane) = position;
            placement.relocate(index.to_owned(), lane.to_owned());
        };
    }

    fn placement_removed(&mut self, placement_id: &PlacementId) {
        self.placements.remove(placement_id);
    }

    fn placements_shifted(&mut self, offset: usize, width: usize) {
        self.placements.iter_mut().for_each(|(_, placement)| {
            if placement.index() >= offset {
                placement.shift_right(width);
            }
        })
    }

    fn splice_placement_schema(
        &mut self,
        placement_id: &PlacementId,
        index: usize,
        del: usize,
        addition: &str,
    ) {
        match self.placements.get_mut(placement_id) {
            Some(AutoPlacement::Interface { .. }) => (),
            Some(AutoPlacement::Command { schema, .. }) => {
                schema.splice(index, del, addition);
            }
            Some(AutoPlacement::Event { schema, .. }) => {
                schema.splice(index, del, addition);
            }
            Some(AutoPlacement::ReadModel { schema, .. }) => {
                schema.splice(index, del, addition);
            }
            None => {
                panic!("Placement with id {:?} not found", placement_id)
            }
        }
    }

    fn lane_added(&mut self, lane: &Lane, index: LaneIndex) {
        match lane {
            Lane::Audience(audience) => self.audiences.insert(index, audience.into()),
            Lane::Stream(stream) => self.streams.insert(index, stream.into()),
        }
    }

    fn lane_renamed(&mut self, lane_id: &LaneId, name: &Name) {
        match lane_id {
            LaneId::Audience(id) => {
                if let Some(a) = self
                    .audiences
                    .iter_mut()
                    .find(|audience| *id == audience.id)
                {
                    a.name = name.into();
                }
            }
            LaneId::Stream(id) => {
                if let Some(s) = self.streams.iter_mut().find(|stream| *id == stream.id) {
                    s.name = name.into();
                }
            }
            _ => (),
        }
    }

    fn lane_reordered(&mut self, lane_id: &LaneId, index: LaneIndex) {
        match lane_id {
            LaneId::Audience(id) => {
                if let Some(idx) = self
                    .audiences
                    .iter_mut()
                    .position(|audience| *id == audience.id)
                {
                    let audience = self.audiences.remove(idx);
                    self.audiences.insert(index, audience);
                }
            }
            LaneId::Stream(id) => {
                if let Some(idx) = self.streams.iter_mut().position(|stream| *id == stream.id) {
                    let stream = self.streams.remove(idx);
                    self.streams.insert(index, stream);
                }
            }
            _ => (),
        }
    }

    fn lane_removed(&mut self, lane_id: &LaneId) {
        match lane_id {
            LaneId::Audience(id) => {
                if let Some(idx) = self
                    .audiences
                    .iter_mut()
                    .position(|audience| *id == audience.id)
                {
                    self.audiences.remove(idx);
                }
            }
            LaneId::Stream(id) => {
                if let Some(idx) = self.streams.iter_mut().position(|stream| *id == stream.id) {
                    self.streams.remove(idx);
                }
            }
            _ => (),
        }
    }

    fn plus_flow(&mut self, flow_arrow: &FlowArrow) {
        self.flows.insert(flow_arrow.id(), flow_arrow.into());
    }

    fn minus_flow(&mut self, flow_id: &FlowId) {
        self.flows.remove(flow_id);
    }
}
