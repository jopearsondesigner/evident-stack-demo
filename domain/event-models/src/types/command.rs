use std::collections::HashMap;

use crate::types::errors::EventModelError;
use crate::types::schema::{CommandSchemaRole, Schema, SubSchemaName};
use crate::types::{Described, Entity, Named};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use super::schema::{HasModifiableSchema, HasSchema};
use super::{ModifiablyDescribed, Renamable};

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
}

impl Renamable for Command {
    fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

impl Described for Command {
    fn description(&self) -> &str {
        &self.description
    }
}

impl ModifiablyDescribed for Command {
    fn set_description(&mut self, description: &str) {
        self.description = description.to_string();
    }

    fn add_to_description(&mut self, index: u32, addition: &str) {
        if self.description.is_empty() {
            self.set_description(addition);
        } else {
            self.description.insert_str(index as usize, addition);
        }
    }

    fn delete_from_description(&mut self, index: u32) {
        if !self.description.is_empty() {
            self.description.remove(index as usize);
        }
    }
}

impl HasSchema for Command {
    fn schema(&self) -> &Schema {
        &self.schema
    }
}

impl HasModifiableSchema for Command {
    fn schema_mut(&mut self) -> &mut Schema {
        &mut self.schema
    }

    fn set_schema(&mut self, schema: Schema) {
        self.schema = schema
    }
}
