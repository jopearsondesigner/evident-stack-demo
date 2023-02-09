use std::collections::HashMap;

use crate::types::schema::{CommandSchemaRole, Schema, SubSchemaName};
use crate::types::{Described, Entity, Named};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use super::schema::HasSchema;

pub type ReadModelId = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadModel {
    id: ReadModelId,
    name: String,
    description: String,
    schema: Schema,
    schema_roles: HashMap<CommandSchemaRole, SubSchemaName>,
}

impl ReadModel {
    pub fn new(id: Uuid, name: &str) -> Self {
        ReadModel {
            id,
            name: name.to_string(),
            description: Default::default(),
            schema: Default::default(),
            schema_roles: Default::default(),
        }
    }
}

impl Entity for ReadModel {
    fn id(&self) -> &Uuid {
        &self.id
    }
}

impl Named for ReadModel {
    fn name(&self) -> &str {
        &self.name
    }

    fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

impl Described for ReadModel {
    fn description(&self) -> &str {
        &self.description
    }

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

impl HasSchema for ReadModel {
    fn schema(&self) -> &Schema {
        todo!()
    }

    fn set_schema(&mut self, schema: Schema) {
        todo!()
    }
}
