use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use crate::types::common::{Described, Entity, Named};

pub type ReadModelId = Uuid;

#[derive(Debug, Serialize, Deserialize)]
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
