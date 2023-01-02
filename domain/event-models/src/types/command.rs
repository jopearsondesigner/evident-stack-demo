use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use crate::types::{Described, Entity, Named};
use crate::types::errors::EventModelError;

pub type CommandId = Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Command {
    id: CommandId,
    name: String,
    description: Option<String>
}

impl Command {
    // TODO:
    pub fn new(id: Uuid, name: &str) -> Result<Self, EventModelError> {
        Ok(Command {
            id,
            name: name.to_string(),
            description: None,
        })
    }
}

impl Entity for Command {
    fn id(&self) -> &Uuid { &self.id }
}

impl Named for Command {
    fn name(&self) -> &str { &self.name }

    fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

impl Described for Command {
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn set_description(&mut self, description: &str) {
        if description.is_empty() {
            self.description = None
        } else {
            self.description = Some(description.to_string());
        }
    }
}
