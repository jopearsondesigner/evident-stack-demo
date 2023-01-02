#[derive(Debug, Clone, PartialEq)]
pub enum EventModelError {
    IllegalState(String),
    InvalidNameError(String),
    CreationError(String),
    ModificationError(String),
    IllegalFlowArrow(String)
}
