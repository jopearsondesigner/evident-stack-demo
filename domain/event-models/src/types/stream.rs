use crate::types::{Entity, Named};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Renamable;

pub type StreamId = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stream {
    id: StreamId,
    name: String,
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

impl Renamable for Stream {
    fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
