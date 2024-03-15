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

#[derive(Debug, Clone)]
pub struct Context {
    user_id: UserId,

    actor: Option<Actor>, // for test purpose only
}

impl Context {
    pub fn new(user_id: String) -> Self {
        Self {
            user_id: UserId::new(user_id),
            actor: None,
        }
    }

    // for test purpose only
    pub fn with_actor(actor: Actor) -> Self {
        Self {
            user_id: actor.user_id().clone(),
            actor: Some(actor),
        }
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub async fn user<R: Repositories>(
        &self,
        repositories: Arc<R>,
    ) -> Result<WithDate<User>, ContextError> {
        repositories
            .user_repository()
            .find_by_id(self.user_id.clone())
            .await?
            .ok_or(ContextError::UserNotFound(self.user_id.clone()))
    }

    pub async fn actor<R: Repositories>(
        &self,
        repositories: Arc<R>,
    ) -> Result<Actor, ContextError> {
        match self.actor {
            Some(ref actor) => Ok(actor.clone()),
            None => {
                let user = self.user(repositories).await?;
                Ok(Actor::new(self.user_id.clone(), user.value.role().clone()))
            }
        }
    }

    pub async fn project<R: Repositories>(
        &self,
        repositories: Arc<R>,
    ) -> Result<Option<OwnedProject>, ContextError> {
        if let Some(project) = repositories
            .project_repository()
            .find_by_owner_id(self.user_id.clone())
            .await?
        {
            return Ok(Some(OwnedProject::Owner(project)));
        }

        if let Some(project) = repositories
            .project_repository()
            .find_by_sub_owner_id(self.user_id.clone())
            .await?
        {
            return Ok(Some(OwnedProject::SubOwner(project)));
        }

        Ok(None)
    }
}
