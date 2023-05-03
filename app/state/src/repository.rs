use std::fmt::Debug;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{from_str, to_string};
use web_sys::{window, Storage};

use epoch::repository::{
    event::{VersionDiff, VersionedRepositoryError},
    state::VersionedStateRepository,
    RepositoryVersion,
};

#[derive(Debug, Clone)]
pub struct LocalStorageStateRepository<State: HasKey + Debug> {
    key: Option<String>,
    initial: State,
}

pub trait HasKey {
    fn get_key(&self) -> Option<String>;
}

impl<State> LocalStorageStateRepository<State>
where
    State: HasKey + Serialize + DeserializeOwned + Debug + Clone + Send + Sync,
{
    pub fn new(key: Option<String>, initial: State) -> Self {
        Self { key, initial }
    }

    fn version_to_usize(
        version: &RepositoryVersion,
    ) -> Result<usize, VersionedRepositoryError<Error>> {
        match version {
            RepositoryVersion::Exact(exact) => Ok(exact.to_owned()),
            RepositoryVersion::NoStream => Ok(0),
            RepositoryVersion::StreamExists => Ok(0),
            RepositoryVersion::Any => Err(VersionedRepositoryError::RepoErr(
                Error::ExactStreamVersionMustBeKnown,
            )),
        }
    }

    fn version_check(
        current: &RepositoryVersion,
        incoming: &RepositoryVersion,
    ) -> Result<(), VersionedRepositoryError<Error>> {
        if Self::version_to_usize(current)? == Self::version_to_usize(incoming)? {
            Ok(())
        } else {
            Err(VersionedRepositoryError::VersionConflict(VersionDiff::new(
                current.to_owned(),
                incoming.to_owned(),
            )))
        }
    }

    fn bump_version(
        version: &RepositoryVersion,
    ) -> Result<RepositoryVersion, VersionedRepositoryError<Error>> {
        Ok(RepositoryVersion::Exact(
            Self::version_to_usize(version)? + 1,
        ))
    }

    fn storage() -> Result<Storage, Error> {
        match window() {
            Some(win) => match win.local_storage() {
                Ok(Some(storage)) => Ok(storage),
                _ => Err(Error::StorageFailure),
            },
            None => Err(Error::StorageFailure),
        }
    }

    fn get_state(&self, storage: &Storage) -> Result<VersionedState<State>, Error> {
        match &self.key {
            Some(key) => match storage.get_item(key) {
                Ok(Some(state_str)) => match from_str(&state_str) {
                    Ok(deserialized) => {
                        let state: VersionedState<State> = deserialized;
                        Ok(state)
                    }
                    Err(_) => Err(Error::StorageFailure),
                },
                Ok(None) => Ok(VersionedState::new(self.initial.to_owned())),
                Err(_) => Err(Error::StorageFailure),
            },
            None => Ok(VersionedState::new(self.initial.to_owned())),
        }
    }
}

#[async_trait]
impl<'a, State> VersionedStateRepository<'a, State, Error> for LocalStorageStateRepository<State>
where
    State: HasKey + Serialize + DeserializeOwned + Debug + Clone + Send + Sync,
{
    type Version = RepositoryVersion;

    async fn reify(&self) -> Result<(State, Self::Version), Error> {
        let storage = Self::storage()?;
        let state = self.get_state(&storage)?;
        Ok((state.data, state.version))
    }

    async fn save(
        &mut self,
        version: &Self::Version,
        state: &State,
    ) -> Result<State, VersionedRepositoryError<Error>> {
        let storage = Self::storage().map_err(VersionedRepositoryError::RepoErr)?;
        match self.get_state(&storage) {
            Ok(mut saved_state) => {
                let _ = Self::version_check(&saved_state.version, version)?;
                saved_state.version = Self::bump_version(&saved_state.version)?;
                saved_state.data = state.to_owned();

                self.key = saved_state.data.get_key();

                if let Some(key) = &self.key {
                    let serialized = to_string(&saved_state).map_err(|_| {
                        VersionedRepositoryError::RepoErr(Error::SerializationFailure)
                    })?;
                    let _ = storage
                        .set_item(key, &serialized)
                        .map_err(|_| VersionedRepositoryError::RepoErr(Error::StorageFailure))?;
                }
                Ok(state.to_owned())
            }
            Err(e) => Err(VersionedRepositoryError::RepoErr(e)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VersionedState<State>
where
    State: Debug,
{
    data: State,
    version: RepositoryVersion,
}

impl<State> VersionedState<State>
where
    State: HasKey + Debug,
{
    fn new(data: State) -> Self {
        Self {
            data,
            version: RepositoryVersion::StreamExists,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    StorageFailure,
    SerializationFailure,
    ExactStreamVersionMustBeKnown,
}
