use url::Url;
use uuid::Uuid;
use crate::domain::types::{Described, Entity, Named};
use crate::domain::types::errors::EventModelModificationError;

pub type InterfaceId = Uuid;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum InterfaceConfig {
    #[default]
    None,
    Figma(Url),
    Image(Url),
    Job
}

#[derive(Debug, Clone, PartialEq)]
pub struct Interface {
    id: InterfaceId,
    name: String,
    description: Option<String>,
    config: InterfaceConfig,
}

impl Interface {
    pub fn new(id: Uuid, name: &str) -> Result<Self, EventModelModificationError> {
        // TODO: validate name
        Ok(Interface {
            id,
            name: name.to_string(),
            description: None,
            config: Default::default(),
        })
    }
}

impl Entity for Interface {
    fn id(&self) -> &Uuid {
        &self.id
    }
}

impl Named for Interface {
    fn name(&self) -> &str {
        &self.name
    }

    fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

impl Described for Interface {
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
