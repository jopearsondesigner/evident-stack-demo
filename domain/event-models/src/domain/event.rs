use uuid::Uuid;
use crate::domain::common::{Described, Entity, Named};

pub type EventId = Uuid;

#[derive(Debug)]
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
