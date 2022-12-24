use uuid::Uuid;
use crate::types::common::{Entity, Named};

pub type StreamId = Uuid;

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
}