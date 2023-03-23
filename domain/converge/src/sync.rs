use epoch::decider::{Decider, Event, Evolver};
use uuid::Uuid;

use crate::{OpSet, Patch};

pub enum SyncCommand<Op> {
    PushPatch(Uuid, Patch<Op>),
    SnapshotOpSet(Uuid),
    DeleteOpSet(Uuid),
}

pub enum SyncEvent<Op> {
    OpSetCreated(Uuid, Uuid),
    PatchReceived(Uuid, Uuid, Patch<Op>),
    OpSetSnapshot(Uuid, Uuid),
    OpSetDeleted(Uuid, Uuid),
}

pub enum SyncError {
    InvalidPatch, // TODO: Validation Errors
    IllegalState, // TODO: Error Message
}

pub enum OpSetState<Op> {
    BeforeCreation,
    Active(OpSet<Op>),
    Deleted,
}

impl<Op: Send + Sync> Decider for OpSetState<Op> {
    type Cmd = SyncCommand<Op>;

    type Err = SyncError;

    fn decide(state: &Self::State, cmd: &Self::Cmd) -> Result<Vec<Self::Evt>, Self::Err> {
        match cmd {
            SyncCommand::PushPatch(id, patch) => todo!(),
            SyncCommand::SnapshotOpSet(id) => todo!(),
            SyncCommand::DeleteOpSet(id) => todo!(),
        }
    }
}

impl<Op> Evolver for OpSetState<Op> {
    type State = OpSetState<Op>;
    type Evt = SyncEvent<Op>;

    fn evolve(state: Self::State, event: &Self::Evt) -> Self::State {
        match event {
            SyncEvent::OpSetCreated(_, id) => match state {
                OpSetState::BeforeCreation => OpSetState::Active(OpSet::default()),
                _ => state,
            },
            SyncEvent::PatchReceived(_, id, patch) => match state {
                OpSetState::Active(_) => todo!(),
                _ => state,
            },
            SyncEvent::OpSetSnapshot(_, id) => match state {
                OpSetState::Active(_) => todo!(),
                _ => state,
            },
            SyncEvent::OpSetDeleted(_, id) => match state {
                OpSetState::Active(_) => todo!(),
                _ => state,
            },
        }
    }

    fn init() -> Self::State {
        OpSetState::BeforeCreation
    }
}

impl<Op> Event for SyncEvent<Op> {
    type EntityId = Uuid;

    fn event_type(&self) -> String {
        match self {
            SyncEvent::OpSetCreated(_, _) => "OpSetCreated".to_string(),
            SyncEvent::PatchReceived(_, _, _) => "PatchReceived".to_string(),
            SyncEvent::OpSetSnapshot(_, _) => "OpSetSnapshot".to_string(),
            SyncEvent::OpSetDeleted(_, _) => "OpSetDeleted".to_string(),
        }
    }

    fn get_id(&self) -> Self::EntityId {
        match self {
            SyncEvent::OpSetCreated(eid, _) => eid.to_owned(),
            SyncEvent::PatchReceived(eid, _, _) => eid.to_owned(),
            SyncEvent::OpSetSnapshot(eid, _) => eid.to_owned(),
            SyncEvent::OpSetDeleted(eid, _) => eid.to_owned(),
        }
    }
}
