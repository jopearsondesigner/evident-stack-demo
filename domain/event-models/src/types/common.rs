use uuid::Uuid;

pub trait Entity {
    fn id(&self) -> &Uuid;
}

// TODO: ensure non-blank strings
pub trait Named: Entity {
    fn name(&self) -> &str;
}

// TODO: ensure non-blank strings
pub trait Described: Named {
    fn description(&self) -> &str;
}
