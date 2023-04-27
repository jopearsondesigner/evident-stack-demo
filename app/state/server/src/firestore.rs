use std::fmt::Debug;

use async_trait::async_trait;
use automerge::AutoCommit;
use collaboration::server::CollaborativeDocument;
use js_sys::Uint8Array;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

use state_shared::{automerge::Reconcilable, strategies::StateRepository, HasKey};

#[wasm_bindgen(module = "$lib/firebase/firestore")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn patches(id: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn appendPatch(id: &str, patch: Vec<u8>) -> Result<(), JsValue>;
}

#[derive(Debug, Clone)]
pub(crate) enum FirestoreError {
    PatchLoadError(String),
    HydrateError(String),
    ReconcileError(String),
    PatchSaveError(String),
}

#[derive(Debug, Clone)]
pub(crate) struct FirestoreAutomergeStateRepository {
    key: Option<Uuid>,
    automerge: AutoCommit,
}

impl FirestoreAutomergeStateRepository {
    // TODO: provide actorId from outside?
    pub fn new(key: Option<Uuid>) -> Self {
        let automerge = AutoCommit::new();
        Self { key, automerge }
    }

    pub fn load_incremental(&mut self, data: &[u8]) -> Result<(), FirestoreError> {
        self.automerge
            .load_incremental(data)
            .map(|_| ())
            .map_err(|e| FirestoreError::PatchLoadError(format!("{:?}", e)))
    }

    pub fn save(&mut self) -> Vec<u8> {
        self.automerge.save()
    }

    pub fn save_incremental(&mut self) -> Vec<u8> {
        self.automerge.save_incremental()
    }
}

#[async_trait(?Send)]
impl StateRepository<CollaborativeDocument, FirestoreError> for FirestoreAutomergeStateRepository {
    // TODO: distinguish BeforeCreation, EventModel(m), and Deleted(id), especially Deleted
    async fn reify(&mut self) -> Result<CollaborativeDocument, FirestoreError> {
        todo!()
    }

    async fn save(
        &mut self,
        state: &CollaborativeDocument,
    ) -> Result<CollaborativeDocument, FirestoreError> {
        todo!()
    }
}
