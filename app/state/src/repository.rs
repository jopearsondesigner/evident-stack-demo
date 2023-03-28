use std::fmt::Debug;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use web_sys::{window, Storage};

use epoch::repository::{
    event::{VersionDiff, VersionedRepositoryError},
    state::VersionedStateRepository,
    RepositoryVersion,
};

#[derive(Debug, Clone)]
pub struct LocalStorageStateRepository<State> {
    key: String,
    default: State,
}

impl<'a, State> LocalStorageStateRepository<State>
where
    State: Serialize + Deserialize<'a> + Debug + Clone + Send + Sync,
{
    pub fn new(key: String, default: State) -> Self {
        Self { key, default }
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
                _ => Err(Error::StorageError),
            },
            None => Err(Error::StorageError),
        }
    }

    fn get_state(&self, storage: Storage) -> Result<(State, RepositoryVersion), Error> {
        match storage.get_item(&self.key) {
            Ok(Some(state_str)) => match from_str(&state_str) {
                Ok(deserialized) => {
                    let state: VersionedState<State> = deserialized;
                    Ok((state.data, state.version))
                }
                Err(_) => Err(Error::StorageError),
            },
            Ok(None) => Ok((self.default, RepositoryVersion::NoStream)),
            Err(_) => Err(Error::StorageError),
        }
    }
}

#[async_trait]
impl<'a, State> VersionedStateRepository<'a, State, Error> for LocalStorageStateRepository<State>
where
    State: Serialize + Deserialize<'a> + Debug + Clone + Send + Sync,
{
    type Version = RepositoryVersion;

    async fn reify(&self) -> Result<(State, Self::Version), Error> {
        let storage = Self::storage()?;
        self.get_state(storage)
    }

    async fn save(
        &mut self,
        version: &Self::Version,
        state: &State,
    ) -> Result<State, VersionedRepositoryError<Error>> {
        let storage = Self::storage().map_err(|e| VersionedRepositoryError::RepoErr(e))?;
        match self.get_state(storage) {
            Ok((_, saved_version)) => {
                let _ = Self::version_check(&saved_version, version)?;

                let serialized = to_string(state)
                    .map_err(|e| VersionedRepositoryError::RepoErr(Error::SerializationError))?;
                storage.set_item(&self.key, &serialized);
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
    State: Debug,
{
    fn new(data: State) -> Self {
        Self {
            data,
            version: RepositoryVersion::StreamExists,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    StorageError,
    SerializationError,
    ExactStreamVersionMustBeKnown,
}
