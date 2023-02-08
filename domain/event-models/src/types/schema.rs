use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CUESchema(String);

// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub struct CDDLSchema(String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MalliSchema(String);

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandSchemaRole {
    CommandSchema,
    ResponseSchema,
    ErrorSchema,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventSchemaRole {
    EventSchema,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReadModelSchemaRole {
    QuerySchema,
    ReadModelSchema,
}
