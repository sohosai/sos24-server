use std::sync::Arc;

use thiserror::Error;

use sos24_domain::{
    entity::{
        common::datetime::DateTimeError,
        form::{FormId, FormIdError, FormItemIdError},
        permission::PermissionDeniedError,
        project::ProjectIdError,
    },
    repository::{form::FormRepositoryError, Repositories},
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
