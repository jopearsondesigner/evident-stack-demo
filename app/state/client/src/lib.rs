extern crate event_models;

pub mod grid;
mod indexed_db;
mod sync;

use std::str::FromStr;

pub use crate::grid::EventModelGrid;
use crate::grid::Lane;
use crate::indexed_db::{IndexedDbError, IndexedDbStateRepository};
pub use crate::indexed_db::{Model, Patch};
use automerge::ActorId;
use event_models::api::commands::EventModelCommand;
use event_models::EventModelError;
use event_models::{implementation::automerge::AutomergeEventModel, EventModelId, EventModelState};
use js_sys::Uint8Array;
use state_shared::strategies::{ReifyDecideSave, ReifyDecideSaveError, StateRepository};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use web_sys::{console, window};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

#[wasm_bindgen]
pub struct EventModelStateManager {
    repository: IndexedDbStateRepository,
}

struct EventModelDecider;

impl ReifyDecideSave for EventModelDecider {
    type Decide = EventModelState<AutomergeEventModel>;
}

fn parse_uuid(uuid_str: String) -> Result<Uuid, JsValue> {
    Uuid::from_str(&uuid_str)
        .map_err(|e| JsValue::from(format!("Error parsing Uuid from string: {:?}", e)))
}

const ACTOR_ID_STORAGE_KEY: &str = "";

fn get_actor() -> ActorId {
    match window() {
        Some(win) => match win.session_storage() {
            Ok(Some(storage)) => match storage.get_item(ACTOR_ID_STORAGE_KEY) {
                Ok(Some(actor_id_str)) => match ActorId::from_str(&actor_id_str) {
                    Ok(actor) => actor,
                    Err(_) => {
                        console::log_2(
                            &"Invalid Actor ID string found in session storage: ".into(),
                            &actor_id_str.into(),
                        );
                        let actor = ActorId::random();
                        storage
                            .set_item(ACTOR_ID_STORAGE_KEY, &actor.to_string())
                            .expect("Session storage error");
                        actor
                    }
                },
                Ok(None) => {
                    let actor = ActorId::random();
                    storage
                        .set_item(ACTOR_ID_STORAGE_KEY, &actor.to_string())
                        .expect("Session storage error");
                    actor
                }
                Err(_) => ActorId::random(),
            },
            _ => ActorId::random(),
        },
        None => ActorId::random(),
    }
}

#[wasm_bindgen]
impl EventModelStateManager {
    #[wasm_bindgen(constructor)]
    pub async fn new(
        maybe_id_str: Option<String>,
        user: String,
    ) -> Result<EventModelStateManager, JsValue> {
        let actor = get_actor();
        if let Some(id_str) = maybe_id_str {
            let event_model_id: EventModelId =
                Uuid::from_str(&id_str).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
            Ok(EventModelStateManager {
                repository: IndexedDbStateRepository::new(Some(event_model_id), user, actor)
                    .await
                    .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?,
            })
        } else {
            Ok(EventModelStateManager {
                repository: IndexedDbStateRepository::new(None, user, actor)
                    .await
                    .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?,
            })
        }
    }

    pub fn refresh(
        &mut self,
        data: Uint8Array,
        latest_patch_id: usize,
    ) -> Result<EventModelGrid, JsValue> {
        if let Some(model) = self.repository.key {
            self.repository
                .load_incremental(Patch {
                    id: Some(latest_patch_id),
                    user: self.repository.user.to_owned(),
                    model: model.to_string(),
                    data: data.to_vec(),
                })
                .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
            self.repository
                .state()
                .map(|ref state| state.into())
                .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
        } else {
            Err("Can't load data into a state manager with no model key!".into())
        }
    }

    pub async fn state(&mut self) -> Result<EventModelGrid, JsValue> {
        self.repository
            .reify()
            .await
            .map(|ref state| state.into())
            .map_err(|err| JsValue::from_str(&format!("RepositoryError: {:?}", err)))
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

    pub async fn rename_lane(
        &mut self,
        model_id_str: String,
        kind: String,
        lane_id_str: String,
        name: String,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        let lane_id = parse_uuid(lane_id_str)?;
        let lane_type = Lane::try_from(kind.as_str())?;

        match lane_type {
            Lane::Audience => {
                self.dispatch(EventModelCommand::RenameAudience(model_id, lane_id, name))
                    .await
            }
            Lane::Stream => {
                self.dispatch(EventModelCommand::RenameStream(model_id, lane_id, name))
                    .await
            }
        }
    }

    pub async fn reorder_lane(
        &mut self,
        model_id_str: String,
        kind: String,
        lane_id_str: String,
        index: usize,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        let lane_id = parse_uuid(lane_id_str)?;
        let lane_type = Lane::try_from(kind.as_str())?;

        match lane_type {
            Lane::Audience => {
                self.dispatch(EventModelCommand::ReorderAudience(model_id, lane_id, index))
                    .await
            }
            Lane::Stream => {
                self.dispatch(EventModelCommand::ReorderStream(model_id, lane_id, index))
                    .await
            }
        }
    }

    pub async fn remove_lane(
        &mut self,
        model_id_str: String,
        kind: String,
        lane_id_str: String,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        let lane_id = parse_uuid(lane_id_str)?;
        let lane_type = Lane::try_from(kind.as_str())?;

        match lane_type {
            Lane::Audience => {
                self.dispatch(EventModelCommand::RemoveAudience(model_id, lane_id))
                    .await
            }
            Lane::Stream => {
                self.dispatch(EventModelCommand::RemoveStream(model_id, lane_id))
                    .await
            }
        }
    }

    pub async fn add_to_description(
        &mut self,
        model_id_str: String,
        index: usize,
        addition: String,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        self.dispatch(EventModelCommand::AddToDescription(
            model_id, index, addition,
        ))
        .await
    }

    pub async fn delete_from_description(
        &mut self,
        model_id_str: String,
        index: usize,
        count: usize,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        self.dispatch(EventModelCommand::DeleteFromDescription(
            model_id, index, count,
        ))
        .await
    }

    async fn dispatch(&mut self, command: EventModelCommand) -> Result<EventModelGrid, JsValue> {
        let result: Result<
            EventModelState<AutomergeEventModel>,
            ReifyDecideSaveError<EventModelError, IndexedDbError>,
        > = EventModelDecider::execute_reify_decide(&mut self.repository, &(), &command).await;
        match &result {
            Ok(state) => Ok(state.into()),
            Err(err) => Err(JsValue::from(format!(
                "Error dispatching command {:?}: {:?}",
                command, err
            ))),
        }
    }
}
