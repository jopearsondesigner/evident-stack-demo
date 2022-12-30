use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use crate::types::audience::{Audience, AudienceId};
use crate::types::command::{Command, CommandId};
use crate::types::event::{Event, EventId};
use crate::types::interface::{Interface, InterfaceId};
use crate::types::placement::PlacementIndex;
use crate::types::read_model::{ReadModel, ReadModelId};
use crate::types::stream::{Stream, StreamId};

pub(crate) mod audience;
pub(crate) mod command;
pub(crate) mod event;
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
    fn set_description(&mut self, description: &str);
}

pub type LaneIndex = u32;

#[derive(Debug, Clone, PartialEq)]
pub enum LaneId<'a> {
    AudienceLaneId(&'a AudienceId),
    StreamLaneId(&'a StreamId)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lane<'a> {
    AudienceLane(&'a Audience),
    StreamLane(&'a Stream)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComponentId {
    InterfaceComponentId(InterfaceId),
    CommandComponentId(CommandId),
    EventComponentId(EventId),
    ReadModelComponentId(ReadModelId),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Component<'a> {
    InterfaceComponent(&'a Interface),
    CommandComponent(&'a Command),
    EventComponent(&'a Event),
    ReadModelComponent(&'a ReadModel),
}

#[derive(Debug)]
pub enum ComponentMut<'a> {
    InterfaceComponentMut(&'a mut Interface),
    CommandComponentMut(&'a mut Command),
    EventComponentMut(&'a mut Event),
    ReadModelComponentMut(&'a mut ReadModel),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlacementPosition<'a> {
    InterfacePosition(&'a PlacementIndex, Option<&'a AudienceId>),
    CommandPosition(&'a PlacementIndex),
    EventPosition(&'a PlacementIndex, Option<&'a StreamId>),
    ReadModelPosition(&'a PlacementIndex),
}