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
    AddToDescription(EventModelId, usize, String),
    DeleteFromDescription(EventModelId, usize, usize),
    SetSchema(EventModelId, String),
    AddToSchema(EventModelId, usize, String),
    DeleteFromSchema(EventModelId, usize),
    Delete(EventModelId),

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

    // Clipboard
    DuplicateInterfacePlacement(EventModelId, PlacementId, usize, Option<AudienceId>),
    DuplicateTimelinePlacement(EventModelId, PlacementId, usize),
    DuplicateEventPlacement(EventModelId, PlacementId, usize, Option<StreamId>),

    // Component Details
    RenameComponent(EventModelId, ComponentId, String),
    SetComponentDescription(EventModelId, ComponentId, String),
    AddToComponentDescription(EventModelId, ComponentId, usize, String),
    DeleteFromComponentDescription(EventModelId, ComponentId, usize),
    SetComponentSchema(EventModelId, ComponentId, String),
    AddToComponentSchema(EventModelId, ComponentId, usize, String),
    DeleteFromComponentSchema(EventModelId, ComponentId, usize),
    ConfigureInterface(EventModelId, InterfaceConfig),

    // Flows
    ConnectFlow(EventModelId, PlacementId, Anchor, PlacementId, Anchor),
    DisconnectFlow(EventModelId, FlowId),
}
