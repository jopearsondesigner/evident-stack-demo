use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventModelError {
    IllegalState(String),
    InvalidNameError(String),
    CreationError(String),
    ModificationError(String),
    IllegalFlowArrow(String),
}
