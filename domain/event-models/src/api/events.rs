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
    AddedToDescription(EventModelId, usize, String),
    DeletedFromDescription(EventModelId, usize, usize),
    SchemaSet(EventModelId, String),
    AddedToSchema(EventModelId, usize, String),
    DeletedFromSchema(EventModelId, usize, usize),
    Deleted(EventModelId),

    // Lanes
    LaneAdded(EventModelId, Lane, usize),
    LaneRenamed(EventModelId, LaneId, String),
    LaneReordered(EventModelId, LaneId, usize),
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
    AddedToComponentDescription(EventModelId, ComponentId, usize, String),
    DeletedFromComponentDescription(EventModelId, ComponentId, usize),
    ComponentSchemaSet(EventModelId, ComponentId, String),
    AddedToComponentSchema(EventModelId, ComponentId, usize, String),
    DeletedFromComponentSchema(EventModelId, ComponentId, usize),
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
