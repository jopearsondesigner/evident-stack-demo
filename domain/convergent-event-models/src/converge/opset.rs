use std::{collections::BTreeMap, fmt::Debug};

use super::{Id, OpSet};

#[derive(Debug)]
pub struct InMemoryOpSet<Op> {
    ops: BTreeMap<Id, Op>,
}

impl<Op> InMemoryOpSet<Op> {
    fn new(ops: BTreeMap<Id, Op>) -> Self {
        InMemoryOpSet { ops }
    }
}

impl<Op> Default for InMemoryOpSet<Op> {
    fn default() -> Self {
        Self {
            ops: Default::default(),
        }
    }
}

impl<Op: Clone + Debug> OpSet<Op> for InMemoryOpSet<Op> {
    fn ops(&self) -> &BTreeMap<Id, Op> {
        &self.ops
    }

    fn apply_patch(&mut self, patch: super::Patch<Op>) {
        let ops = &mut self.ops;
        for (id, op) in patch {
            ops.insert(id, op);
        }
    }
}
