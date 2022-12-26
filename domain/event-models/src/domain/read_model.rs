use uuid::Uuid;
use crate::domain::common::{Described, Entity, Named};

pub type ReadModelId = Uuid;

#[derive(Debug)]
pub struct ReadModel {
    id: ReadModelId,
    name: String,
    description: String
}

impl Entity for ReadModel {
    fn id(&self) -> &Uuid { &self.id }
}

impl Named for ReadModel {
    fn name(&self) -> &str { &self.name }
}

impl Described for ReadModel {
    fn description(&self) -> &str {
        &self.description
    }
}
