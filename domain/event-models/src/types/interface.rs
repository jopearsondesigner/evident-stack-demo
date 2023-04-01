use crate::api::errors::EventModelError;
use crate::types::{Described, Entity, Named};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use super::{ModifiablyDescribed, Renamable};

pub type InterfaceId = Uuid;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterfaceConfig {
    #[default]
    Blank,
    Figma {
        url: Url,
        width: Option<usize>,
        height: Option<usize>,
    },
    Image {
        url: Url,
        width: Option<usize>,
        height: Option<usize>,
    },
    Job,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Interface {
    id: InterfaceId,
    name: String,
    description: String,
    config: InterfaceConfig,
}

impl Interface {
    pub fn create(
        id: Uuid,
        name: String,
        description: String,
        config: InterfaceConfig,
    ) -> Result<Self, EventModelError> {
        // TODO: validate name
        Ok(Interface {
            id,
            name,
            description,
            config,
        })
    }

    pub fn config(&self) -> &InterfaceConfig {
        &self.config
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
}

impl Renamable for Interface {
    fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

impl Described for Interface {
    fn description(&self) -> &str {
        &self.description
    }
}

impl ModifiablyDescribed for Interface {
    fn description_mut(&mut self) -> &mut String {
        &mut self.description
    }
}
