extern crate event_models;

pub mod grid;
mod repository;
mod utils;

use std::str::FromStr;

use crate::repository::LocalStorageStateRepository;
use epoch::{repository::state::VersionedStateRepository, strategies::ReifyDecideSave};
use event_models::api::commands::EventModelCommand;
use event_models::{
    implementation::in_memory::{InMemoryCreationDetails, InMemoryEventModel},
    types::Entity,
    EventModelId, EventModelState,
};
pub use grid::EventModelGrid;
use js_sys::Uint8Array;
use repository::HasKey;
pub use utils::set_panic_hook;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

impl HasKey for EventModelState<InMemoryEventModel> {
    fn get_key(&self) -> Option<String> {
        match self {
            EventModelState::BeforeCreation(_) => None,
            EventModelState::EventModel(model) => Some(model.id().to_string()),
            EventModelState::Deleted(id) => Some(id.to_string()),
        }
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct EventModelStateManager {
    repository: LocalStorageStateRepository<EventModelState<InMemoryEventModel>>,
    // node: Node // TODO: convergent creation context details
}

pub struct EventModelDecider;

impl ReifyDecideSave for EventModelDecider {
    type Decide = EventModelState<InMemoryEventModel>;
}

fn parse_uuid(uuid_str: String) -> Result<Uuid, JsValue> {
    Uuid::from_str(&uuid_str)
        .map_err(|e| JsValue::from(format!("Error parsing Uuid from string: {:?}", e)))
}

#[wasm_bindgen]
impl EventModelStateManager {
    #[wasm_bindgen(constructor)]
    pub fn new(maybe_id_str: Option<String>) -> Result<EventModelStateManager, JsValue> {
        if let Some(id_str) = maybe_id_str {
            let event_model_id: EventModelId =
                Uuid::from_str(&id_str).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
            Ok(EventModelStateManager {
                repository: LocalStorageStateRepository::new(
                    Some(event_model_id.to_string()),
                    EventModelState::BeforeCreation(InMemoryCreationDetails),
                ),
            })
        } else {
            Ok(EventModelStateManager {
                repository: LocalStorageStateRepository::new(
                    None,
                    EventModelState::BeforeCreation(InMemoryCreationDetails),
                ),
            })
        }
    }

    pub async fn state(&self) -> Result<EventModelGrid, JsValue> {
        match self.repository.reify().await {
            Ok((state, _version)) => Ok(state.into()),
            Err(err) => Err(JsValue::from_str(&format!("{:?}", err))),
        }
    }

    pub async fn create(&mut self, name: String) -> Result<EventModelGrid, JsValue> {
        self.dispatch(EventModelCommand::Create(name)).await
    }

    pub async fn delete(&mut self, model_id_str: String) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        self.dispatch(EventModelCommand::Delete(model_id)).await
    }

    pub async fn import(
        &mut self,
        model_id_str: String,
        json: Uint8Array,
        offset: usize,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;

        self.dispatch(EventModelCommand::Import(model_id, offset, json.to_vec()))
            .await
    }

    async fn dispatch(&mut self, command: EventModelCommand) -> Result<EventModelGrid, JsValue> {
        let result =
            EventModelDecider::execute_reify_decide(&mut self.repository, &(), &command, None)
                .await;
        match result {
            Ok(state) => Ok(state.into()),
            Err(err) => Err(JsValue::from(format!(
                "Error dispatching command {:?}: {:?}",
                command, err
            ))),
        }
    }
}
