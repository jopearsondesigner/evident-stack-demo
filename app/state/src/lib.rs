extern crate event_models;

pub mod grid;
mod repository;

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
use js_sys::{Function, Uint8Array};
use repository::HasKey;
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

#[wasm_bindgen(js_name = setPanicHook)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
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
    store_setter: Option<js_sys::Function>,
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
    // TODO: we'll need to store a reference to the Svelte store's
    // setter here, for non-Command-driven state changes (e.g. background sync)
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
                store_setter: None,
            })
        } else {
            Ok(EventModelStateManager {
                repository: LocalStorageStateRepository::new(
                    None,
                    EventModelState::BeforeCreation(InMemoryCreationDetails),
                ),
                store_setter: None,
            })
        }
    }

    #[wasm_bindgen(setter)]
    pub fn set_store_setter(&mut self, setter: Option<Function>) {
        self.store_setter = setter
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

    pub async fn define_and_place_interface(
        &mut self,
        model_id_str: String,
        name: String,
        index: usize,
        maybe_audience_str: Option<String>,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        let audience = match maybe_audience_str {
            Some(audience_str) => Some(parse_uuid(audience_str)?),
            None => None,
        };
        self.dispatch(EventModelCommand::DefineAndPlaceInterface(
            model_id, name, index, audience,
        ))
        .await
    }

    pub async fn define_and_place_command(
        &mut self,
        model_id_str: String,
        name: String,
        index: usize,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        self.dispatch(EventModelCommand::DefineAndPlaceCommand(
            model_id, name, index,
        ))
        .await
    }

    pub async fn define_and_place_event(
        &mut self,
        model_id_str: String,
        name: String,
        index: usize,
        maybe_stream_str: Option<String>,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        let stream = match maybe_stream_str {
            Some(stream_str) => Some(parse_uuid(stream_str)?),
            None => None,
        };
        self.dispatch(EventModelCommand::DefineAndPlaceEvent(
            model_id, name, index, stream,
        ))
        .await
    }

    pub async fn define_and_place_read_model(
        &mut self,
        model_id_str: String,
        name: String,
        index: usize,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        self.dispatch(EventModelCommand::DefineAndPlaceReadModel(
            model_id, name, index,
        ))
        .await
    }

    pub async fn duplicate_interface_placement(
        &mut self,
        model_id_str: String,
        placement_id_str: String,
        index: usize,
        maybe_audience_str: Option<String>,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        let placement_id = parse_uuid(placement_id_str)?;
        let audience = match maybe_audience_str {
            Some(audience_str) => Some(parse_uuid(audience_str)?),
            None => None,
        };
        self.dispatch(EventModelCommand::DuplicateInterfacePlacement(
            model_id,
            placement_id,
            index,
            audience,
        ))
        .await
    }

    pub async fn duplicate_timeline_placement(
        &mut self,
        model_id_str: String,
        placement_id_str: String,
        index: usize,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        let placement_id = parse_uuid(placement_id_str)?;
        self.dispatch(EventModelCommand::DuplicateTimelinePlacement(
            model_id,
            placement_id,
            index,
        ))
        .await
    }

    pub async fn duplicate_event_placement(
        &mut self,
        model_id_str: String,
        placement_id_str: String,
        index: usize,
        maybe_stream_str: Option<String>,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        let placement_id = parse_uuid(placement_id_str)?;
        let stream = match maybe_stream_str {
            Some(stream_str) => Some(parse_uuid(stream_str)?),
            None => None,
        };
        self.dispatch(EventModelCommand::DuplicateEventPlacement(
            model_id,
            placement_id,
            index,
            stream,
        ))
        .await
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

    pub async fn move_interface_placement(
        &mut self,
        model_id_str: String,
        placement_id_str: String,
        index: usize,
        maybe_audience_str: Option<String>,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        let placement_id = parse_uuid(placement_id_str)?;
        let audience = match maybe_audience_str {
            Some(audience_str) => Some(parse_uuid(audience_str)?),
            None => None,
        };
        self.dispatch(EventModelCommand::MoveInterfacePlacement(
            model_id,
            placement_id,
            index,
            audience,
        ))
        .await
    }

    pub async fn move_timeline_placement(
        &mut self,
        model_id_str: String,
        placement_id_str: String,
        index: usize,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        let placement_id = parse_uuid(placement_id_str)?;
        self.dispatch(EventModelCommand::MoveTimelinePlacement(
            model_id,
            placement_id,
            index,
        ))
        .await
    }

    pub async fn move_event_placement(
        &mut self,
        model_id_str: String,
        placement_id_str: String,
        index: usize,
        maybe_stream_str: Option<String>,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        let placement_id = parse_uuid(placement_id_str)?;
        let stream = match maybe_stream_str {
            Some(stream_str) => Some(parse_uuid(stream_str)?),
            None => None,
        };
        self.dispatch(EventModelCommand::MoveEventPlacement(
            model_id,
            placement_id,
            index,
            stream,
        ))
        .await
    }

    pub async fn remove_placement(
        &mut self,
        model_id_str: String,
        placement_id_str: String,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        let placement_id = parse_uuid(placement_id_str)?;
        self.dispatch(EventModelCommand::RemovePlacement(model_id, placement_id))
            .await
    }

    pub async fn rename_placement(
        &mut self,
        model_id_str: String,
        placement_id_str: String,
        name: String,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        let placement_id = parse_uuid(placement_id_str)?;

        self.dispatch(EventModelCommand::RenamePlacement(
            model_id,
            placement_id,
            name,
        ))
        .await
    }

    async fn dispatch(&mut self, command: EventModelCommand) -> Result<EventModelGrid, JsValue> {
        let result =
            EventModelDecider::execute_reify_decide(&mut self.repository, &(), &command, None)
                .await;
        match result {
            Ok(state) => {
                if let Some(setter) = &self.store_setter {
                    let this = JsValue::null();
                    let grid: EventModelGrid = state.clone().into();
                    let _ = setter.call1(&this, &JsValue::from(grid));
                }
                Ok(state.into())
            }
            Err(err) => Err(JsValue::from(format!(
                "Error dispatching command {:?}: {:?}",
                command, err
            ))),
        }
    }
}
