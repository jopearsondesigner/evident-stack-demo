use uuid::Uuid;
use crate::domain::common::Entity;
use crate::domain::command::CommandId;
use crate::domain::event::EventId;
use crate::domain::interface::InterfaceId;
use crate::domain::read_model::ReadModelId;
use crate::domain::audience::AudienceId;
use crate::domain::stream::StreamId;

pub type PlacementIndex = u32;
pub type PlacementId = Uuid;

#[derive(Debug)]
pub enum Placement {
    Interface(InterfacePlacement),
    Command(CommandPlacement),
    Event(EventPlacement),
    ReadModel(ReadModelPlacement)
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct InterfacePlacement {
    id: PlacementId,
    index: PlacementIndex,
    interface: InterfaceId,
    audience: Option<AudienceId>
}

#[derive(Debug)]
pub struct CommandPlacement {
    id: PlacementId,
    index: PlacementIndex,
    command: CommandId
}

#[derive(Debug)]
pub struct EventPlacement {
    id: PlacementId,
    index: PlacementIndex,
    event: EventId,
    stream: Option<StreamId>
}

#[derive(Debug)]
pub struct ReadModelPlacement {
    id: PlacementId,
    index: PlacementIndex,
    read_model: ReadModelId,
}
