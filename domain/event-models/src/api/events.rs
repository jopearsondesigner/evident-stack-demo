use crate::types::flow::{FlowArrow, FlowId};
use crate::types::interface::InterfaceConfig;
use crate::types::placement::{Placement, PlacementId, PlacementPosition};
use crate::types::{Component, ComponentId, Lane, LaneId};
use crate::EventModelId;
use epoch::decider::Event;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventModelEvent {
    Created(EventModelId, String),
    Renamed(EventModelId, String),
    DescriptionSet(EventModelId, String),
    AddedToDescription(EventModelId, u32, String),
    DeletedFromDescription(EventModelId, u32),
    SchemaSet(EventModelId, String),
    AddedToSchema(EventModelId, u32, String),
    DeletedFromSchema(EventModelId, u32),
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
    ComponentDescriptionSet(EventModelId, ComponentId, String),
    AddedToComponentDescription(EventModelId, ComponentId, u32, String),
    DeletedFromComponentDescription(EventModelId, ComponentId, u32),
    ComponentSchemaSet(EventModelId, ComponentId, String),
    AddedToComponentSchema(EventModelId, ComponentId, u32, String),
    DeletedFromComponentSchema(EventModelId, ComponentId, u32),
    InterfaceConfigured(EventModelId, InterfaceConfig),

    // Flows
    FlowConnected(EventModelId, FlowArrow),
    FlowDisconnected(EventModelId, FlowId),
}

impl Event for EventModelEvent {
    type EntityId = String;

    fn event_type(&self) -> String {
        todo!()
    }

    fn get_id(&self) -> Self::EntityId {
        todo!()
    }
}
