extern crate event_models;

mod utils;

use js_sys::Function;
use wasm_bindgen::prelude::*;
use event_models::domain::commands::EventModelCommand;
use event_models::{EventModel, EventModelId};
use event_models::default::DefaultEventModel;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct EventModelCreationContext;

#[wasm_bindgen]
impl EventModelCreationContext {
    #[wasm_bindgen(constructor)]
    pub fn new() -> EventModelCreationContext {
        EventModelCreationContext
    }

    pub fn dispatch(&self, js_command: JsValue) -> Result<(), JsValue> {
        let command: EventModelCommand = serde_wasm_bindgen::from_value(js_command)?;
        log(&format!("Dispatching {:?}", command));
        Ok(())
    }
}

#[wasm_bindgen]
pub struct EventModelStateManager {
    id: EventModelId,
    setter: Option<Function>
}

#[wasm_bindgen]
impl EventModelStateManager {
    #[wasm_bindgen(constructor)]
    pub fn new(js_id: JsValue) -> Result<EventModelStateManager, JsValue> {
        let id: EventModelId = serde_wasm_bindgen::from_value(js_id)?;
        Ok(EventModelStateManager {
            id,
            setter: None
        })
    }

    pub fn initialize(&mut self, setter: Function) {
        self.setter = Some(setter);
        match &self.setter {
            None => (),
            Some(setter) => {
                setter.call1(&JsValue::null(), &JsValue::from(1));
            }
        };
    }

    pub fn dispatch(&self, js_command: JsValue) -> Result<(), JsValue> {
        let command: EventModelCommand = serde_wasm_bindgen::from_value(js_command)?;
        log(&format!("Dispatching {:?}", command));
        log(&format!("Setter: {:?}", &self.setter));
        Ok(())
    }
}