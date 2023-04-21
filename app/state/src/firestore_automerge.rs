use std::fmt::Debug;

use async_trait::async_trait;
use automerge::AutoCommit;
use event_models::{implementation::automerge::AutomergeEventModel, EventModelState};
use js_sys::Uint8Array;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

use crate::{automerge::Reconcilable, strategies::StateRepository, HasKey};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

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
impl StateRepository<EventModelState<AutomergeEventModel>, FirestoreError>
    for FirestoreAutomergeStateRepository
{
    // TODO: distinguish BeforeCreation, EventModel(m), and Deleted(id), especially Deleted
    async fn reify(&mut self) -> Result<EventModelState<AutomergeEventModel>, FirestoreError> {
        if let Some(ref k) = self.key {
            let patches = patches(&k.to_string())
                .await
                .map_err(|e| FirestoreError::PatchLoadError(format!("{:?}", e)))?;
            if let Some(iterator) = js_sys::try_iter(&patches)
                .map_err(|e| FirestoreError::PatchLoadError(format!("{:?}", e)))?
            {
                for patch in iterator {
                    let patch =
                        patch.map_err(|e| FirestoreError::PatchLoadError(format!("{:?}", e)))?;

                    let data: Vec<u8> = Uint8Array::new(&patch).to_vec();

                    self.load_incremental(&data)
                        .map_err(|e| FirestoreError::PatchLoadError(format!("{:?}", e)))?;
                }
            }

            EventModelState::<AutomergeEventModel>::hydrate(&self.automerge)
                .map_err(|e| FirestoreError::HydrateError(format!("{:?}", e)))
        } else {
            Ok(EventModelState::BeforeCreation)
        }
    }

    async fn save(
        &mut self,
        state: &EventModelState<AutomergeEventModel>,
    ) -> Result<EventModelState<AutomergeEventModel>, FirestoreError> {
        match state.get_key() {
            Some(id) => {
                self.key = Some(id);
                state
                    .reconcile(&mut self.automerge)
                    .map_err(|e| FirestoreError::ReconcileError(format!("{:?}", e)))?;
                appendPatch(&id.to_string(), self.save_incremental())
                    .await
                    .map_err(|e| FirestoreError::PatchSaveError(format!("{:?}", e)))?;
                Ok(state.to_owned())
            }
            None => Ok(EventModelState::BeforeCreation),
        }
    }
}
