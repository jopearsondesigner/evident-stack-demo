use std::ops::Deref;
use url::Url;
use uuid::Uuid;
use crate::types::common::{Described, Entity, Named};
use crate::types::text::Text;

pub type InterfaceId = Uuid;

pub enum InterfaceConfig {
    Figma(Url),
    Image(Url),
    Job
}

pub struct Interface {
    id: InterfaceId,
    config: InterfaceConfig,
    name: String,
    description: Box<dyn Text>
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
        let desc = &self.description;
        desc.deref().into()
    }
}