use std::fmt::Debug;

use crate::{automerge::Reconcilable, strategies::StateRepository, HasKey};
use async_trait::async_trait;
use automerge::AutoCommit;
use event_models::{
    implementation::automerge::AutomergeEventModel, Described, EventModelState, Named,
};
use js_sys::{Array, Uint8Array};
use uuid::Uuid;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::console;

#[wasm_bindgen(module = "$lib/indexed_db")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn patches(id: &str, user: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn save(model: Model, patch: Patch) -> Result<(), JsValue>;
}

pub struct IndexedDbStateRepository {
    key: Option<Uuid>,
    user: String,
    automerge: AutoCommit,
}

#[derive(Debug, Clone)]
pub enum IndexedDbError {
    StorageError(JsValue),
    PatchLoadError(String),
    HydrateError(String),
    ReconcileError(String),
    PatchSaveError(String),
}

#[derive(Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct Patch {
    pub user: String,
    pub model: String,
    pub data: Vec<u8>,
}

#[derive(Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct Model {
    pub id: String,
    pub user: String,
    pub name: String,
    pub description: String,
}

impl IndexedDbStateRepository {
    pub async fn new(key: Option<Uuid>, user: String) -> Result<Self, IndexedDbError> {
        let automerge = AutoCommit::new();
        Ok(Self {
            key,
            user,
            automerge,
        })
    }

    pub fn load_incremental(&mut self, data: Vec<u8>) -> Result<(), IndexedDbError> {
        self.automerge
            .load_incremental(&data)
            .map(|_| ())
            .map_err(|e| IndexedDbError::PatchLoadError(format!("{:?}", e)))
    }

    pub fn save_incremental(&mut self) -> Vec<u8> {
        self.automerge.save_incremental()
    }
}

#[async_trait(?Send)]
impl StateRepository<EventModelState<AutomergeEventModel>, IndexedDbError>
    for IndexedDbStateRepository
{
    async fn reify(&mut self) -> Result<EventModelState<AutomergeEventModel>, IndexedDbError> {
        match &self.key {
            Some(key) => {
                let lookup_key = Array::new_with_length(2);
                let name: &str = &self.user;
                lookup_key.set(0, name.into());
                lookup_key.set(1, key.to_string().into());

                let patches = patches(&key.to_string(), &self.user)
                    .await
                    .map_err(|e| IndexedDbError::PatchLoadError(format!("{:?}", e)))?;

                match js_sys::try_iter(&patches) {
                    Ok(Some(it)) => {
                        for patch in it {
                            let patch = patch
                                .map_err(|e| IndexedDbError::PatchLoadError(format!("{:?}", e)))?;

                            let data: Vec<u8> = Uint8Array::new(&patch).to_vec();

                            self.load_incremental(data)?;
                        }
                        EventModelState::<AutomergeEventModel>::hydrate(&self.automerge)
                            .map_err(|e| IndexedDbError::HydrateError(format!("{:?}", e)))
                    }
                    Ok(None) => Ok(EventModelState::BeforeCreation),
                    Err(e) => Err(IndexedDbError::PatchLoadError(format!("{:?}", e))),
                }
            }
            None => Ok(EventModelState::BeforeCreation),
        }
    }

    async fn save(
        &mut self,
        state: &EventModelState<AutomergeEventModel>,
    ) -> Result<EventModelState<AutomergeEventModel>, IndexedDbError> {
        match state.get_key() {
            Some(id) => {
                self.key = Some(id);
                console::log_2(&"Saving model".into(), &id.to_string().into());
                state
                    .reconcile(&mut self.automerge)
                    .map_err(|e| IndexedDbError::ReconcileError(format!("{:?}", e)))?;
                console::log_1(&"  - [x] reconciled".into());
                match state {
                    EventModelState::BeforeCreation => (),
                    EventModelState::Deleted(_) => (),
                    EventModelState::EventModel(m) => {
                        let model = Model {
                            id: id.to_string(),
                            user: self.user.to_owned(),
                            name: m.name().into(),
                            description: m.description().to_owned(),
                        };
                        let patch = Patch {
                            model: id.to_string(),
                            user: self.user.to_owned(),
                            data: self.save_incremental(),
                        };
                        console::log_3(
                            &"  - [x] model and patch created".into(),
                            &model.clone().into(),
                            &patch.clone().into(),
                        );
                        save(model, patch)
                            .await
                            .map_err(|e| IndexedDbError::ReconcileError(format!("{:?}", e)))?;
                        console::log_1(&"  - [x] model and patch saved".into());
                    }
                }

                Ok(state.to_owned())
            }
            None => Ok(EventModelState::BeforeCreation),
        }
    }
}
