extern crate event_models;

mod utils;

// use epoch::decider::{Decider, Evolver};
// use event_models::default::DefaultEventModel;
// use event_models::domain::commands::EventModelCommand;
// use event_models::domain::{EventModelDecider, EventModelState};
use event_models::EventModelId;
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
pub struct EventModelCreationContext;

impl Default for EventModelCreationContext {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl EventModelCreationContext {
    #[wasm_bindgen(constructor)]
    pub fn new() -> EventModelCreationContext {
        EventModelCreationContext
    }

    pub fn dispatch(&self, js_command: JsValue) -> Result<(), JsValue> {
        todo!()
        // let command: EventModelCommand = serde_wasm_bindgen::from_value(js_command)?;
        // log(&format!("Dispatching {:?}", command));
        // let current_state = EventModelState::BeforeCreation;
        // let events = EventModelDecider::decide(&current_state, &command)
        //     .map_err(|e| serde_wasm_bindgen::to_value(&e).unwrap())?;
        // let mut next_state: EventModelState<DefaultEventModel> = current_state;
        // for evt in events {
        //     next_state = EventModelDecider::evolve(next_state, &evt);
        // }
        // log(&format!("next_state: {:?}", next_state));
        // Ok(())
    }
}

#[wasm_bindgen]
pub struct EventModelStateManager {
    id: EventModelId,
    setter: Option<Function>,
}

#[wasm_bindgen]
impl EventModelStateManager {
    #[wasm_bindgen(constructor)]
    pub fn new(js_id: JsValue) -> Result<EventModelStateManager, JsValue> {
        let id: EventModelId = serde_wasm_bindgen::from_value(js_id)?;
        Ok(EventModelStateManager { id, setter: None })
    }

    pub fn initialize(&mut self, setter: Function) {
        self.setter = Some(setter);
        // TODO: load initial state to pass to setter
        let initial_state = &JsValue::from(1);
        self.set(initial_state);
    }

    pub fn dispatch(&self, js_command: JsValue) -> Result<(), JsValue> {
        // let command: EventModelCommand = serde_wasm_bindgen::from_value(js_command)?;
        // log(&format!("Dispatching {:?}...", command));
        // let current_state = EventModelState::BeforeCreation; // TODO: look up current state
        // log(&format!("...against current state {:?}", current_state));
        // let events = EventModelDecider::decide(&current_state, &command)
        //     .map_err(|e| serde_wasm_bindgen::to_value(&e).unwrap())?;
        // let mut next_state: EventModelState<DefaultEventModel> = current_state;
        // for evt in events {
        //     next_state = EventModelDecider::evolve(next_state, &evt);
        // }
        // log(&format!("next_state: {:?}", next_state));
        Ok(())
    }

    fn set(&self, next_state: &JsValue) {
        log(&format!(
            "setting: {:?} via setter {:?}",
            &next_state, &self.setter
        ));
        match &self.setter {
            None => (),
            Some(setter) => {
                setter.call1(&JsValue::null(), next_state).unwrap(); // TODO: panic on Err?
            }
        };
    }
}
