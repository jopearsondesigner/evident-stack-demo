use std::ops::Deref;
use uuid::Uuid;
use crate::types::common::{Described, Entity, Named};
use crate::types::text::Text;

pub type CommandId = Uuid;

pub struct Command {
    id: CommandId,
    name: String,
    description: Box<dyn Text>
}

impl Entity for Command {
    fn id(&self) -> &Uuid { &self.id }
}

impl Named for Command {
    fn name(&self) -> &str { &self.name }
}

impl Described for Command {
    fn description(&self) -> &str {
        let desc = &self.description;
        desc.deref().into()
    }
}