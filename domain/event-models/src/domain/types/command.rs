use uuid::Uuid;
use crate::domain::types::{Described, Entity, Named};
use crate::domain::types::errors::EventModelCreationError;

pub type CommandId = Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    id: CommandId,
    name: String,
    description: Option<String>
}

impl Command {
    // TODO:
    pub fn new(id: Uuid, name: &str) -> Result<Self, EventModelCreationError> {
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

    fn add_to_description(&mut self, index: u32, addition: &str) {
        todo!()
    }

    fn remove_from_description(&mut self, index: u32) {
        todo!()
    }
}
