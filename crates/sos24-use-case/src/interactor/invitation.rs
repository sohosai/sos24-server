use std::sync::Arc;

use thiserror::Error;

use sos24_domain::{
    entity::{
        common::email::EmailError,
        invitation::{InvitationError, InvitationId, InvitationIdError},
        permission::PermissionDeniedError,
        project::{ProjectError, ProjectId, ProjectIdError},
        project_application_period::ProjectApplicationPeriod,
        user::UserId,
    },
    repository::{
        invitation::InvitationRepositoryError, project::ProjectRepositoryError,
        Repositories, user::UserRepositoryError,
    },
};

use crate::context::ContextError;

pub mod create;
pub mod delete_by_id;
pub mod find_by_id;
pub mod list;
pub mod receive;

#[derive(Debug, Error)]
pub enum InvitationUseCaseError {
    #[error("Invitation not found: {0:?}")]
    NotFound(InvitationId),
    #[error("Inviter not found: {0:?}")]
    InviterNotFound(UserId),
    #[error("Project not found: {0:?}")]
    ProjectNotFound(ProjectId),
    #[error("Already owner or sub-owner")]
    AlreadyOwnerOrSubOwner,
    #[error("User not found: {0:?}")]
    UserNotFound(UserId),

    #[error(transparent)]
    ProjectError(#[from] ProjectError),
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
}
