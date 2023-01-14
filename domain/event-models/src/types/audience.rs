use crate::types::{Entity, Named};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

pub type AudienceId = Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Audience {
    id: AudienceId,
    name: String,
}

impl Entity for Audience {
    fn id(&self) -> &Uuid {
        &self.id
    }
}

impl Named for Audience {
    fn name(&self) -> &str {
        &self.name
    }

    fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
