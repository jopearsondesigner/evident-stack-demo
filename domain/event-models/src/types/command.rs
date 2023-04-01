use crate::api::errors::EventModelError;
use crate::types::schema::Schema;
use crate::types::{Described, Entity, Named};
use serde::{Deserialize, Serialize};
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
}

impl Command {
    pub fn create(
        id: Uuid,
        name: String,
        description: String,
        schema: Schema,
    ) -> Result<Self, EventModelError> {
        Ok(Command {
            id,
            name,
            description,
            schema,
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
    fn description_mut(&mut self) -> &mut String {
        &mut self.description
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
