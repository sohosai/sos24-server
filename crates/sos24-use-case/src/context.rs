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

#[derive(Clone, Debug, Default)]
pub struct Config {
    pub email_sender_address: String,
    pub email_reply_to_address: String,
    pub app_url: String,
}

#[allow(async_fn_in_trait)]
pub trait ContextProvider: Send + Sync + 'static {
    fn user_id(&self) -> String;
    fn requested_at(&self) -> &chrono::DateTime<chrono::Utc>;
    fn config(&self) -> &Config;

    async fn user<R: Repositories>(
        &self,
        repositories: &R,
    ) -> Result<WithDate<User>, ContextError> {
        let user_id = UserId::new(self.user_id());
        repositories
            .user_repository()
            .find_by_id(user_id.clone())
            .await?
            .ok_or(ContextError::UserNotFound(user_id))
    }

    async fn actor<R: Repositories>(&self, repositories: &R) -> Result<Actor, ContextError> {
        let user = self.user(repositories).await?;
        Ok(Actor::new(
            user.value.id().clone(),
            user.value.role().clone(),
        ))
    }

    async fn project<R: Repositories>(
        &self,
        repositories: &R,
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
    config: Config,
}

impl TestContext {
    pub fn new(actor: Actor) -> Self {
        Self {
            actor,
            requested_at: chrono::Utc::now(),
            config: Config::default(),
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

    fn config(&self) -> &Config {
        &self.config
    }

    async fn actor<R: Repositories>(&self, _repositories: &R) -> Result<Actor, ContextError> {
        Ok(self.actor.clone())
    }
}
