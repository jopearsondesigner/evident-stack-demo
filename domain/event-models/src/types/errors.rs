#[derive(Debug, Clone, PartialEq)]
pub enum EventModelError {
    CreationError(String),
    ModificationError(String)
}
