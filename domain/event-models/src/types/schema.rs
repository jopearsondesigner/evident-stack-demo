use std::collections::HashMap;
use crate::types::text::Text;

pub enum Schema {
    CUE(Box<dyn Text>),
    CDDL(Box<dyn Text>),
    Malli(Box<dyn Text>)
}

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