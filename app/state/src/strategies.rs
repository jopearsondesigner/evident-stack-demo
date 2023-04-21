use std::fmt::Debug;

use async_trait::async_trait;
use epoch::decider::{DeciderWithContext, Evolver};

#[async_trait(?Send)]
pub trait StateRepository<State, Err> {
    async fn reify(&mut self) -> Result<State, Err>;
    async fn save(&mut self, state: &State) -> Result<State, Err>;
}

#[async_trait(?Send)]
pub trait ReifyDecideSave
where
    <<Self as ReifyDecideSave>::Decide as DeciderWithContext>::Ctx: Send + Sync,
    <<Self as ReifyDecideSave>::Decide as DeciderWithContext>::Cmd: Send + Sync + Debug,
    <<Self as ReifyDecideSave>::Decide as DeciderWithContext>::Err: Send + Sync,
    <<Self as ReifyDecideSave>::Decide as Evolver>::Evt: Send + Sync,
    <<Self as ReifyDecideSave>::Decide as Evolver>::State: Send + Sync,
{
    type Decide: DeciderWithContext + Send + Sync;

    async fn execute_reify_decide<'a, RepoErr>(
        state_repository: &mut (impl StateRepository<<Self::Decide as Evolver>::State, RepoErr>
                  + Send
                  + Sync),
        ctx: &<<Self as ReifyDecideSave>::Decide as DeciderWithContext>::Ctx,
        cmd: &<<Self as ReifyDecideSave>::Decide as DeciderWithContext>::Cmd,
        retrys: Option<u32>,
    ) -> Result<
        <Self::Decide as Evolver>::State,
        ReifyDecideSaveError<<Self::Decide as DeciderWithContext>::Err, RepoErr>,
    >
    where
        RepoErr: Send + Sync,
    {
        let mut local_state = state_repository
            .reify()
            .await
            .map_err(ReifyDecideSaveError::RepositoryErr)?;

        for r in 1..retrys.unwrap_or(20) {
            let evts = <Self::Decide as DeciderWithContext>::decide(ctx, &local_state, cmd)
                .map_err(ReifyDecideSaveError::DecideErr)?;

            let new_state = evts
                .iter()
                .fold(local_state, <Self::Decide as Evolver>::evolve);

            match state_repository.save(&new_state).await {
                Ok(s) => return Ok(s),
                Err(e) => return Err(ReifyDecideSaveError::RepositoryErr(e)),
            }
        }

        Err(ReifyDecideSaveError::OccMaxRetries)
    }
}

#[derive(Debug)]
pub enum ReifyDecideSaveError<DecideErr: Send + Sync, RepoErr> {
    OccMaxRetries,
    DecideErr(DecideErr),
    RepositoryErr(RepoErr),
}
