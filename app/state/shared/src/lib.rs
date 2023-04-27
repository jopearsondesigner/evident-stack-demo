use crate::automerge::Reconcilable;
use autosurgeon::{hydrate, reconcile, Doc, HydrateError, ReadDoc, ReconcileError};
use event_models::{implementation::automerge::AutomergeEventModel, EventModel, EventModelState};
use uuid::Uuid;

pub mod automerge;
pub mod strategies;

pub trait HasKey {
    fn get_key(&self) -> Option<Uuid>;
}

impl<E: EventModel> HasKey for EventModelState<E> {
    fn get_key(&self) -> Option<Uuid> {
        match self {
            EventModelState::BeforeCreation => None,
            EventModelState::EventModel(model) => Some(model.id()),
            EventModelState::Deleted(id) => Some(*id),
        }
    }
}

impl Reconcilable for EventModelState<AutomergeEventModel> {
    fn reconcile(&self, doc: &mut impl Doc) -> Result<(), ReconcileError> {
        if let EventModelState::EventModel(m) = self {
            reconcile(doc, m)
        } else {
            Ok(())
        }
    }

    fn hydrate(doc: &impl ReadDoc) -> Result<Self, HydrateError> {
        let model = hydrate(doc)?;
        Ok(EventModelState::EventModel(model))
    }
}
