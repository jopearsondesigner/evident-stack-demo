use uuid::Uuid;
use crate::domain::common::{Described, Entity, Named};

pub type CommandId = Uuid;

#[derive(Debug)]
pub struct Command {
    id: CommandId,
    name: String,
    description: String
}

impl Entity for Command {
    fn id(&self) -> &Uuid { &self.id }
}

impl Named for Command {
    fn name(&self) -> &str { &self.name }
}

impl Described for Command {
    fn description(&self) -> &str {
        &self.description
    }
}
