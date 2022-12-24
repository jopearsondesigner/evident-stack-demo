use std::ops::Deref;
use uuid::Uuid;
use crate::types::common::{Described, Entity, Named};
use crate::types::text::Text;

pub type ReadModelId = Uuid;

pub struct ReadModel {
    id: ReadModelId,
    name: String,
    description: Box<dyn Text>
}

impl Entity for ReadModel {
    fn id(&self) -> &Uuid { &self.id }
}

impl Named for ReadModel {
    fn name(&self) -> &str { &self.name }
}

impl Described for ReadModel {
    fn description(&self) -> &str {
        let desc = &self.description;
        desc.deref().into()
    }
}