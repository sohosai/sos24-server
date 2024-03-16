use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::{
        common::email::EmailError,
        permission::{PermissionDeniedError, Permissions},
        project::{ProjectId, ProjectIdError},
    },
    repository::{
        invitation::{InvitationRepository, InvitationRepositoryError},
        project::{ProjectRepository, ProjectRepositoryError},
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

        if self
            .repositories
            .project_repository()
            .find_by_id(invitation.project_id().clone())
            .await?
            .is_none()
        {
            return Err(InvitationUseCaseError::ProjectNotFound(
                invitation.project_id().clone(),
            ));
        }

        self.repositories
            .invitation_repository()
            .create(invitation)
            .await?;

        Ok(())
    }
}
