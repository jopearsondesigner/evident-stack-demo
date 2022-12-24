use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum Schema {
    CUE(String),
    CDDL(String),
    Malli(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SchemaRole {
    CommandSchema,
    ResultSchema,

    EventBodySchema,

    QuerySchema,
    ReadModelSchema,

    ErrorSchema,
}

pub trait HasSchemaByRole {
    fn schemas(&self) -> HashMap<&SchemaRole, &Schema>;
}
