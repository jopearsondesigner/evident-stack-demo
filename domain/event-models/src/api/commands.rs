use crate::json::JsonImport;
use crate::types::audience::AudienceId;
use crate::types::flow::{Anchor, FlowId};
use crate::types::interface::InterfaceConfig;
use crate::types::placement::PlacementId;
use crate::types::stream::StreamId;
use crate::types::ComponentId;
use crate::EventModelId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventModelCommand {
    // Event Model Details
    Create(String),
    Rename(EventModelId, String),
    SetDescription(EventModelId, String),
    AddToDescription(EventModelId, u32, String),
    DeleteFromDescription(EventModelId, u32),
    SetSchema(EventModelId, String),
    AddToSchema(EventModelId, u32, String),
    DeleteFromSchema(EventModelId, u32),
    Delete(EventModelId),

    // Composite Actions
    Import(Box<JsonImport>),

    // Lanes
    AddAudience(EventModelId, u32, String),
    RenameAudience(EventModelId, AudienceId, String),
    ReorderAudience(EventModelId, AudienceId, u32),
    RemoveAudience(EventModelId, AudienceId),
    AddStream(EventModelId, u32, String),
    RenameStream(EventModelId, StreamId, String),
    ReorderStream(EventModelId, StreamId, u32),
    RemoveStream(EventModelId, StreamId),

    // Canvas
    DefineAndPlaceInterface(EventModelId, String, u32, Option<AudienceId>),
    DefineAndPlaceCommand(EventModelId, String, u32),
    DefineAndPlaceEvent(EventModelId, String, u32, Option<StreamId>),
    DefineAndPlaceReadModel(EventModelId, String, u32),
    RenamePlacement(EventModelId, PlacementId, String),
    MoveInterfacePlacement(EventModelId, PlacementId, u32, Option<AudienceId>),
    MoveTimelinePlacement(EventModelId, PlacementId, u32),
    MoveEventPlacement(EventModelId, PlacementId, u32, Option<StreamId>),
    RemovePlacement(EventModelId, PlacementId),

    // Clipboard
    DuplicateInterfacePlacement(EventModelId, PlacementId, u32, Option<AudienceId>),
    DuplicateTimelinePlacement(EventModelId, PlacementId, u32),
    DuplicateEventPlacement(EventModelId, PlacementId, u32, Option<StreamId>),

    // Component Details
    RenameComponent(EventModelId, ComponentId, String),
    SetComponentDescription(EventModelId, ComponentId, String),
    AddToComponentDescription(EventModelId, ComponentId, u32, String),
    DeleteFromComponentDescription(EventModelId, ComponentId, u32),
    SetComponentSchema(EventModelId, ComponentId, String),
    AddToComponentSchema(EventModelId, ComponentId, u32, String),
    DeleteFromComponentSchema(EventModelId, ComponentId, u32),
    ConfigureInterface(EventModelId, InterfaceConfig),

    // Flows
    ConnectFlow(EventModelId, PlacementId, Anchor, PlacementId, Anchor),
    DisconnectFlow(EventModelId, FlowId),
}
