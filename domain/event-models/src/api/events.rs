use crate::{
    Component, ComponentId, EventModelId, FlowArrow, FlowId, InterfaceConfig, Lane, LaneId, Name,
    Placement, PlacementId, PlacementPosition, TextEdit,
};
use epoch::decider::Event;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventModelEvent {
    Created(EventModelId, Name),
    Renamed(EventModelId, Name),
    DescriptionEdited(EventModelId, TextEdit),
    SchemaEdited(EventModelId, TextEdit),
    Deleted(EventModelId),

    // TODO: remove these:
    DescriptionSet(EventModelId, String),
    AddedToDescription(EventModelId, usize, String),
    DeletedFromDescription(EventModelId, usize, usize),
    SchemaSet(EventModelId, String),
    AddedToSchema(EventModelId, usize, String),
    DeletedFromSchema(EventModelId, usize, usize),
    // </TODO: remove these>

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
    ComponentSchemaEdited(EventModelId, ComponentId, TextEdit),
    InterfaceConfigured(EventModelId, ComponentId, InterfaceConfig),

    // TODO: remove these:
    ComponentDescriptionSet(EventModelId, ComponentId, String),
    AddedToComponentDescription(EventModelId, ComponentId, usize, String),
    DeletedFromComponentDescription(EventModelId, ComponentId, usize),
    ComponentSchemaSet(EventModelId, ComponentId, String),
    AddedToComponentSchema(EventModelId, ComponentId, usize, String),
    DeletedFromComponentSchema(EventModelId, ComponentId, usize),
    // </TODO: remove these>

    // Placement Details
    PlacementSchemaEdited(EventModelId, TextEdit),

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
