use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use crate::types::common::{Entity, Named};

pub type StreamId = Uuid;

#[derive(Debug, Serialize, Deserialize)]
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