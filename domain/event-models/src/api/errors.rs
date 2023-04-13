use serde::{Deserialize, Serialize};

use crate::types::LaneId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventModelError {
    IllegalState(String),
    InvalidNameError(String),
    CreationError(String),
    ModificationError(String),
    IllegalFlowArrow(String),
    SerializationError(String),
    LaneNotFound(LaneId),
}
