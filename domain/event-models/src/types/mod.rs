pub use crate::api::errors::EventModelError;
pub use crate::types::audience::{Audience, AudienceId};
pub use crate::types::command::{Command, CommandId};
pub use crate::types::event::{Event, EventId};
pub use crate::types::flow::{FlowArrow, FlowId};
pub use crate::types::interface::{Interface, InterfaceConfig, InterfaceId};
pub use crate::types::placement::{Placement, PlacementId, PlacementPosition};
pub use crate::types::read_model::{ReadModel, ReadModelId};
pub use crate::types::schema::Schema;
pub use crate::types::stream::{Stream, StreamId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub(crate) mod audience;
pub(crate) mod command;
pub(crate) mod event;
pub mod flow;
pub(crate) mod interface;
pub(crate) mod placement;
pub(crate) mod read_model;
pub mod schema;
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
}

pub trait Renamable: Named {
    fn rename(&mut self, name: &str);
}

// Description cannot be an empty string
pub trait Described: Named {
    fn description(&self) -> &str;
}

pub trait ModifiablyDescribed: Described {
    fn description_mut(&mut self) -> &mut String;

    fn set_description(&mut self, description: &str) {
        let mut desc = self.description_mut();
        *desc = description.to_string();
    }

    fn add_to_description(&mut self, index: usize, addition: &str) {
        let mut desc = self.description_mut();
        if desc.is_empty() {
            self.set_description(addition);
        } else {
            desc.insert_str(index, addition);
        }
    }

    fn delete_from_description(&mut self, index: usize, count: usize) {
        let mut desc = self.description_mut();
        if !desc.is_empty() {
            for i in 0..count {
                desc.remove(index + i);
            }
        }
    }
}

pub type LaneIndex = usize;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
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
