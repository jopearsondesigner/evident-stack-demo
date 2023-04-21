use std::fmt::Debug;

use async_trait::async_trait;
use automerge::AutoCommit;
use base64::prelude::*;
use event_models::{implementation::automerge::AutomergeEventModel, EventModelState};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use web_sys::{window, Storage};

use crate::{automerge::Reconcilable, strategies::StateRepository, HasKey};

#[derive(Debug, Clone)]
pub struct LocalStorageStateRepository {
    key: Option<Uuid>,
    automerge: AutoCommit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocalStorageError {
    StorageFailure,
    PatchLoadError(String),
    HydrateError(String),
    ReconcileError(String),
    PatchSaveError(String),
}

impl LocalStorageStateRepository {
    pub fn new(key: Option<Uuid>) -> Self {
        let automerge = AutoCommit::new();
        Self { key, automerge }
    }

    fn storage() -> Result<Storage, LocalStorageError> {
        match window() {
            Some(win) => match win.local_storage() {
                Ok(Some(storage)) => Ok(storage),
                _ => Err(LocalStorageError::StorageFailure),
            },
            None => Err(LocalStorageError::StorageFailure),
        }
    }

    pub fn load_incremental(&mut self, data: Vec<u8>) -> Result<(), LocalStorageError> {
        self.automerge
            .load_incremental(&data)
            .map(|_| ())
            .map_err(|e| LocalStorageError::PatchLoadError(format!("{:?}", e)))
    }

    pub fn save(&mut self) -> Vec<u8> {
        self.automerge.save()
    }

    pub fn save_incremental(&mut self) -> Vec<u8> {
        self.automerge.save_incremental()
    }
}

#[async_trait(?Send)]
impl StateRepository<EventModelState<AutomergeEventModel>, LocalStorageError>
    for LocalStorageStateRepository
{
    async fn reify(&mut self) -> Result<EventModelState<AutomergeEventModel>, LocalStorageError> {
        let storage = Self::storage()?;
        match &self.key {
            Some(key) => match storage.get_item(&key.to_string()) {
                Ok(Some(s)) => {
                    let data = BASE64_STANDARD
                        .decode(s)
                        .map_err(|e| LocalStorageError::PatchLoadError(format!("{:?}", e)))?;
                    self.load_incremental(data)?;
                    EventModelState::<AutomergeEventModel>::hydrate(&self.automerge)
                        .map_err(|e| LocalStorageError::HydrateError(format!("{:?}", e)))
                }
                Ok(None) => Ok(EventModelState::BeforeCreation),
                Err(e) => Err(LocalStorageError::PatchLoadError(format!("{:?}", e))),
            },
            None => Ok(EventModelState::BeforeCreation),
        }
    }

    async fn save(
        &mut self,
        state: &EventModelState<AutomergeEventModel>,
    ) -> Result<EventModelState<AutomergeEventModel>, LocalStorageError> {
        let storage = Self::storage()?;
        match state.get_key() {
            Some(id) => {
                self.key = Some(id);
                state
                    .reconcile(&mut self.automerge)
                    .map_err(|e| LocalStorageError::ReconcileError(format!("{:?}", e)))?;
                let serialized = BASE64_STANDARD.encode(self.save());
                storage
                    .set_item(&id.to_string(), &serialized)
                    .map_err(|e| LocalStorageError::PatchSaveError(format!("{:?}", e)))?;
                Ok(state.to_owned())
            }
            None => Ok(EventModelState::BeforeCreation),
        }
    }
}
