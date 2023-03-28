extern crate event_models;

mod repository;
mod utils;

use crate::repository::LocalStorageStateRepository;
use epoch::{repository::state::VersionedStateRepository, strategies::ReifyDecideSave};
pub use event_models::api::commands::EventModelCommand;
use event_models::{
    implementation::in_memory::{InMemoryCreationDetails, InMemoryEventModel},
    types::Entity,
    EventModelId, EventModelState,
};
use repository::HasKey;
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

#[wasm_bindgen]
impl EventModelStateManager {
    #[wasm_bindgen(constructor)]
    pub fn new(js_id: JsValue) -> Result<EventModelStateManager, JsValue> {
        let event_model_id: Option<EventModelId> = serde_wasm_bindgen::from_value(js_id)?;
        Ok(EventModelStateManager {
            repository: LocalStorageStateRepository::new(
                event_model_id.map(|x| x.to_string()),
                EventModelState::BeforeCreation(InMemoryCreationDetails),
            ),
        })
    }

    pub async fn state(&self) -> Result<JsValue, JsValue> {
        match self.repository.reify().await {
            Ok((data, _version)) => Ok(serde_wasm_bindgen::to_value(&data)?),
            Err(err) => Err(serde_wasm_bindgen::to_value(&err)?),
        }
    }

    pub async fn dispatch(&mut self, js_command: JsValue) -> Result<JsValue, JsValue> {
        let command: EventModelCommand = serde_wasm_bindgen::from_value(js_command)?;
        log(&format!("Dispatching {:?}...", command));
        let result =
            EventModelDecider::execute_reify_decide(&mut self.repository, &(), &command, None)
                .await;
        match result {
            Ok(state) => {
                log(&format!(
                    "...dispatched command {:?} and got next state {:?}",
                    command, state
                ));
                Ok(serde_wasm_bindgen::to_value(&state)?)
            }
            Err(err) => Err(JsValue::from(format!(
                "Error dispatching command {:?}: {:?}",
                command, err
            ))),
        }
    }
}
