use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::{
        common::email::EmailError,
        invitation::{InvitationError, InvitationId, InvitationIdError, InvitationPosition},
        permission::{PermissionDeniedError, Permissions},
        project::{ProjectId, ProjectIdError},
        project_application_period::ProjectApplicationPeriod,
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
    dto::{
        invitation::{CreateInvitationDto, InvitationDto},
        FromEntity, ToEntity,
    },
};

#[derive(Debug, Error)]
pub enum InvitationUseCaseError {
    #[error("Invitation not found: {0:?}")]
    NotFound(InvitationId),
    #[error("Inviter not found: {0:?}")]
    InviterNotFound(UserId),
    #[error("Project not found: {0:?}")]
    ProjectNotFound(ProjectId),

    #[error(transparent)]
    InvitationError(#[from] InvitationError),
    #[error(transparent)]
    InvitationIdError(#[from] InvitationIdError),
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
    project_application_period: ProjectApplicationPeriod, // TODO
}

impl<R: Repositories> InvitationUseCase<R> {
    pub fn new(repositories: Arc<R>, project_application_period: ProjectApplicationPeriod) -> Self {
        Self {
            repositories,
            project_application_period,
        }
    }

    pub async fn list(&self, ctx: &Context) -> Result<Vec<InvitationDto>, InvitationUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_INVITATION_ALL));

        let raw_invitation_list = self.repositories.invitation_repository().list().await?;
        let invitation_list = raw_invitation_list
            .into_iter()
            .map(|invitation| InvitationDto::from_entity(invitation));
        Ok(invitation_list.collect())
    }

    pub async fn create(
        &self,
        ctx: &Context,
        raw_invitation: CreateInvitationDto,
    ) -> Result<(), InvitationUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::CREATE_INVITATION));

        let invitation = raw_invitation.into_entity()?;

        ensure!(
            self.project_application_period.contains(ctx.requested_at())
                || actor.has_permission(Permissions::CREATE_INVITATION_ANYTIME)
        );

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

    pub async fn receive(&self, ctx: &Context, id: String) -> Result<(), InvitationUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;

        // TODO: トランザクションを貼るとより良い

        let id = InvitationId::try_from(id)?;
        let invitation = self
            .repositories
            .invitation_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(InvitationUseCaseError::NotFound(id.clone()))?;

        let project_id = invitation.value.project_id().clone();
        let project = self
            .repositories
            .project_repository()
            .find_by_id(project_id.clone())
            .await?
            .ok_or(InvitationUseCaseError::ProjectNotFound(project_id))?;

        let mut new_project = project.value;
        match invitation.value.position() {
            InvitationPosition::Owner => new_project.set_owner_id(ctx.user_id().clone()),
            InvitationPosition::SubOwner => new_project.set_sub_owner_id(ctx.user_id().clone()),
        }
        self.repositories
            .project_repository()
            .update(new_project)
            .await?;

        let mut new_invitation = invitation.value;
        new_invitation.receive(actor.user_id().clone())?;
        self.repositories
            .invitation_repository()
            .update(new_invitation)
            .await?;

        Ok(())
    }

    pub async fn find_by_id(
        &self,
        ctx: &Context,
        id: String,
    ) -> Result<InvitationDto, InvitationUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;

        let id = InvitationId::try_from(id)?;
        let raw_invitation = self
            .repositories
            .invitation_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(InvitationUseCaseError::NotFound(id.clone()))?;

        ensure!(raw_invitation.value.is_visible_to(&actor));

        Ok(InvitationDto::from_entity(raw_invitation))
    }
}
