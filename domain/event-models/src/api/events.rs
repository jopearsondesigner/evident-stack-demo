use crate::{
    Component, ComponentId, EventModelId, FlowArrow, FlowId, InterfaceConfig, InterfaceId, Lane,
    LaneId, Name, Placement, PlacementId, PlacementPosition, TextEdit,
};
use epoch::decider::Event;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventModelEvent {
    Created(EventModelId, Name),
    Renamed(EventModelId, Name),
    DescriptionEdited(EventModelId, TextEdit),
    DataEdited(EventModelId, TextEdit),
    Deleted(EventModelId),

    // Lanes
    LaneAdded(EventModelId, Lane, usize),
    LaneRenamed(EventModelId, LaneId, Name),
    LaneReordered(EventModelId, LaneId, usize),
    LaneRemoved(EventModelId, LaneId),

    // Grid
    ComponentDefined(EventModelId, Component),
    ComponentPlaced(EventModelId, Placement),
    ComponentRenamed(EventModelId, ComponentId, Name),
    PlacementMoved(EventModelId, PlacementPosition),
    PlacementRemoved(EventModelId, PlacementId),
    ComponentRemoved(EventModelId, ComponentId),
    PlacementsShifted(EventModelId, usize, usize),

    // Component Details
    ComponentDescriptionEdited(EventModelId, ComponentId, TextEdit),
    ComponentDataEdited(EventModelId, ComponentId, TextEdit),
    InterfaceConfigured(EventModelId, InterfaceId, InterfaceConfig),

    // Placement Details
    PlacementDataEdited(EventModelId, TextEdit),

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
