mod firestore;

use std::str::FromStr;

use crate::firestore::{FirestoreAutomergeStateRepository, FirestoreError};
use collaboration::{
    server::{CollaborationCommand, CollaborationDecider, CollaborativeDocument},
    CollaborationError,
};
use js_sys::Uint8Array;
use state_shared::strategies::{ReifyDecideSave, ReifyDecideSaveError, StateRepository};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

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
pub struct AutomergeSyncServerStateManager {
    repository: FirestoreAutomergeStateRepository,
}

struct AutomergeSyncServerDecider;

impl ReifyDecideSave for AutomergeSyncServerDecider {
    type Decide = CollaborationDecider;
}

fn parse_uuid(uuid_str: String) -> Result<Uuid, JsValue> {
    Uuid::from_str(&uuid_str)
        .map_err(|e| JsValue::from(format!("Error parsing Uuid from string: {:?}", e)))
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum Role {
    Owner,
    Admin,
    Editor,
    Viewer,
}

impl From<Role> for collaboration::Role {
    fn from(value: Role) -> Self {
        todo!()
    }
}

impl From<collaboration::Role> for Role {
    fn from(value: collaboration::Role) -> Self {
        todo!()
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct EventModelIndexItem {
    document_id: Uuid,
    pub name: String,
    pub description: String,
}

#[wasm_bindgen]
impl EventModelIndexItem {
    #[wasm_bindgen(getter)]
    pub fn document_id(&self) -> String {
        self.document_id.to_string()
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct Invitation {
    document_id: Uuid,
    pub invitor_id: String,
    pub invited_email: String,
    pub invited_role: Role,
}

#[wasm_bindgen]
impl Invitation {
    #[wasm_bindgen(getter)]
    pub fn document_id(&self) -> String {
        self.document_id.to_string()
    }
}

impl From<Invitation> for collaboration::server::Invitation {
    fn from(value: Invitation) -> Self {
        todo!()
    }
}

impl From<collaboration::server::Invitation> for Invitation {
    fn from(value: collaboration::server::Invitation) -> Self {
        todo!()
    }
}

#[wasm_bindgen]
pub struct CollaborationResponse {}

#[wasm_bindgen]
impl AutomergeSyncServerStateManager {
    #[wasm_bindgen(constructor)]
    pub async fn new(
        maybe_id_str: Option<String>,
        user_id: String,
        peer_id: String,
    ) -> Result<AutomergeSyncServerStateManager, JsValue> {
        todo!()
    }

    pub async fn get_initial_message(&mut self) -> Result<Option<Uint8Array>, JsValue> {
        todo!()
    }

    pub async fn get_invitation(&mut self) -> Result<Option<Invitation>, JsValue> {
        todo!()
    }

    pub async fn sync() -> Result<Option<Uint8Array>, JsValue> {
        todo!()
    }

    pub async fn invite_user() -> Result<(), JsValue> {
        todo!()
    }

    pub async fn accept_invitation() -> Result<(), JsValue> {
        todo!()
    }

    pub async fn revoke_access() -> Result<(), JsValue> {
        todo!()
    }

    pub async fn delete_document() -> Result<(), JsValue> {
        todo!()
    }

    async fn dispatch(
        &mut self,
        command: CollaborationCommand,
    ) -> Result<CollaborativeDocument, JsValue> {
        AutomergeSyncServerDecider::execute_reify_decide(&mut self.repository, &(), &command)
            .await
            .map_err(|e| JsValue::from(format!("Error dispatching command {:?}: {:?}", command, e)))
    }
}
