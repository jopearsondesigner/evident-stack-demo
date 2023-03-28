extern crate event_models;

mod utils;

use epoch::{
    repository::{
        in_memory::state::versioned::InMemoryStateRepository, state::VersionedStateRepository,
    },
    strategies::ReifyDecideSave,
};
pub use event_models::api::commands::EventModelCommand;
// use epoch::decider::{Decider, Evolver};
// use event_models::default::DefaultEventModel;
// use event_models::domain::commands::EventModelCommand;
// use event_models::domain::{EventModelDecider, EventModelState};
use event_models::{
    implementation::in_memory::{InMemoryCreationDetails, InMemoryEventModel},
    EventModelId, EventModelState,
};
use js_sys::Function;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct EventModelStateManager {
    event_model_id: EventModelId,
    repository: InMemoryStateRepository<EventModelState<InMemoryEventModel>>,
    // node: Node // TODO: convergent creation context details
    setter: Option<Function>,
}

pub struct EventModelDecider;

impl ReifyDecideSave for EventModelDecider {
    type Decide = EventModelState<InMemoryEventModel>;
}

#[wasm_bindgen]
impl EventModelStateManager {
    #[wasm_bindgen(constructor)]
    pub fn new(js_id: JsValue) -> Result<EventModelStateManager, JsValue> {
        let event_model_id: EventModelId = serde_wasm_bindgen::from_value(js_id)?;
        Ok(EventModelStateManager {
            event_model_id,
            repository: InMemoryStateRepository::new(EventModelState::BeforeCreation(
                InMemoryCreationDetails,
            )),
            setter: None,
        })
    }

    pub async fn initialize(&mut self, setter: Function) -> Result<(), JsValue> {
        self.setter = Some(setter);
        let initial_state = match self.repository.reify().await {
            Ok((state, _)) => state,
            Err(e) => {
                return Err(JsValue::from(format!(
                    "Error while loading initial state: {:?}",
                    e
                )))
            }
        };
        self.set(&initial_state);
        Ok(())
    }

    pub async fn dispatch(&mut self, js_command: JsValue) -> Result<(), JsValue> {
        let command: EventModelCommand = serde_wasm_bindgen::from_value(js_command)?;
        log(&format!("Dispatching {:?}...", command));
        let result =
            EventModelDecider::execute_reify_decide(&mut self.repository, &(), &command, None)
                .await;
        return match result {
            Ok(state) => {
                self.set(&state);
                log(&format!(
                    "...dispatched command {:?} and got next state {:?}",
                    command, state
                ));
                Ok(())
            }
            Err(err) => Err(JsValue::from(format!(
                "Error dispatching command {:?}: {:?}",
                command, err
            ))),
        };
    }

    fn set(&self, next_state: &EventModelState<InMemoryEventModel>) {
        log(&format!(
            "setting: {:?} via setter {:?}",
            &next_state, &self.setter
        ));
        match &self.setter {
            None => (),
            Some(setter) => {
                setter
                    .call1(
                        &JsValue::null(),
                        &serde_wasm_bindgen::to_value(next_state)
                            .expect("error serializing state when updating Svelte store"),
                    )
                    .expect("error while updating Svelte store");
            }
        };
    }
}
