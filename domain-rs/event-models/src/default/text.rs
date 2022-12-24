use crate::types::text::{TextSuccession, Text};

impl Text for String {}

impl TextSuccession for String {
    fn with_insertion(&self, index: u32, addition: &str) -> Self {
        let mut new = self.clone();
        new.insert_str(index as usize, addition);
        new
    }

    fn with_removal(&self, index: u32, count: u32) -> String {
        let mut new = self.clone();
        for _ in 0..count {
            new.remove(index as usize);
        }
        new
    }
}