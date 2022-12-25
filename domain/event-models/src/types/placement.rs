use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use crate::types::common::Entity;
use crate::types::command::CommandId;
use crate::types::event::EventId;
use crate::types::interface::InterfaceId;
use crate::types::read_model::ReadModelId;
use crate::types::audience::AudienceId;
use crate::types::stream::StreamId;

pub type PlacementIndex = u32;
pub type PlacementId = Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum Placement {
    Interface(InterfacePlacement),
    Command(CommandPlacement),
    Event(EventPlacement),
    ReadModel(ReadModelPlacement)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TimelinePlacement {
    Command(CommandPlacement),
    ReadModel(ReadModelPlacement)
}

impl Entity for Placement {
    fn id(&self) -> &Uuid {
        match self {
            Placement::Interface(i) => &i.id,
            Placement::Command(c) => &c.id,
            Placement::Event(e) => &e.id,
            Placement::ReadModel(r) => &r.id
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InterfacePlacement {
    id: PlacementId,
    index: PlacementIndex,
    interface: InterfaceId,
    audience: Option<AudienceId>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandPlacement {
    id: PlacementId,
    index: PlacementIndex,
    command: CommandId
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventPlacement {
    id: PlacementId,
    index: PlacementIndex,
    event: EventId,
    stream: Option<StreamId>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadModelPlacement {
    id: PlacementId,
    index: PlacementIndex,
    read_model: ReadModelId,
}
