use std::fmt::Debug;

use async_trait::async_trait;
use automerge::AutoCommit;
use epoch::repository::state::StateRepository;
use js_sys::Promise;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

use crate::HasKey;

// #[wasm_bindgen(module = "$lib/firebase/firestore")]
// extern "C" {
//     type QuerySnapshot;
//     fn patches(id: &str) -> Promise<QuerySnapshot>;
// }

#[derive(Debug, Clone)]
pub struct FirestoreAutomergeStateRepository {
    key: Option<Uuid>,
    automerge: AutoCommit,
}

impl FirestoreAutomergeStateRepository {
    // TODO: provide actorId from outside?
    pub fn load(id: &Uuid) -> Self {
        let data: &[u8] = &[];
        let automerge = AutoCommit::load(data).unwrap();
        FirestoreAutomergeStateRepository {
            key: Some(id.to_owned()),
            automerge,
        }
    }

    pub fn load_incremental(&mut self, data: &[u8]) {
        self.automerge.load_incremental(data).unwrap();
    }

    pub fn save(&mut self) -> Vec<u8> {
        self.automerge.save()
    }

    pub fn save_incremental(&mut self) -> Vec<u8> {
        self.automerge.save_incremental()
    }
}

#[async_trait]
impl<State, Err> StateRepository<State, Err> for FirestoreAutomergeStateRepository
where
    State: HasKey + Debug + Send + Sync,
{
    async fn reify(&self) -> State {
        todo!()
    }
    async fn save(&mut self, state: &State) -> Result<State, Err> {
        todo!()
    }
}
