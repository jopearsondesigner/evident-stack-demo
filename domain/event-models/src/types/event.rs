use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use crate::types::common::{Described, Entity, Named};

pub type EventId = Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    id: EventId,
    name: String,
    description: String
}

impl Entity for Event {
    fn id(&self) -> &Uuid { &self.id }
}

impl Named for Event {
    fn name(&self) -> &str { &self.name }
}

impl Described for Event {
    fn description(&self) -> &str {
        &self.description
    }
}
