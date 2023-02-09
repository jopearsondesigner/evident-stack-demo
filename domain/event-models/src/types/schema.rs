use serde_derive::{Deserialize, Serialize};

use super::Entity;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CUESchema(pub String);

// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub struct CDDLSchema(String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MalliSchema(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Schema {
    CUE(CUESchema),
    // CDDL(CDDLSchema),
    Malli(MalliSchema),
}

impl Default for Schema {
    fn default() -> Self {
        Schema::CUE(CUESchema(Default::default()))
    }
}

// ***** Schema Roles *****

pub type SubSchemaName = String;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandSchemaRole {
    CommandSchema,
    ResponseSchema,
    ErrorSchema,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventSchemaRole {
    EventSchema,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReadModelSchemaRole {
    QuerySchema,
    ReadModelSchema,
}

// ***** Applying to Entities *****

pub trait HasSchema: Entity {
    fn schema(&self) -> &Schema;
    fn schema_mut(&mut self) -> &mut Schema;

    fn set_schema(&mut self, schema: Schema);
    fn add_to_schema(&mut self, index: u32, addition: &str) {
        match self.schema_mut() {
            Schema::CUE(CUESchema(s)) => s.insert_str(index as usize, addition),
            Schema::Malli(MalliSchema(s)) => s.insert_str(index as usize, addition),
        };
    }

    fn delete_from_schema(&mut self, index: u32) {
        match self.schema_mut() {
            Schema::CUE(CUESchema(s)) => s.remove(index as usize),
            Schema::Malli(MalliSchema(s)) => s.remove(index as usize),
        };
    }
}
