extern crate event_models;

pub mod grid;
mod indexed_db;
pub mod strategies;

use std::str::FromStr;

pub use crate::grid::EventModelGrid;
use crate::indexed_db::{IndexedDbError, IndexedDbStateRepository};
pub use crate::indexed_db::{Model, Patch};
use crate::strategies::{ReifyDecideSave, ReifyDecideSaveError, StateRepository};
use automerge::ActorId;
use autosurgeon::{hydrate, reconcile, Doc, HydrateError, ReadDoc, ReconcileError};
use event_models::api::commands::EventModelCommand;
use event_models::json::{JsonExport, JsonV1_0_0Transfer};
use event_models::{implementation::automerge::AutomergeEventModel, EventModelId, EventModelState};
use event_models::{Anchor, ColumnShift, ComponentId, EventModel, EventModelError, Named};
use js_sys::Uint8Array;
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

pub trait Reconcilable
where
    Self: Sized,
{
    fn reconcile(&self, doc: &mut impl Doc) -> Result<(), ReconcileError>;
    fn hydrate(doc: &impl ReadDoc) -> Result<Self, HydrateError>;
}

pub trait HasKey {
    fn get_key(&self) -> Option<Uuid>;
}

impl<E: EventModel> HasKey for EventModelState<E> {
    fn get_key(&self) -> Option<Uuid> {
        match self {
            EventModelState::BeforeCreation => None,
            EventModelState::EventModel(model) => Some(model.id()),
            EventModelState::Deleted(id) => Some(*id),
        }
    }
}

impl Reconcilable for EventModelState<AutomergeEventModel> {
    fn reconcile(&self, doc: &mut impl Doc) -> Result<(), ReconcileError> {
        if let EventModelState::EventModel(m) = self {
            reconcile(doc, m)
        } else {
            Ok(())
        }
    }

    fn hydrate(doc: &impl ReadDoc) -> Result<Self, HydrateError> {
        let model = hydrate(doc)?;
        Ok(EventModelState::EventModel(model))
    }
}

pub enum Lane {
    Audience,
    Stream,
}

impl TryFrom<&str> for Lane {
    type Error = JsValue;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "audience" => Ok(Self::Audience),
            "stream" => Ok(Self::Stream),
            &_ => Err(JsValue::from(format!(
                "Value {:?} is not a lane type",
                value
            ))),
        }
    }
}

pub enum ComponentType {
    Interface,
    Command,
    Event,
    ReadModel,
}

impl TryFrom<&str> for ComponentType {
    type Error = JsValue;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "interface" => Ok(Self::Interface),
            "command" => Ok(Self::Command),
            "event" => Ok(Self::Event),
            "read_model" => Ok(Self::ReadModel),
            &_ => Err(JsValue::from(format!(
                "Value {:?} is not a lane type",
                value
            ))),
        }
    }
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
    pub async fn name(&mut self) -> Option<String> {
        if let Ok(EventModelState::EventModel(model)) = self.repository.reify().await {
            Some(model.name().into())
        } else {
            None
        }
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        maybe_id_str: Option<String>,
        user: String,
    ) -> Result<EventModelStateManager, JsValue> {
        let actor = get_actor();
        if let Some(id_str) = maybe_id_str {
            let event_model_id: EventModelId =
                Uuid::from_str(&id_str).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
            Ok(EventModelStateManager {
                repository: IndexedDbStateRepository::new(Some(event_model_id), user, actor)
                    .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?,
            })
        } else {
            Ok(EventModelStateManager {
                repository: IndexedDbStateRepository::new(None, user, actor)
                    .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?,
            })
        }
    }

    pub fn refresh(&mut self, bin: Uint8Array) -> Result<(), JsValue> {
        if let Some(_model) = self.repository.key {
            // Save any local unsaved changes
            let local_changes = self.repository.save_incremental();
            // Load from caller arg
            self.repository
                .load_incremental(bin.to_vec())
                .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
            // Reset save_incremental state to prevent large, redundant patches after refresh
            self.repository.save_incremental();
            // Load local unsaved changes to they appear in next incremental save
            self.repository
                .load_incremental(local_changes)
                .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
            Ok(())
        } else {
            Err("Can't load data into a state manager with no model key!".into())
        }
    }

    pub async fn export(&mut self) -> Result<String, JsValue> {
        let illegal_state = JsValue::from_str("Can't export model");
        match self.repository.reify().await {
            Ok(m) => match m {
                EventModelState::BeforeCreation => Err(illegal_state),
                EventModelState::Deleted(_) => Err(illegal_state),
                EventModelState::EventModel(model) => {
                    let transfer: JsonV1_0_0Transfer = model.into();
                    transfer
                        .export()
                        .map_err(|err| JsValue::from_str(&format!("RepositoryError: {:?}", err)))
                }
            },
            Err(e) => Err(JsValue::from_str(&format!("RepositoryError: {:?}", e))),
        }
    }

    pub async fn grid(&mut self) -> Result<EventModelGrid, JsValue> {
        self.repository
            .reify()
            .await
            .map(|ref state| state.into())
            .map_err(|err| JsValue::from_str(&format!("RepositoryError: {:?}", err)))
    }

    pub async fn create(&mut self, name: String) -> Result<EventModelGrid, JsValue> {
        self.dispatch(EventModelCommand::Create(name)).await
    }

    pub async fn rename(
        &mut self,
        name: String,
        model_id_str: String,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        self.dispatch(EventModelCommand::Rename(model_id, name))
            .await
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

    pub async fn insert_columns(
        &mut self,
        model_id_str: String,
        index: usize,
        direction: String,
        count: usize,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        let column_shift =
            ColumnShift::try_from((direction.as_str(), index, count)).map_err(|_| {
                JsValue::from(format!(
                    "{:?} is an invalid column shift direction",
                    direction
                ))
            })?;

        self.dispatch(EventModelCommand::ShiftPlacements(model_id, column_shift))
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

    pub async fn add_lane(
        &mut self,
        model_id_str: String,
        kind: String,
        index: usize,
        name: String,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        let lane_type = Lane::try_from(kind.as_str())?;

        match lane_type {
            Lane::Audience => {
                let cmd = EventModelCommand::AddAudience(model_id, index, name);

                console::log_2(
                    &"Add Lane Command".into(),
                    &format!("{:?}", cmd).as_str().into(),
                );

                let res = self.dispatch(cmd).await;

                if let Ok(inner) = &res {
                    console::log_2(&"Add Lane Command Res".into(), &inner.audiences());
                } else {
                    console::log_1(&"failed".into())
                }

                res
            }
            Lane::Stream => {
                self.dispatch(EventModelCommand::AddStream(model_id, index, name))
                    .await
            }
        }
    }

    pub async fn edit_description(
        &mut self,
        model_id_str: String,
        index: usize,
        deletion_count: usize,
        addition: String,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        self.dispatch(EventModelCommand::EditDescription(
            model_id,
            index,
            deletion_count,
            addition,
        ))
        .await
    }

    pub async fn edit_component_description(
        &mut self,
        model_id_str: String,
        component_type_str: String,
        component_id_str: String,
        index: usize,
        deletion_count: usize,
        addition: String,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        let component_type = ComponentType::try_from(component_type_str.as_str())?;
        let component_id_uuid = parse_uuid(component_id_str)?;
        let component_id = match component_type {
            ComponentType::Interface => ComponentId::Interface(component_id_uuid),
            ComponentType::Command => ComponentId::Command(component_id_uuid),
            ComponentType::Event => ComponentId::Event(component_id_uuid),
            ComponentType::ReadModel => ComponentId::ReadModel(component_id_uuid),
        };
        self.dispatch(EventModelCommand::EditComponentDescription(
            model_id,
            component_id,
            index,
            deletion_count,
            addition,
        ))
        .await
    }

    pub async fn connect_flow(
        &mut self,
        model_id_str: String,
        source_placement_id_str: String,
        source_anchor_str: Option<String>,
        target_placement_id_str: String,
        target_anchor_str: Option<String>,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        let source_placement_id = parse_uuid(source_placement_id_str)?;
        let target_placement_id = parse_uuid(target_placement_id_str)?;
        let source_anchor: Anchor = source_anchor_str
            .try_into()
            .map_err(|e| JsValue::from(format!("Anchor from string error {:?}", e)))?;

        let target_anchor: Anchor = target_anchor_str
            .try_into()
            .map_err(|e| JsValue::from(format!("Anchor from string error {:?}", e)))?;

        self.dispatch(EventModelCommand::ConnectFlow(
            model_id,
            source_placement_id,
            source_anchor,
            target_placement_id,
            target_anchor,
        ))
        .await
    }

    pub async fn configure_interface(
        &mut self,
        model_id_str: String,
        interface_id_str: String,
        interface_type: String,
        interface_url: Option<String>,
    ) -> Result<EventModelGrid, JsValue> {
        let model_id = parse_uuid(model_id_str)?;
        let interface_id = parse_uuid(interface_id_str)?;
        self.dispatch(EventModelCommand::ConfigureInterface(
            model_id,
            event_models::ComponentId::Interface(interface_id),
            interface_type,
            interface_url,
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
