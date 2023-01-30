use crate::types::flow::{FlowArrow, FlowId};
use crate::types::interface::InterfaceConfig;
use crate::types::placement::{Placement, PlacementId, PlacementPosition};
use crate::types::{Component, ComponentId, Lane, LaneId};
use crate::EventModelId;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventModelEvent {
    Created(EventModelId, String),
    Renamed(EventModelId, String),
    AddedToDescription(EventModelId, u32, String),
    DeletedFromDescription(EventModelId, u32),
    Deleted(EventModelId),

    // Lanes
    LaneAdded(EventModelId, Lane),
    LaneRenamed(EventModelId, LaneId, String),
    LaneReordered(EventModelId, LaneId, u32),
    LaneRemoved(EventModelId, LaneId),

    // Grid
    ComponentDefined(EventModelId, Component),
    ComponentPlaced(EventModelId, Placement),
    ComponentRenamed(EventModelId, ComponentId, String),
    PlacementMoved(EventModelId, PlacementPosition),
    PlacementRemoved(EventModelId, PlacementId),
    ComponentRemoved(EventModelId, ComponentId),

    // Component Details
    AddedToComponentDescription(EventModelId, ComponentId, u32, String),
    DeletedFromComponentDescription(EventModelId, ComponentId, u32),
    InterfaceConfigured(EventModelId, InterfaceConfig),

    // Flows
    FlowConnected(EventModelId, FlowArrow),
    FlowDisconnected(EventModelId, FlowId),
    // TODO: Schemas
}
