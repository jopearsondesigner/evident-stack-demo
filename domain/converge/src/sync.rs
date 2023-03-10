use epoch::decider::{Decider, Event, Evolver};
use uuid::Uuid;

use crate::{Clock, Node, OpSet, Patch};

pub struct PushPatch<Op> {
    opset_id: Uuid,
    node: Node,
    node_clock: Clock,
    patch: Patch<Op>,
}

pub enum SyncCommand<Op> {
    PushPatch(PushPatch<Op>),
    //    SnapshotOpSet(Uuid),
    DeleteOpSet(Uuid, Node),
}

pub struct OpSetCreated {
    event_id: Uuid,
    opset_id: Uuid,
    node: Node,
}

pub struct PatchReceived<Op> {
    event_id: Uuid,
    opset_id: Uuid,
    node: Node,
    patch: Patch<Op>,
}

pub struct OpSetDeleted {
    event_id: Uuid,
    opset_id: Uuid,
    node: Node,
}

pub enum SyncEvent<Op> {
    OpSetCreated(OpSetCreated),
    PatchReceived(PatchReceived<Op>),
    //    OpSetSnapshot(Uuid, Uuid, BTreeMap<OpId, Op>),
    OpSetDeleted(OpSetDeleted),
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

fn validate_patch<Err, Op>(
    patch: Patch<Op>,
    node: &u32,
    node_clock: &Clock,
    local_clock: &Clock,
) -> Result<Patch<Op>, Err> {
    todo!()
}

impl<Op: Send + Sync + Clone> Decider for OpSetState<Op> {
    type Cmd = SyncCommand<Op>;

    type Err = SyncError;

    fn decide(state: &Self::State, cmd: &Self::Cmd) -> Result<Vec<Self::Evt>, Self::Err> {
        match cmd {
            SyncCommand::PushPatch(PushPatch {
                opset_id,
                node,
                node_clock,
                patch,
            }) => match state {
                OpSetState::BeforeCreation => {
                    let valid_patch = validate_patch::<Self::Err, Op>(
                        patch.to_owned(),
                        node,
                        node_clock,
                        &Clock::default(),
                    )?;
                    Ok(vec![
                        SyncEvent::OpSetCreated(OpSetCreated {
                            event_id: Uuid::new_v4(),
                            opset_id: *opset_id,
                            node: *node,
                        }),
                        SyncEvent::PatchReceived(PatchReceived {
                            event_id: Uuid::new_v4(),
                            opset_id: *opset_id,
                            node: *node,
                            patch: valid_patch,
                        }),
                    ])
                }
                OpSetState::Active(opset) => {
                    let valid_patch = validate_patch::<Self::Err, Op>(
                        patch.to_owned(),
                        node,
                        node_clock,
                        &opset.clock(),
                    )?;
                    if &opset.id == opset_id {
                        Ok(vec![SyncEvent::PatchReceived(PatchReceived {
                            event_id: Uuid::new_v4(),
                            opset_id: *opset_id,
                            node: *node,
                            patch: valid_patch,
                        })])
                    } else {
                        Err(SyncError::InvalidPatch)
                    }
                }
                OpSetState::Deleted => Err(SyncError::IllegalState),
            },

            SyncCommand::DeleteOpSet(opset_id, node) => match state {
                OpSetState::Active(opset) => {
                    if &opset.id == opset_id {
                        Ok(vec![SyncEvent::OpSetDeleted(OpSetDeleted {
                            event_id: Uuid::new_v4(),
                            opset_id: *opset_id,
                            node: *node,
                        })])
                    } else {
                        Err(SyncError::InvalidPatch)
                    }
                }
                _ => Err(SyncError::IllegalState),
            },
        }
    }
}

impl<Op: Clone> Evolver for OpSetState<Op> {
    type State = OpSetState<Op>;
    type Evt = SyncEvent<Op>;

    fn evolve(state: Self::State, event: &Self::Evt) -> Self::State {
        match event {
            SyncEvent::OpSetCreated(OpSetCreated { opset_id, .. }) => match state {
                OpSetState::BeforeCreation => OpSetState::Active(OpSet::new(*opset_id)),
                _ => state,
            },
            SyncEvent::PatchReceived(PatchReceived { patch, .. }) => match state {
                OpSetState::Active(mut opset) => {
                    opset.apply_patch(patch.to_owned());
                    OpSetState::Active(opset)
                }
                _ => state,
            },
            SyncEvent::OpSetDeleted(_) => match state {
                OpSetState::Active(_) => OpSetState::Deleted,
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
            SyncEvent::OpSetCreated(_) => "OpSetCreated".to_string(),
            SyncEvent::PatchReceived(_) => "PatchReceived".to_string(),
            SyncEvent::OpSetDeleted(_) => "OpSetDeleted".to_string(),
        }
    }

    fn get_id(&self) -> Self::EntityId {
        match self {
            SyncEvent::OpSetCreated(OpSetCreated { event_id, .. }) => event_id.to_owned(),
            SyncEvent::PatchReceived(PatchReceived { event_id, .. }) => event_id.to_owned(),
            SyncEvent::OpSetDeleted(OpSetDeleted { event_id, .. }) => event_id.to_owned(),
        }
    }
}
