use url::Url;
use uuid::Uuid;
use crate::domain::common::{Described, Entity, Named};

pub type InterfaceId = Uuid;

#[derive(Debug)]
pub enum InterfaceConfig {
    Figma(Url),
    Image(Url),
    Job
}

#[derive(Debug)]
pub struct Interface {
    id: InterfaceId,
    config: InterfaceConfig,
    name: String,
    description: String
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

impl Described for Interface {
    fn description(&self) -> &str {
        &self.description
    }
}
