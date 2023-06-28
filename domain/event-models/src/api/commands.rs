use crate::{
    Anchor, AudienceId, ColumnShift, ComponentId, EventModelId, FlowId, PlacementId, StreamId,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventModelCommand {
    // Event Model Details
    Create(String),
    Rename(EventModelId, String),
    Delete(EventModelId),
    EditDescription(EventModelId, usize, usize, String),
    EditData(EventModelId, usize, usize, String),

    // Composite Actions
    Import(EventModelId, usize, Vec<u8>),

    // Lanes
    AddAudience(EventModelId, usize, String),
    RenameAudience(EventModelId, AudienceId, String),
    ReorderAudience(EventModelId, AudienceId, usize),
    RemoveAudience(EventModelId, AudienceId),
    AddStream(EventModelId, usize, String),
    RenameStream(EventModelId, StreamId, String),
    ReorderStream(EventModelId, StreamId, usize),
    RemoveStream(EventModelId, StreamId),

    // Canvas
    DefineAndPlaceInterface(EventModelId, String, usize, Option<AudienceId>),
    DefineAndPlaceCommand(EventModelId, String, usize),
    DefineAndPlaceEvent(EventModelId, String, usize, Option<StreamId>),
    DefineAndPlaceReadModel(EventModelId, String, usize),
    RenamePlacement(EventModelId, PlacementId, String),
    MoveInterfacePlacement(EventModelId, PlacementId, usize, Option<AudienceId>),
    MoveTimelinePlacement(EventModelId, PlacementId, usize),
    MoveEventPlacement(EventModelId, PlacementId, usize, Option<StreamId>),
    RemovePlacement(EventModelId, PlacementId),
    ShiftPlacements(EventModelId, ColumnShift),

    // Clipboard
    DuplicateInterfacePlacement(EventModelId, PlacementId, usize, Option<AudienceId>),
    DuplicateTimelinePlacement(EventModelId, PlacementId, usize),
    DuplicateEventPlacement(EventModelId, PlacementId, usize, Option<StreamId>),

    // Component Details
    RenameComponent(EventModelId, ComponentId, String),
    ConfigureInterface(EventModelId, ComponentId, String, Option<String>),
    EditComponentDescription(EventModelId, ComponentId, usize, usize, String),
    EditComponentData(EventModelId, ComponentId, usize, usize, String),

    // Placement Details
    EditPlacementData(EventModelId, PlacementId, usize, usize, String),

    // Flows
    ConnectFlow(EventModelId, PlacementId, Anchor, PlacementId, Anchor),
    DisconnectFlow(EventModelId, FlowId),
}
