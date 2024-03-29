use std::sync::Arc;

use thiserror::Error;

use sos24_domain::{
    entity::{
        common::datetime::DateTimeError,
        form::{FormError, FormId, FormIdError, FormItemIdError},
        permission::PermissionDeniedError,
        project::ProjectIdError,
    },
    repository::{form::FormRepositoryError, form_answer::FormAnswerRepositoryError, Repositories},
};

use crate::context::ContextError;
use crate::interactor::project::ProjectUseCaseError;

pub mod create;
pub mod delete_by_id;
pub mod find_by_id;
pub mod list;
pub mod update;

#[derive(Debug, Error)]
pub enum FormUseCaseError {
    #[error("Form not found: {0:?}")]
    NotFound(FormId),
    #[error("Form has answers")]
    HasAnswers,

    #[error(transparent)]
    FormError(#[from] FormError),
    #[error(transparent)]
    FormAnswerRepositoryError(#[from] FormAnswerRepositoryError),
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

pub struct FormUseCase<R: Repositories> {
    repositories: Arc<R>,
}

impl<R: Repositories> FormUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }
}
