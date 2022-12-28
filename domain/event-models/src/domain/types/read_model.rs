use uuid::Uuid;
use crate::domain::types::{Described, Entity, Named};

pub type ReadModelId = Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct ReadModel {
    id: ReadModelId,
    name: String,
    description: Option<String>
}

impl ReadModel {
    pub fn new(id: Uuid, name: &str) -> Self {
        ReadModel {
            id,
            name: name.to_string(),
            description: None,
        }
    }
}

impl Entity for ReadModel {
    fn id(&self) -> &Uuid { &self.id }
}

impl Named for ReadModel {
    fn name(&self) -> &str { &self.name }

    fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

impl Described for ReadModel {
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn add_to_description(&mut self, index: u32, addition: &str) {
        todo!()
    }

    fn remove_from_description(&mut self, index: u32) {
        todo!()
    }
}
