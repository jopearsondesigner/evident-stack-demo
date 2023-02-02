use crate::types::audience::AudienceId;
use crate::types::command::CommandId;
use crate::types::event::EventId;
use crate::types::interface::InterfaceId;
use crate::types::read_model::ReadModelId;
use crate::types::stream::StreamId;
use crate::types::Entity;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

pub type PlacementIndex = u32;
pub type PlacementId = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Placement {
    Interface(InterfacePlacement),
    Command(CommandPlacement),
    Event(EventPlacement),
    ReadModel(ReadModelPlacement),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimelinePlacement {
    Command(CommandPlacement),
    ReadModel(ReadModelPlacement),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlacementPosition {
    InterfacePosition(PlacementId, PlacementIndex, Option<AudienceId>),
    CommandPosition(PlacementId, PlacementIndex),
    EventPosition(PlacementId, PlacementIndex, Option<StreamId>),
    ReadModelPosition(PlacementId, PlacementIndex),
}

impl Entity for Placement {
    fn id(&self) -> &Uuid {
        match self {
            Placement::Interface(i) => &i.id,
            Placement::Command(c) => &c.id,
            Placement::Event(e) => &e.id,
            Placement::ReadModel(r) => &r.id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InterfacePlacement {
    id: PlacementId,
    index: PlacementIndex,
    interface: InterfaceId,
    audience: Option<AudienceId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandPlacement {
    id: PlacementId,
    index: PlacementIndex,
    command: CommandId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventPlacement {
    id: PlacementId,
    index: PlacementIndex,
    event: EventId,
    stream: Option<StreamId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadModelPlacement {
    id: PlacementId,
    index: PlacementIndex,
    read_model: ReadModelId,
}
