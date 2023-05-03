use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema(pub String);

impl Default for Schema {
    fn default() -> Self {
        Schema(Default::default())
    }
}

// ***** Applying to Entities *****

pub trait HasSchema {
    fn schema(&self) -> &Schema;
}

pub trait HasModifiableSchema: HasSchema {
    fn schema_mut(&mut self) -> &mut Schema;

    fn set_schema(&mut self, schema: Schema) {
        let mut_schema = self.schema_mut();
        *mut_schema = schema
    }

    fn add_to_schema(&mut self, index: usize, addition: &str) {
        let Schema(s) = self.schema_mut();
        s.insert_str(index, addition);
    }

    fn delete_from_schema(&mut self, index: usize, count: usize) {
        let Schema(s) = self.schema_mut();
        for i in 0..count {
            s.remove(index + i);
        }
    }
}
