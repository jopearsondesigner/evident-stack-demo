use std::collections::HashMap;

use crate::types::audience::AudienceId;
use crate::types::command::CommandId;
use crate::types::event::EventId;
use crate::types::interface::InterfaceId;
use crate::types::read_model::ReadModelId;
use crate::types::schema::{CommandSchemaRole, Schema, SubSchemaName};
use crate::types::stream::StreamId;
use crate::types::Entity;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use super::LaneId;

pub type PlacementIndex = u32;
pub type PlacementId = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlacementPosition(pub PlacementId, pub PlacementIndex, pub LaneId);

impl Entity for PlacementPosition {
    fn id(&self) -> &Uuid {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Placement {
    Interface(InterfacePlacement),
    Command(CommandPlacement),
    Event(EventPlacement),
    ReadModel(ReadModelPlacement),
}

impl Placement {
    pub fn index(&self) -> &PlacementIndex {
        match self {
            Placement::Interface(i) => &i.index,
            Placement::Command(c) => &c.index,
            Placement::Event(e) => &e.index,
            Placement::ReadModel(r) => &r.index,
        }
    }

    pub fn lane(&self) -> LaneId {
        match self {
            Placement::Interface(i) => match i.audience {
                Some(id) => LaneId::Audience(id),
                None => LaneId::DefaultAudience,
            },
            Placement::Command(_) => LaneId::Timeline,
            Placement::Event(e) => match e.stream {
                Some(id) => LaneId::Stream(id),
                None => LaneId::DefaultStream,
            },
            Placement::ReadModel(_) => LaneId::Timeline,
        }
    }

    pub fn relocate(&mut self, index: PlacementIndex, lane: LaneId) {
        match self {
            Placement::Interface(i) => {
                i.index = index;
                match lane {
                    LaneId::DefaultAudience => i.audience = None,
                    LaneId::Audience(id) => i.audience = Some(id),
                    _ => (),
                };
            }
            Placement::Command(c) => c.index = index,
            Placement::Event(e) => {
                e.index = index;
                match lane {
                    LaneId::Stream(id) => e.stream = Some(id),
                    LaneId::DefaultStream => e.stream = None,
                    _ => (),
                }
            }
            Placement::ReadModel(r) => r.index = index,
        }
    }
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
    schema: Schema,
    schema_roles: HashMap<CommandSchemaRole, SubSchemaName>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventPlacement {
    id: PlacementId,
    index: PlacementIndex,
    event: EventId,
    stream: Option<StreamId>,
    schema: Schema,
    schema_roles: HashMap<CommandSchemaRole, SubSchemaName>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadModelPlacement {
    id: PlacementId,
    index: PlacementIndex,
    read_model: ReadModelId,
    schema: Schema,
    schema_roles: HashMap<CommandSchemaRole, SubSchemaName>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimelinePlacement {
    Command(CommandPlacement),
    ReadModel(ReadModelPlacement),
}
