use std::fmt::Debug;

use crate::{strategies::StateRepository, HasKey, Reconcilable};
use async_trait::async_trait;
use automerge::{ActorId, AutoCommit};
use event_models::{
    implementation::automerge::AutomergeEventModel, Described, EventModelState, Named,
};
use serde::Deserialize;
use uuid::Uuid;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(module = "$lib/state/dexie")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn save(model: Model, patch: Patch) -> Result<(), JsValue>;
}

#[derive(Debug, Clone)]
pub enum IndexedDbError {
    PatchLoad(String),
    Hydrate(String),
    Reconcile(String),
    PatchSave(String),
}

#[derive(Clone, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct Model {
    pub id: String,
    pub user: String,
    pub name: String,
    pub description: String,
}

#[derive(Clone, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct Patch {
    pub model: String,
    pub data: Vec<u8>,
}

pub struct IndexedDbStateRepository {
    pub(crate) key: Option<Uuid>,
    pub(crate) user: String,
    pub(crate) automerge: AutoCommit,
}

impl IndexedDbStateRepository {
    pub async fn new(
        key: Option<Uuid>,
        user: String,
        actor: ActorId,
    ) -> Result<Self, IndexedDbError> {
        let automerge = AutoCommit::new().with_actor(actor);
        Ok(Self {
            key,
            user,
            automerge,
        })
    }

    pub fn load_incremental(&mut self, data: Vec<u8>) -> Result<(), IndexedDbError> {
        self.automerge
            .load_incremental(&data)
            .map_err(|e| IndexedDbError::PatchLoad(format!("{:?}", e)))?;
        Ok(())
    }

    pub fn save_incremental(&mut self) -> Vec<u8> {
        self.automerge.save_incremental()
    }

    pub fn state(&mut self) -> Result<EventModelState<AutomergeEventModel>, IndexedDbError> {
        if self.automerge.document().is_empty() {
            Ok(EventModelState::BeforeCreation)
        } else {
            EventModelState::<AutomergeEventModel>::hydrate(&self.automerge)
                .map_err(|e| IndexedDbError::Hydrate(format!("{:?}", e)))
        }
    }
}

#[async_trait(?Send)]
impl StateRepository<EventModelState<AutomergeEventModel>, IndexedDbError>
    for IndexedDbStateRepository
{
    async fn reify(&mut self) -> Result<EventModelState<AutomergeEventModel>, IndexedDbError> {
        // We rely on in-memory AutoCommit doc, updated out-of-band via load_incremental, rather than reifying from storage
        self.state()
    }

    async fn save(
        &mut self,
        state: &EventModelState<AutomergeEventModel>,
    ) -> Result<EventModelState<AutomergeEventModel>, IndexedDbError> {
        match state.get_key() {
            Some(id) => {
                self.key = Some(id);
                state
                    .reconcile(&mut self.automerge)
                    .map_err(|e| IndexedDbError::Reconcile(format!("{:?}", e)))?;
                match state {
                    EventModelState::BeforeCreation => (),
                    EventModelState::Deleted(_) => (),
                    EventModelState::EventModel(m) => {
                        let data = self.save_incremental();
                        if !data.is_empty() {
                            let model = Model {
                                id: id.to_string(),
                                user: self.user.to_owned(),
                                name: m.name().into(),
                                description: m.description().to_owned(),
                            };
                            let patch = Patch {
                                model: id.to_string(),
                                data,
                            };
                            save(model, patch)
                                .await
                                .map_err(|e| IndexedDbError::PatchSave(format!("{:?}", e)))?;
                        }
                    }
                }

                Ok(state.to_owned())
            }
            None => Ok(EventModelState::BeforeCreation),
        }
    }
}
