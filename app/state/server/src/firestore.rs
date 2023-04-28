use std::{collections::HashMap, fmt::Debug};

use async_trait::async_trait;
use automerge::AutoCommit;
use collaboration::{
    server::{CollaborativeDocument, Grant, Invitation, Peer, PeerDocument},
    Email,
};
use js_sys::Uint8Array;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

use state_shared::strategies::StateRepository;
use web_sys::{Request, RequestInit};

use crate::EventModelIndexItem;

#[wasm_bindgen(module = "$lib/firebase/admin/firestore")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn patches(id: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn appendPatch(id: &str, patch: Vec<u8>) -> Result<(), JsValue>;
}

#[derive(Debug, Clone)]
pub(crate) enum FirestoreError {
    NotFound,
    NotAuthorized,
    PatchLoadError(String),
    HydrateError(String),
    ReconcileError(String),
    PatchSaveError(String),
}

#[derive(Debug, Clone)]
pub(crate) struct FirestoreAutomergeStateRepository {
    document_id: Option<Uuid>,
    user_id: String,
    firebase_token: String,
    firebase_db_url: String,
}

enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl FirestoreAutomergeStateRepository {
    pub fn new(
        document_id: Option<Uuid>,
        user_id: String,
        firebase_token: String,
        firebase_db_url: String,
    ) -> Self {
        Self {
            document_id,
            user_id,
            firebase_token,
            firebase_db_url,
        }
    }

    async fn reify_state(&mut self) -> Result<CollaborativeDocument, FirestoreError> {
        match self.document_id {
            Some(document_id) => match self.user_model(&document_id).await? {
                Some(doc) => Ok(CollaborativeDocument::PeerDocument(doc)),
                None => Err(FirestoreError::NotFound),
            },
            None => Ok(CollaborativeDocument::BeforeCreation),
        }
    }

    async fn save_state(
        &mut self,
        state: &CollaborativeDocument,
    ) -> Result<CollaborativeDocument, FirestoreError> {
        match state {
            CollaborativeDocument::BeforeCreation => (),
            CollaborativeDocument::PeerDocument(doc) => {
                let tx_id = self.save_patch(&doc.document).await?;
                self.save_peer(&doc.peer, &tx_id).await?;
                self.grant_roles(&doc.roles_granted, &tx_id).await?;
                self.revoke_roles(&doc.roles_revoked, &tx_id).await?;
                self.add_invitations(&doc.invitations_added, &tx_id).await?;
                self.remove_invitations(&doc.invitations_removed, &tx_id)
                    .await?;
                self.commit_transaction(tx_id).await?;
            }
            CollaborativeDocument::Deleted(document_id) => {
                self.delete_model(document_id).await?;
            }
        }
        Ok(state.to_owned())
    }

    async fn firestore_request(&self, path: &str, method: HttpMethod) -> Result<Request, JsValue> {
        let mut opts = RequestInit::new();
        match method {
            HttpMethod::GET => opts.method("GET"),
            HttpMethod::POST => opts.method("POST"),
            HttpMethod::PUT => opts.method("PUT"),
            HttpMethod::DELETE => opts.method("DELETE"),
        };

        let url = format!("{}/{}", &self.firebase_db_url, path);
        let request = Request::new_with_str_and_init(&url, &opts)?;

        request.headers().set("Accept", "application/json")?;
        request
            .headers()
            .set("Authorization", &format!("Bearer {}", &self.firebase_token))?;

        Ok(request)
    }

    async fn user_model_index(&self) -> Result<Vec<EventModelIndexItem>, FirestoreError> {
        todo!()
    }

    async fn user_model(&self, document_id: &Uuid) -> Result<Option<PeerDocument>, FirestoreError> {
        todo!()
    }

    async fn save_patch(&self, doc: &AutoCommit) -> Result<String, FirestoreError> {
        todo!()
    }

    async fn save_peer(&self, peer: &Option<Peer>, tx_id: &String) -> Result<(), FirestoreError> {
        match peer {
            Some(p) => todo!(),
            None => todo!(),
        }
    }

    async fn grant_roles(
        &self,
        roles_granted: &HashMap<String, Grant>,
        tx_id: &String,
    ) -> Result<(), FirestoreError> {
        todo!()
    }

    async fn revoke_roles(
        &self,
        roles_revoked: &HashMap<String, String>,
        tx_id: &String,
    ) -> Result<(), FirestoreError> {
        todo!()
    }

    async fn add_invitations(
        &self,
        invitations_added: &HashMap<Email, Invitation>,
        tx_id: &String,
    ) -> Result<(), FirestoreError> {
        todo!()
    }

    async fn remove_invitations(
        &self,
        invitations_removed: &HashMap<Email, String>,
        tx_id: &String,
    ) -> Result<(), FirestoreError> {
        todo!()
    }

    async fn commit_transaction(&self, tx_id: String) -> Result<(), FirestoreError> {
        todo!()
    }

    async fn delete_model(&self, document_id: &Uuid) -> Result<(), FirestoreError> {
        todo!()
    }
}

#[async_trait(?Send)]
impl StateRepository<CollaborativeDocument, FirestoreError> for FirestoreAutomergeStateRepository {
    // TODO: distinguish BeforeCreation, EventModel(m), and Deleted(id), especially Deleted
    async fn reify(&mut self) -> Result<CollaborativeDocument, FirestoreError> {
        self.reify_state().await
    }

    async fn save(
        &mut self,
        state: &CollaborativeDocument,
    ) -> Result<CollaborativeDocument, FirestoreError> {
        self.save_state(state).await
    }
}
