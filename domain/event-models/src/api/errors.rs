use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventModelError {
    IllegalState(String),
    InvalidNameError(String),
    CreationError(String),
    ModificationError(String),
    IllegalFlowArrow(String),
    SerializationError(String),
}
