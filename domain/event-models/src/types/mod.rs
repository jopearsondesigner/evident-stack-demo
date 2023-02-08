pub use crate::types::audience::{Audience, AudienceId};
pub use crate::types::command::{Command, CommandId};
pub use crate::types::errors::EventModelError;
pub use crate::types::event::{Event, EventId};
pub use crate::types::flow::{FlowArrow, FlowId};
pub use crate::types::interface::{Interface, InterfaceId};
pub use crate::types::placement::{Placement, PlacementId, PlacementPosition};
pub use crate::types::read_model::{ReadModel, ReadModelId};
pub use crate::types::schema::{CommandSchemaRole, EventSchemaRole, ReadModelSchemaRole, Schema};
pub use crate::types::stream::{Stream, StreamId};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

pub(crate) mod audience;
pub(crate) mod command;
pub(crate) mod errors;
pub(crate) mod event;
pub(crate) mod flow;
pub(crate) mod interface;
pub(crate) mod placement;
pub(crate) mod read_model;
pub(crate) mod schema;
pub(crate) mod stream;

pub trait Entity {
    fn id(&self) -> &Uuid;
}

pub fn validate_name(name: &str) -> Result<String, EventModelError> {
    if !name.is_empty() {
        Ok(name.to_string())
    } else {
        Err(EventModelError::InvalidNameError(
            "Name cannot be empty".to_string(),
        ))
    }
}

// Name cannot be an empty string
pub trait Named: Entity {
    fn name(&self) -> &str;
    fn rename(&mut self, name: &str);
}

// Description cannot be an empty string
pub trait Described: Named {
    fn description(&self) -> Option<&str>;
    fn set_description(&mut self, description: &str);
}

pub type LaneIndex = u32;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LaneId {
    DefaultAudience,
    Audience(AudienceId),
    Timeline,
    Stream(StreamId),
    DefaultStream,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Lane {
    Audience(Audience),
    Stream(Stream),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentId {
    InterfaceComponentId(InterfaceId),
    CommandComponentId(CommandId),
    EventComponentId(EventId),
    ReadModelComponentId(ReadModelId),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Component {
    InterfaceComponent(Interface),
    CommandComponent(Command),
    EventComponent(Event),
    ReadModelComponent(ReadModel),
}

#[derive(Debug)]
pub(crate) enum ComponentMut<'a> {
    InterfaceComponentMut(&'a mut Interface),
    CommandComponentMut(&'a mut Command),
    EventComponentMut(&'a mut Event),
    ReadModelComponentMut(&'a mut ReadModel),
}
