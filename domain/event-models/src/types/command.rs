use std::collections::HashMap;

use crate::types::errors::EventModelError;
use crate::types::schema::{CommandSchemaRole, Schema, SubSchemaName};
use crate::types::{Described, Entity, Named};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

pub type CommandId = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Command {
    id: CommandId,
    name: String,
    description: String,
    schema: Schema,
    schema_roles: HashMap<CommandSchemaRole, SubSchemaName>,
}

impl Command {
    // TODO:
    pub fn new(id: Uuid, name: &str) -> Result<Self, EventModelError> {
        Ok(Command {
            id,
            name: name.to_string(),
            description: Default::default(),
            schema: Default::default(),
            schema_roles: Default::default(),
        })
    }
}

impl Entity for Command {
    fn id(&self) -> &Uuid {
        &self.id
    }
}

impl Named for Command {
    fn name(&self) -> &str {
        &self.name
    }

    fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

impl Described for Command {
    fn description(&self) -> &str {
        &self.description
    }

    fn set_description(&mut self, description: &str) {
        self.description = description.to_string();
    }
}
