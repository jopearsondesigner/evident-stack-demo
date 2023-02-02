use crate::types::errors::EventModelError;
use crate::types::{Described, Entity, Named};
use serde_derive::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

pub type InterfaceId = Uuid;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterfaceConfig {
    #[default]
    None,
    Figma(Url, Option<u32>, Option<u32>),
    Image(Url, Option<u32>, Option<u32>),
    Job,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Interface {
    id: InterfaceId,
    name: String,
    description: Option<String>,
    config: InterfaceConfig,
}

impl Interface {
    pub fn new(id: Uuid, name: &str) -> Result<Self, EventModelError> {
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

    fn set_description(&mut self, description: &str) {
        if description.is_empty() {
            self.description = None
        } else {
            self.description = Some(description.to_string());
        }
    }
}
