use uuid::Uuid;
use crate::domain::common::{Entity, Named};

pub type StreamId = Uuid;

#[derive(Debug)]
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
