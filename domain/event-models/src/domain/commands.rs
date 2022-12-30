use serde_derive::{Deserialize, Serialize};
use crate::types::audience::AudienceId;
use crate::types::ComponentId;
use crate::types::placement::PlacementId;
use crate::types::stream::StreamId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventModelCommand {
    // Event Model Details
    Create(String),
    Rename(String),
    AddToDescription(u32, String),
    DeleteFromDescription(u32),

    // Lanes
    AddAudience(u32, String),
    RenameAudience(AudienceId, String),
    ReorderAudience(AudienceId, u32),
    RemoveAudience(AudienceId),
    AddStream(u32, String),
    RenameStream(StreamId, String),
    ReorderStream(StreamId, u32),
    RemoveStream(StreamId),

    // Canvas
    DefineAndPlaceInterface(String, u32, Option<AudienceId>),
    DefineAndPlaceCommand(String, u32),
    DefineAndPlaceEvent(String, u32, Option<StreamId>),
    DefineAndPlaceReadModel(String, u32),
    RenamePlacement(PlacementId, String),
    MoveInterfacePlacement(PlacementId, u32, Option<AudienceId>),
    MoveTimelinePlacement(PlacementId, u32),
    MoveEventPlacement(PlacementId, u32, Option<StreamId>),
    RemovePlacement(PlacementId),

    // Clipboard
    DuplicateInterfacePlacement(PlacementId, u32, Option<AudienceId>),
    DuplicateTimelinePlacement(PlacementId, u32),
    DuplicateEventPlacement(PlacementId, u32, Option<StreamId>),

    // Component Details
    RenameComponent(ComponentId, String),
    AddToComponentDescription(),
    DeleteFromComponentDescription(),

}