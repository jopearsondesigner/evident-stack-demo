use autosurgeon::{Doc, HydrateError, ReadDoc, ReconcileError};

pub trait Reconcilable
where
    Self: Sized,
{
    fn reconcile(&self, doc: &mut impl Doc) -> Result<(), ReconcileError>;
    fn hydrate(doc: &impl ReadDoc) -> Result<Self, HydrateError>;
}
