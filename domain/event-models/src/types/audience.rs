use crate::types::{Entity, Named};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Renamable;

pub type AudienceId = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
}

impl Renamable for Audience {
    fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
