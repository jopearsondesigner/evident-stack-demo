use async_trait::async_trait;
use std::collections::{BTreeMap, HashMap};

// TODO: Snapshot event prunes any component definitions not present in placements
//  In a api context, we can't know when deleting a placement whether it's
//  the last placement for a given component definition in order to delete that
//  definition.  So we should leave the definitions in place until a snapshot,
//  then prune. Subsequent additions of placements against that definition
//  should then fail

pub type Node = usize;
pub type Counter = usize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub Counter, pub Node);

const MIN_ID: Id = Id(0, 0);
const MAX_ID: Id = Id(Counter::MAX, Node::MAX);

pub type Clock = HashMap<Node, Counter>;
pub type Patch<Op> = HashMap<Id, Op>;

pub trait OpSet<Op> {
    fn ops(&self) -> &BTreeMap<Id, Op>;

    fn max_counter(&self) -> Option<Counter> {
        self.clock().into_values().max()
    }

    fn clock(&self) -> Clock {
        let mut clock: Clock = HashMap::new();
        for id in self.ops().keys() {
            clock.insert(id.1, id.0);
        }
        clock
    }

    fn patch_from_clock(&self, clock: Clock) -> Patch<Op> {
        let mut patch: Patch<Op> = HashMap::new();
        for (id, op) in self.ops() {
            match clock.get(&id.1) {
                None => {
                    patch.insert(*id.to_owned(), *op.to_owned());
                }
                Some(counter) => if &id.0 > counter {
                    patch.insert(*id.to_owned(), *op.to_owned());
                }
            };
        }
        patch
    }

    fn node_patch_from_counter(&self, node: Node, counter: Counter) -> Patch<Op> {
        let mut patch: Patch<Op> = HashMap::new();
        for (id, op) in self.ops() {
            if id.1 == node && id.0 > counter {
                patch.insert(*id.to_owned(), *op.to_owned());
            }
        }
        patch
    }
}

#[async_trait]
pub trait OpSetStorage<Op>: OpSet<Op> {
    type Err;

    // On success, result conveys the number of ops loaded
    async fn load_ops(&self) -> Result<usize, Self::Err>;
    async fn commit_patch(&self, patch: &Patch<Op>) -> Result<(), Self::Err>;
}

pub trait Interpreter<Op> {
    type Interpretation: Default;

    fn init() -> Self::Interpretation;
    fn evolve(state: &Self::Interpretation, op: &Op) -> Self::Interpretation;

    fn interpret(opset: &impl OpSet<Op>) -> Self::Interpretation {
        let state = Self::init();
        for (id, op) in opset.ops() {
            state = Self::evolve(&state, op);
        }
        state
    }
}
