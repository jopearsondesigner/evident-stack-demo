pub mod sync;

use async_trait::async_trait;
use rand::random;
use std::collections::{BTreeMap, HashMap};
use uuid::Uuid;

// TODO: Snapshot event prunes any component definitions not present in placements
//  In a api context, we can't know when deleting a placement whether it's
//  the last placement for a given component definition in order to delete that
//  definition.  So we should leave the definitions in place until a snapshot,
//  then prune. Subsequent additions of placements against that definition
//  should then fail

pub type Node = u32;
pub type Counter = u32;

pub fn random_node() -> Node {
    random::<u32>()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct OpId(pub Counter, pub Node);

const MIN_ID: OpId = OpId(0, 0);
const MAX_ID: OpId = OpId(Counter::MAX, Node::MAX);

#[derive(Debug, Clone)]
pub struct Clock(HashMap<Node, Counter>);

#[derive(Debug, Clone)]
pub struct Patch<Op>(HashMap<OpId, Op>);

impl<Op: Clone> Patch<Op> {
    pub fn insert(&mut self, id: OpId, op: Op) {
        self.0.insert(id, op);
    }
}

impl<Op> Default for Patch<Op> {
    fn default() -> Self {
        Patch(HashMap::default())
    }
}

#[derive(Debug, Hash, Clone)]
pub struct OpSet<Op> {
    id: Uuid,
    ops: BTreeMap<OpId, Op>,
}

impl<Op> Default for OpSet<Op> {
    fn default() -> Self {
        Self {
            id: Default::default(),
            ops: Default::default(),
        }
    }
}

impl<Op: Clone> OpSet<Op> {
    pub fn new(id: Uuid) -> Self {
        OpSet {
            id,
            ops: Default::default(),
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn ops(&self) -> &BTreeMap<OpId, Op> {
        &self.ops
    }

    pub fn apply_patch(&mut self, patch: Patch<Op>) {
        let ops = &mut self.ops;
        for (id, op) in patch.0 {
            ops.insert(id, op);
        }
    }

    pub fn max_counter(&self) -> Counter {
        self.clock().0.into_values().max().unwrap_or(0)
    }

    pub fn next_id(&self, node: &Node) -> OpId {
        OpId(self.max_counter() + 1, node.to_owned())
    }

    pub fn clock(&self) -> Clock {
        let mut clock = Clock(HashMap::new());
        for id in self.ops().keys() {
            clock.0.insert(id.1, id.0);
        }
        clock
    }

    pub fn patch_from_clock(&self, clock: Clock) -> Patch<Op> {
        let mut patch = Patch(HashMap::new());
        for (id, op) in self.ops() {
            match clock.0.get(&id.1) {
                None => {
                    patch.0.insert(id.to_owned(), op.to_owned());
                }
                Some(counter) => {
                    if &id.0 > counter {
                        patch.0.insert(id.to_owned(), op.to_owned());
                    }
                }
            };
        }
        patch
    }

    pub fn node_patch_from_counter(&self, node: Node, counter: Counter) -> Patch<Op> {
        let mut patch = Patch(HashMap::new());
        for (id, op) in self.ops() {
            if id.1 == node && id.0 > counter {
                patch.0.insert(id.to_owned(), op.to_owned());
            }
        }
        patch
    }
}

// Bound Op as serde
#[async_trait]
pub trait OpSetReadableStorage<Op> {
    type Err;

    async fn load_clock(&self, opset_id: Uuid) -> Result<Clock, Self::Err>;

    async fn load_all_ops(&self, opset_id: Uuid) -> Result<Vec<Op>, Self::Err>;
    async fn load_ops_starting_at(
        &self,
        opset_id: Uuid,
        start_id: OpId,
    ) -> Result<Vec<Op>, Self::Err>;
}

// Bound Op as serde
#[async_trait]
pub trait OpSetWriteableStorage<Op>: OpSetReadableStorage<Op> {
    async fn commit_patch(&self, patch: &Patch<Op>) -> Result<(), Self::Err>;
}

pub trait Interpreter<Op: Clone> {
    type Interpretation;

    fn evolve(state: Self::Interpretation, id: &OpId, op: &Op) -> Self::Interpretation;

    fn interpret(initial: Self::Interpretation, opset: &OpSet<Op>) -> Self::Interpretation {
        opset.ops().iter().fold(initial, Self::evolve_from_op_pair)
    }

    fn evolve_from_op_pair(
        state: Self::Interpretation,
        (id, op): (&OpId, &Op),
    ) -> Self::Interpretation {
        Self::evolve(state, id, op)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
