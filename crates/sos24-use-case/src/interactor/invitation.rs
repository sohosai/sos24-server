use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::{
        common::email::EmailError,
        permission::{PermissionDeniedError, Permissions},
        project::{ProjectId, ProjectIdError},
        user::UserId,
    },
    repository::{
        invitation::{InvitationRepository, InvitationRepositoryError},
        project::{ProjectRepository, ProjectRepositoryError},
        user::{UserRepository, UserRepositoryError},
        Repositories,
    },
};
use thiserror::Error;

use crate::{
    context::{Context, ContextError},
    dto::{invitation::CreateInvitationDto, ToEntity},
};

#[derive(Debug, Error)]
pub enum InvitationUseCaseError {
    #[error("Inviter not found: {0:?}")]
    InviterNotFound(UserId),
    #[error("Project not found: {0:?}")]
    ProjectNotFound(ProjectId),

    #[error(transparent)]
    ProjectIdError(#[from] ProjectIdError),
    #[error(transparent)]
    EmailError(#[from] EmailError),
    #[error(transparent)]
    InvitationRepositoryError(#[from] InvitationRepositoryError),
    #[error(transparent)]
    ProjectRepositoryError(#[from] ProjectRepositoryError),
    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
    #[error(transparent)]
    ContextError(#[from] ContextError),
    #[error(transparent)]
    PermissionDeniedError(#[from] PermissionDeniedError),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

pub struct InvitationUseCase<R: Repositories> {
    repositories: Arc<R>,
}

impl<R: Repositories> InvitationUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }

    pub async fn create(
        &self,
        ctx: &Context,
        raw_invitation: CreateInvitationDto,
    ) -> Result<(), InvitationUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::CREATE_INVITATION));

        let invitation = raw_invitation.into_entity()?;

        self.repositories
            .user_repository()
            .find_by_id(invitation.inviter().clone())
            .await?
            .ok_or(InvitationUseCaseError::InviterNotFound(
                invitation.inviter().clone(),
            ))?;

        self.repositories
            .project_repository()
            .find_by_id(invitation.project_id().clone())
            .await?
            .ok_or(InvitationUseCaseError::ProjectNotFound(
                invitation.project_id().clone(),
            ))?;

        self.repositories
            .invitation_repository()
            .create(invitation)
            .await?;

        Ok(())
    }
}
