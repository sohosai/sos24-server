use std::sync::Arc;

use sos24_domain::{
    entity::{
        actor::Actor,
        common::date::WithDate,
        project::Project,
        user::{User, UserId},
    },
    repository::{
        project::{ProjectRepository, ProjectRepositoryError},
        user::{UserRepository, UserRepositoryError},
        Repositories,
    },
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContextError {
    #[error("User not found: {0:?}")]
    UserNotFound(UserId),

    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
    #[error(transparent)]
    ProjectRepositoryError(#[from] ProjectRepositoryError),
}

#[derive(Debug)]
pub enum OwnedProject {
    Owner(WithDate<Project>),
    SubOwner(WithDate<Project>),
}

#[allow(async_fn_in_trait)]
pub trait ContextProvider: Send + Sync + 'static {
    fn user_id(&self) -> String;
    fn requested_at(&self) -> &chrono::DateTime<chrono::Utc>;

    async fn user<R: Repositories>(
        &self,
        repositories: Arc<R>,
    ) -> Result<WithDate<User>, ContextError> {
        let user_id = UserId::new(self.user_id());
        repositories
            .user_repository()
            .find_by_id(user_id.clone())
            .await?
            .ok_or(ContextError::UserNotFound(user_id))
    }

    async fn actor<R: Repositories>(&self, repositories: Arc<R>) -> Result<Actor, ContextError> {
        let user = self.user(repositories).await?;
        Ok(Actor::new(
            user.value.id().clone(),
            user.value.role().clone(),
        ))
    }

    async fn project<R: Repositories>(
        &self,
        repositories: Arc<R>,
    ) -> Result<Option<OwnedProject>, ContextError> {
        let user_id = UserId::new(self.user_id());
        if let Some(project) = repositories
            .project_repository()
            .find_by_owner_id(user_id.clone())
            .await?
        {
            return Ok(Some(OwnedProject::Owner(project)));
        }

        if let Some(project) = repositories
            .project_repository()
            .find_by_sub_owner_id(user_id)
            .await?
        {
            return Ok(Some(OwnedProject::SubOwner(project)));
        }

        Ok(None)
    }
}

pub struct TestContext {
    actor: Actor,
    requested_at: chrono::DateTime<chrono::Utc>,
}

impl TestContext {
    pub fn new(actor: Actor) -> Self {
        Self {
            actor,
            requested_at: chrono::Utc::now(),
        }
    }
}

impl ContextProvider for TestContext {
    fn user_id(&self) -> String {
        self.actor.user_id().clone().value()
    }

    fn requested_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.requested_at
    }

    async fn actor<R: Repositories>(&self, _repositories: Arc<R>) -> Result<Actor, ContextError> {
        Ok(self.actor.clone())
    }
}
