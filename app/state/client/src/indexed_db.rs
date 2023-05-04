use std::fmt::Debug;

use async_trait::async_trait;
use automerge::{ActorId, AutoCommit};
use event_models::{
    implementation::automerge::AutomergeEventModel, Described, EventModelState, Named,
};
use js_sys::Array;
use serde::Deserialize;
use state_shared::{automerge::Reconcilable, strategies::StateRepository, HasKey};
use uuid::Uuid;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::console;

#[wasm_bindgen(module = "$lib/indexed_db")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn patches(id: &str, user: &str, starting_at: Option<usize>) -> Result<JsValue, JsValue>;

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
    pub id: Option<usize>,
    pub user: String,
    pub model: String,
    pub data: Vec<u8>,
}

pub struct IndexedDbStateRepository {
    pub(crate) key: Option<Uuid>,
    pub(crate) user: String,
    pub(crate) automerge: AutoCommit,
    pub(crate) latest_patch: Option<usize>,
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
            latest_patch: None,
        })
    }

    pub fn load_incremental(&mut self, patch: Patch) -> Result<(), IndexedDbError> {
        let loaded_count = self
            .automerge
            .load_incremental(&patch.data)
            .map_err(|e| IndexedDbError::PatchLoad(format!("{:?}", e)))?;
        console::log_1(&format!("loaded ops into automerge {:?}", loaded_count).into());
        self.latest_patch = patch.id;
        Ok(())
    }

    pub fn save_incremental(&mut self) -> Vec<u8> {
        self.automerge.save_incremental()
    }

    pub fn state(&self) -> Result<EventModelState<AutomergeEventModel>, IndexedDbError> {
        EventModelState::<AutomergeEventModel>::hydrate(&self.automerge)
            .map_err(|e| IndexedDbError::Hydrate(format!("{:?}", e)))
    }
}

#[async_trait(?Send)]
impl StateRepository<EventModelState<AutomergeEventModel>, IndexedDbError>
    for IndexedDbStateRepository
{
    async fn reify(&mut self) -> Result<EventModelState<AutomergeEventModel>, IndexedDbError> {
        match &self.key {
            Some(key) => {
                let name: &str = &self.user;

                let patches = patches(&key.to_string(), &self.user, self.latest_patch)
                    .await
                    .map_err(|e| IndexedDbError::PatchLoad(format!("{:?}", e)))?;

                match js_sys::try_iter(&patches) {
                    Ok(Some(it)) => {
                        for patch in it {
                            let patch =
                                patch.map_err(|e| IndexedDbError::PatchLoad(format!("{:?}", e)))?;

                            let patch: Patch = serde_wasm_bindgen::from_value(patch)
                                .map_err(|e| IndexedDbError::PatchLoad(format!("{:?}", e)))?;

                            self.load_incremental(patch)?;
                        }
                        self.state()
                    }
                    Ok(None) => Ok(EventModelState::BeforeCreation),
                    Err(e) => Err(IndexedDbError::PatchLoad(format!("{:?}", e))),
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
                state
                    .reconcile(&mut self.automerge)
                    .map_err(|e| IndexedDbError::Reconcile(format!("{:?}", e)))?;
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
                            id: None,
                            model: id.to_string(),
                            user: self.user.to_owned(),
                            data: self.save_incremental(),
                        };
                        save(model, patch)
                            .await
                            .map_err(|e| IndexedDbError::PatchSave(format!("{:?}", e)))?;
                    }
                }

                Ok(state.to_owned())
            }
            None => Ok(EventModelState::BeforeCreation),
        }
    }
}
