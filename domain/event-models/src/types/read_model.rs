use crate::types::schema::Schema;
use crate::types::{Described, Entity, Named};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::schema::{HasModifiableSchema, HasSchema};
use super::{EventModelError, ModifiablyDescribed, Renamable};

pub type ReadModelId = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadModel {
    id: ReadModelId,
    name: String,
    description: String,
    schema: Schema,
}

impl ReadModel {
    pub fn create(
        id: Uuid,
        name: String,
        description: String,
        schema: Schema,
    ) -> Result<Self, EventModelError> {
        Ok(ReadModel {
            id,
            name,
            description,
            schema,
        })
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
}

impl Renamable for ReadModel {
    fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

impl Described for ReadModel {
    fn description(&self) -> &str {
        &self.description
    }
}

impl ModifiablyDescribed for ReadModel {
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
        &self.schema
    }
}

impl HasModifiableSchema for ReadModel {
    fn schema_mut(&mut self) -> &mut Schema {
        &mut self.schema
    }

    fn set_schema(&mut self, schema: Schema) {
        self.schema = schema
    }
}
