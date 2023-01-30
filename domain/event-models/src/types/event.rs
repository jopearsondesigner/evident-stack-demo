use crate::types::{Described, Entity, Named};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

pub type EventId = Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    id: EventId,
    name: String,
    description: Option<String>,
}

impl Event {
    pub fn new(id: Uuid, name: &str) -> Self {
        Event {
            id,
            name: name.to_string(),
            description: None,
        }
    }
}

impl Entity for Event {
    fn id(&self) -> &Uuid {
        &self.id
    }
}

impl Named for Event {
    fn name(&self) -> &str {
        &self.name
    }

    fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

impl Described for Event {
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
