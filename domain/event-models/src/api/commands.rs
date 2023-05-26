use crate::{
    Anchor, AudienceId, ComponentId, EventModelId, FlowId, InterfaceConfig, PlacementId, StreamId, ColumnShift,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventModelCommand {
    // Event Model Details
    Create(String),
    Rename(EventModelId, String),
    Delete(EventModelId),
    EditDescription(EventModelId, usize, usize, String),
    EditSchema(EventModelId, usize, usize, String),

    // TODO: remove these:
    SetDescription(EventModelId, String),
    AddToDescription(EventModelId, usize, String),
    DeleteFromDescription(EventModelId, usize, usize),
    SetSchema(EventModelId, String),
    AddToSchema(EventModelId, usize, String),
    DeleteFromSchema(EventModelId, usize),
    // </TODO: remove these>

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
    ShiftPlacements(EventModelId, usize, ColumnShift),

    // Clipboard
    DuplicateInterfacePlacement(EventModelId, PlacementId, usize, Option<AudienceId>),
    DuplicateTimelinePlacement(EventModelId, PlacementId, usize),
    DuplicateEventPlacement(EventModelId, PlacementId, usize, Option<StreamId>),

    // Component Details
    RenameComponent(EventModelId, ComponentId, String),
    ConfigureInterface(EventModelId, ComponentId, InterfaceConfig),
    EditComponentDescription(EventModelId, ComponentId, usize, usize, String),
    EditComponentSchema(EventModelId, ComponentId, usize, usize, String),

    // TODO: remove these:
    SetComponentDescription(EventModelId, ComponentId, String),
    AddToComponentDescription(EventModelId, ComponentId, usize, String),
    DeleteFromComponentDescription(EventModelId, ComponentId, usize),
    SetComponentSchema(EventModelId, ComponentId, String),
    AddToComponentSchema(EventModelId, ComponentId, usize, String),
    DeleteFromComponentSchema(EventModelId, ComponentId, usize),
    // </TODO: remove these>

    // Placement Details
    EditPlacementSchema(EventModelId, PlacementId, usize, usize, String),

    // Flows
    ConnectFlow(EventModelId, PlacementId, Anchor, PlacementId, Anchor),
    DisconnectFlow(EventModelId, FlowId),
}
