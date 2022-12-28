use uuid::Uuid;
use crate::domain::types::audience::{Audience, AudienceId};
use crate::domain::types::command::{Command, CommandId};
use crate::domain::types::event::{Event, EventId};
use crate::domain::types::interface::{Interface, InterfaceId};
use crate::domain::types::placement::PlacementIndex;
use crate::domain::types::read_model::{ReadModel, ReadModelId};
use crate::domain::types::stream::{Stream, StreamId};

pub(crate) mod audience;
pub(crate) mod command;
pub(crate) mod event;
pub(crate) mod event_model;
pub(crate) mod flow;
pub(crate) mod interface;
pub(crate) mod placement;
pub(crate) mod read_model;
pub(crate) mod schema;
pub(crate) mod stream;
pub(crate) mod errors;

pub trait Entity {
    fn id(&self) -> &Uuid;
}

pub trait Named: Entity {
    fn name(&self) -> &str;
    fn rename(&mut self, name: &str);
}

// TODO: ensure non-blank strings
pub trait Described: Named {
    fn description(&self) -> Option<&str>;
    fn add_to_description(&mut self, index: u32, addition: &str);
    fn remove_from_description(&mut self, index: u32);
}

pub type LaneIndex = u32;

pub enum LaneId<'a> {
    AudienceLaneId(&'a AudienceId),
    StreamLaneId(&'a StreamId)
}

pub enum Lane<'a> {
    AudienceLane(&'a Audience),
    StreamLane(&'a Stream)
}

pub enum ComponentId<'a> {
    InterfaceComponentId(&'a InterfaceId),
    CommandComponentId(&'a CommandId),
    EventComponentId(&'a EventId),
    ReadModelComponentId(&'a ReadModelId),
}

pub enum Component<'a> {
    InterfaceComponent(&'a Interface),
    CommandComponent(&'a Command),
    EventComponent(&'a Event),
    ReadModelComponent(&'a ReadModel),
}

pub enum PlacementPosition<'a> {
    InterfacePosition(&'a PlacementIndex, Option<&'a AudienceId>),
    CommandPosition(&'a PlacementIndex),
    EventPosition(&'a PlacementIndex, Option<&'a StreamId>),
    ReadModelPosition(&'a PlacementIndex),
}