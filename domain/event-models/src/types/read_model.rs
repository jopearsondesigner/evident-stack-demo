use crate::types::{Described, Entity, Named};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

pub type ReadModelId = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadModel {
    id: ReadModelId,
    name: String,
    description: Option<String>,
}

impl ReadModel {
    pub fn new(id: Uuid, name: &str) -> Self {
        ReadModel {
            id,
            name: name.to_string(),
            description: None,
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
