use uuid::Uuid;

pub(crate) type SchemaId = Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct CUESchema {
    id: SchemaId,
    definition: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CDDLSchema {
    id: SchemaId,
    definition: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MalliSchema {
    id: SchemaId,
    name: String,
    definition: String,
    description: Option<String>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Schema {
    CUE(CUESchema),
    CDDL(CDDLSchema),
    Malli(MalliSchema),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SchemaRole {
    CommandSchema,
    ResultSchema,

    EventBodySchema,

    QuerySchema,
    ReadModelSchema,

    ErrorSchema,
}
