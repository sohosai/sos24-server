use std::sync::Arc;

use sos24_domain::entity::user::UserId;
use sos24_domain::repository::user::UserRepositoryError;
use thiserror::Error;

use sos24_domain::entity::file_data::FileIdError;
use sos24_domain::entity::project::ProjectId;
use sos24_domain::repository::project::ProjectRepositoryError;
use sos24_domain::{
    entity::{
        common::datetime::DateTimeError,
        form::{FormError, FormId, FormIdError, FormItemIdError},
        permission::PermissionDeniedError,
        project::ProjectIdError,
    },
    repository::{form::FormRepositoryError, form_answer::FormAnswerRepositoryError, Repositories},
};

use crate::project::ProjectUseCaseError;
use crate::shared::adapter::Adapters;
use crate::shared::context::ContextError;

pub mod dto;
pub mod interactor;

#[derive(Debug, Error)]
pub enum FormUseCaseError {
    #[error("Form not found: {0:?}")]
    NotFound(FormId),
    #[error("Project not found: {0:?}")]
    ProjectNotFound(ProjectId),
    #[error("Form has answers")]
    HasAnswers,
    #[error("User not found: {0:?}")]
    UserNotFound(UserId),

    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
    #[error(transparent)]
    ProjectRepositoryError(#[from] ProjectRepositoryError),
    #[error(transparent)]
    FormError(#[from] FormError),
    #[error(transparent)]
    FormAnswerRepositoryError(#[from] FormAnswerRepositoryError),
    #[error(transparent)]
    FileIdError(#[from] FileIdError),
    #[error(transparent)]
    ProjectUseCaseError(#[from] ProjectUseCaseError),
    #[error(transparent)]
    ProjectIdError(#[from] ProjectIdError),
    #[error(transparent)]
    FormIdError(#[from] FormIdError),
    #[error(transparent)]
    FormItemIdError(#[from] FormItemIdError),
    #[error(transparent)]
    DateTimeError(#[from] DateTimeError),
    #[error(transparent)]
    FormRepositoryError(#[from] FormRepositoryError),
    #[error(transparent)]
    ContextError(#[from] ContextError),
    #[error(transparent)]
    PermissionDeniedError(#[from] PermissionDeniedError),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

pub struct FormUseCase<R: Repositories, A: Adapters> {
    repositories: Arc<R>,
    adapters: Arc<A>,
}

impl<R: Repositories, A: Adapters> FormUseCase<R, A> {
    pub fn new(repositories: Arc<R>, adapters: Arc<A>) -> Self {
        Self {
            repositories,
            adapters,
        }
    }
}
