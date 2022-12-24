use std::ops::Deref;
use uuid::Uuid;
use crate::types::common::{Described, Entity, Named};
use crate::types::text::Text;

pub type EventId = Uuid;

pub struct Event {
    id: EventId,
    name: String,
    description: Box<dyn Text>
}

impl Entity for Event {
    fn id(&self) -> &Uuid { &self.id }
}

impl Named for Event {
    fn name(&self) -> &str { &self.name }
}

impl Described for Event {
    fn description(&self) -> &str {
        let desc = &self.description;
        desc.deref().into()
    }
}