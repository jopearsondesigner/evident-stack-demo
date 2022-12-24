pub trait Text {}

pub trait TextSuccession {
    fn with_insertion(&self, index: u32, addition: &str) -> Self;
    fn with_removal(&self, index: u32, count: u32) -> Self;
}

impl From<&dyn Text> for &str {
    fn from(_: &dyn Text) -> Self {
        todo!()
    }
}
