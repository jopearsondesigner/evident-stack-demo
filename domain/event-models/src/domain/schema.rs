use std::collections::HashMap;

#[derive(Debug)]
pub enum Schema {
    CUE(String),
    CDDL(String),
    Malli(String)
}

#[derive(Debug)]
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
