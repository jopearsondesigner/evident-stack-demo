use crate::types::audience::AudienceId;
use crate::types::command::CommandId;
use crate::types::event::EventId;
use crate::types::interface::InterfaceId;
use crate::types::read_model::ReadModelId;
use crate::types::schema::Schema;
use crate::types::stream::StreamId;
use crate::types::Entity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{ComponentId, LaneId};

pub type PlacementIndex = usize;
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
    Interface {
        id: PlacementId,
        index: PlacementIndex,
        interface: InterfaceId,
        audience: Option<AudienceId>,
    },
    Command {
        id: PlacementId,
        index: PlacementIndex,
        command: CommandId,
        schema: Schema,
    },
    Event {
        id: PlacementId,
        index: PlacementIndex,
        event: EventId,
        stream: Option<StreamId>,
        schema: Schema,
    },
    ReadModel {
        id: PlacementId,
        index: PlacementIndex,
        read_model: ReadModelId,
        schema: Schema,
    },
}

impl Placement {
    pub fn index(&self) -> &PlacementIndex {
        match self {
            Placement::Interface { index, .. } => &index,
            Placement::Command { index, .. } => &index,
            Placement::Event { index, .. } => &index,
            Placement::ReadModel { index, .. } => &index,
        }
    }

    pub fn lane(&self) -> LaneId {
        match self {
            Placement::Interface { audience, .. } => match audience {
                Some(id) => LaneId::Audience(*id),
                None => LaneId::DefaultAudience,
            },
            Placement::Command { .. } => LaneId::Timeline,
            Placement::Event { stream, .. } => match stream {
                Some(id) => LaneId::Stream(*id),
                None => LaneId::DefaultStream,
            },
            Placement::ReadModel { .. } => LaneId::Timeline,
        }
    }

    pub fn relocate(&mut self, idx: PlacementIndex, lane: LaneId) {
        match self {
            Placement::Interface {
                index, audience, ..
            } => {
                *index = idx;
                match lane {
                    LaneId::DefaultAudience => *audience = None,
                    LaneId::Audience(id) => *audience = Some(id),
                    _ => (),
                };
            }
            Placement::Command { index, .. } => *index = idx,
            Placement::Event { index, stream, .. } => {
                *index = idx;
                match lane {
                    LaneId::Stream(id) => *stream = Some(id),
                    LaneId::DefaultStream => *stream = None,
                    _ => (),
                }
            }
            Placement::ReadModel { index, .. } => *index = idx,
        }
    }

    pub fn component_id(&self) -> ComponentId {
        match self {
            Placement::Interface { interface, .. } => {
                ComponentId::InterfaceComponentId(interface.to_owned())
            }
            Placement::Command { command, .. } => {
                ComponentId::CommandComponentId(command.to_owned())
            }
            Placement::Event { event, .. } => ComponentId::EventComponentId(event.to_owned()),
            Placement::ReadModel { read_model, .. } => {
                ComponentId::ReadModelComponentId(read_model.to_owned())
            }
        }
    }
}

impl Entity for Placement {
    fn id(&self) -> &Uuid {
        match self {
            Placement::Interface { id, .. } => &id,
            Placement::Command { id, .. } => &id,
            Placement::Event { id, .. } => &id,
            Placement::ReadModel { id, .. } => &id,
        }
    }
}
