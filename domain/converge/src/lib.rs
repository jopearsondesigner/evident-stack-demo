use std::collections::HashMap;

type Node = u64;
type Counter = u64;

pub trait OpSet {
    fn max_counter(&self) -> Counter;
    fn clock(&self) -> Clock {
        self.sorted_ops()
    }
    fn sorted_ops(&self) -> &BoxStream<(Id, Op)>;
    fn commit_patch(&self, patch: &HashMap<Id, Op>);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
