use std::sync::Arc;

use sos24_domain::entity::form::FormItemIdError;
use thiserror::Error;

use sos24_domain::entity::file_data::{FileId, FileIdError};
use sos24_domain::repository::file_data::FileDataRepositoryError;
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

use crate::shared::context::ContextError;

use super::form::FormUseCaseError;

pub mod dto;
pub mod interactor;

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
    #[error("File not found: {0:?}")]
    FileNotFound(FileId),
    #[error("Not a project owner or subowner")]
    NotProjectOwner,
    #[error("Export failed")]
    ExportFailed,
    #[error("Form closed")]
    FormClosed,

    #[error(transparent)]
    FileIdError(#[from] FileIdError),
    #[error(transparent)]
    FileDataRepositoryError(#[from] FileDataRepositoryError),
    #[error(transparent)]
    FormIdError(#[from] FormIdError),
    #[error(transparent)]
    FormItemIdError(#[from] FormItemIdError),
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
    creation_lock: tokio::sync::Mutex<()>, // FIXME
}

impl<R: Repositories> FormAnswerUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self {
            repositories,
            creation_lock: tokio::sync::Mutex::new(()),
        }
    }
}
