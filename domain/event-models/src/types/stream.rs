use uuid::Uuid;
use crate::types::{Entity, Named};

pub type StreamId = Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Stream {
    id: StreamId,
    name: String
}

impl Entity for Stream {
    fn id(&self) -> &Uuid {
        &self.id
    }
}

impl Named for Stream {
    fn name(&self) -> &str {
        &self.name
    }

    fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
