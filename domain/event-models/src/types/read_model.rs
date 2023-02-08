use std::collections::HashMap;

use crate::types::schema::{CommandSchemaRole, Schema, SubSchemaName};
use crate::types::{Described, Entity, Named};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

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
}
