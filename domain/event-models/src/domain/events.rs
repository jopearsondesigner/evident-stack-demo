use serde_derive::{Deserialize, Serialize};
use crate::EventModelId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventModelEvent {
    Created(EventModelId, String),
    Renamed(EventModelId, String),
}