use uuid::Uuid;
use crate::domain::common::{Entity, Named};

pub type AudienceId = Uuid;

#[derive(Debug)]
pub struct Audience {
    id: AudienceId,
    name: String
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
