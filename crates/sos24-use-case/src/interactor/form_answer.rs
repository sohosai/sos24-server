use std::sync::Arc;

use thiserror::Error;

use sos24_domain::{
    entity::{
        form::{FormId, FormIdError},
        form_answer::{FormAnswerId, FormAnswerIdError},
        permission::PermissionDeniedError,
        project::{ProjectId, ProjectIdError},
    },
    repository::{
        form::FormRepositoryError, form_answer::FormAnswerRepositoryError,
        project::ProjectRepositoryError, Repositories,
    },
    service::verify_form_answer::VerifyFormAnswerError,
};

use crate::context::ContextError;

use super::form::FormUseCaseError;

pub mod create;
pub mod find_by_form_id;
pub mod find_by_id;
pub mod find_by_project_id;
pub mod list;
pub mod update;

#[derive(Debug, Error)]
pub enum FormAnswerUseCaseError {
    #[error("Form answer not found: {0:?}")]
    NotFound(FormAnswerId),
    #[error("Project not found: {0:?}")]
    ProjectNotFound(ProjectId),
    #[error("Form not found: {0:?}")]
    FormNotFound(FormId),
    #[error("Already answered")]
    AlreadyAnswered,

    #[error(transparent)]
    FormIdError(#[from] FormIdError),
    #[error(transparent)]
    ProjectIdError(#[from] ProjectIdError),
    #[error(transparent)]
    FormAnswerIdError(#[from] FormAnswerIdError),
    #[error(transparent)]
    VerifyFormAnswerError(#[from] VerifyFormAnswerError),
    #[error(transparent)]
    FormRepositoryError(#[from] FormRepositoryError),
    #[error(transparent)]
    FormUseCaseError(#[from] FormUseCaseError),
    #[error(transparent)]
    FormAnswerRepositoryError(#[from] FormAnswerRepositoryError),
    #[error(transparent)]
    ProjectRepositoryError(#[from] ProjectRepositoryError),
    #[error(transparent)]
    ContextError(#[from] ContextError),
    #[error(transparent)]
    PermissionDeniedError(#[from] PermissionDeniedError),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

pub struct FormAnswerUseCase<R: Repositories> {
    repositories: Arc<R>,
}

impl<R: Repositories> FormAnswerUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }
}
