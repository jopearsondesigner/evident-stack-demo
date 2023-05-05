use std::collections::HashMap;

use automerge::AutoCommit;
use base64::Engine;
use epoch::decider::{DeciderWithContext, Event, Evolver};

use crate::{Base64String, DocumentId, EventId, UserId};

#[derive(Debug, Clone)]
pub struct ClientSyncState {
    pub document_id: DocumentId,
    pub user_id: UserId,
    pub remote_doc: AutoCommit,
    pub patches_to_store: Vec<Vec<u8>>,
    pub local_doc: AutoCommit,
    pub patches_to_send: HashMap<usize, Base64String>,
    pub last_sent_patch: usize,
}

#[derive(Debug)]
pub enum CollaborationClientError {
    IllegalState,
    PatchParseError,
    RemoteStorageError,
    NotAuthorized,
}

#[derive(Debug)]
pub enum CollaborationClientCommand {
    Sync {
        document_id: DocumentId,
        user_id: UserId,
    },
}

#[derive(Debug)]
pub enum CollaborationClientEvent {
    Synced {
        event_id: EventId,
        document_id: DocumentId,
        user_id: UserId,
    },
    LocalPatchAvailable {
        event_id: EventId,
        document_id: DocumentId,
        user_id: UserId,
        patch: Base64String,
        patch_index: usize,
    },
    LocalPatchesSent {
        event_id: EventId,
        document_id: DocumentId,
        user_id: UserId,
        composite_patch: Base64String,
        patch_indexes: Vec<usize>,
    },
    RemotePatchReceived {
        event_id: EventId,
        document_id: DocumentId,
        sender_id: UserId,
        patch: Base64String,
    },
}

impl Event for CollaborationClientEvent {
    type EntityId = EventId;

    fn event_type(&self) -> String {
        match self {
            CollaborationClientEvent::Synced { .. } => "Synced".to_string(),
            CollaborationClientEvent::LocalPatchAvailable { .. } => {
                "LocalPatchAvailable".to_string()
            }
            CollaborationClientEvent::LocalPatchesSent { .. } => "LocalPatchSent".to_string(),
            CollaborationClientEvent::RemotePatchReceived { .. } => {
                "RemotePatchReceived".to_string()
            }
        }
    }

    fn get_id(&self) -> Self::EntityId {
        match self {
            CollaborationClientEvent::Synced { event_id, .. } => *event_id,
            CollaborationClientEvent::LocalPatchAvailable { event_id, .. } => *event_id,
            CollaborationClientEvent::LocalPatchesSent { event_id, .. } => *event_id,
            CollaborationClientEvent::RemotePatchReceived { event_id, .. } => *event_id,
        }
    }
}

#[derive(Debug)]
pub struct CollaborationDecider;

impl DeciderWithContext for CollaborationDecider {
    type Ctx = ();

    type Cmd = CollaborationClientCommand;

    type Err = CollaborationClientError;

    fn decide(
        _ctx: &Self::Ctx,
        state: &Self::State,
        cmd: &Self::Cmd,
    ) -> Result<Vec<Self::Evt>, Self::Err> {
        match cmd {
            CollaborationClientCommand::Sync {
                document_id,
                user_id,
            } => todo!(),
        }
    }
}

impl Evolver for CollaborationDecider {
    type State = ClientSyncState;

    type Evt = CollaborationClientEvent;

    fn evolve(mut state: Self::State, event: &Self::Evt) -> Self::State {
        match event {
            CollaborationClientEvent::Synced {
                event_id,
                document_id,
                user_id,
            } => todo!(),
            CollaborationClientEvent::LocalPatchAvailable {
                event_id,
                document_id,
                user_id,
                patch,
                patch_index,
            } => {
                state.patches_to_send.insert(*patch_index, patch.to_owned());
                state
            }
            CollaborationClientEvent::LocalPatchesSent {
                event_id,
                document_id,
                user_id,
                composite_patch,
                patch_indexes,
            } => todo!(),
            CollaborationClientEvent::RemotePatchReceived {
                event_id,
                document_id,
                sender_id,
                patch,
            } => todo!(),
        }
    }
}
