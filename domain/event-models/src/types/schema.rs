use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema(String);

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

    fn set_schema(&mut self, schema: Schema);
    fn add_to_schema(&mut self, index: u32, addition: &str) {
        let Schema(s) = self.schema_mut();
        s.insert_str(index as usize, addition);
    }

    fn delete_from_schema(&mut self, index: u32) {
        let Schema(s) = self.schema_mut();
        s.remove(index as usize);
    }
}
