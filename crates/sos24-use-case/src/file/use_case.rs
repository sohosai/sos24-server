use std::sync::Arc;

use thiserror::Error;

use sos24_domain::entity::file_data::{FileId, FileIdError};
use sos24_domain::entity::form::{FormId, FormIdError, FormItemId};
use sos24_domain::entity::project::{ProjectId, ProjectIdError};
use sos24_domain::repository::file_data::FileDataRepositoryError;
use sos24_domain::repository::file_object::FileObjectRepositoryError;
use sos24_domain::repository::form::FormRepositoryError;
use sos24_domain::repository::form_answer::FormAnswerRepositoryError;
use sos24_domain::repository::project::ProjectRepositoryError;
use sos24_domain::{entity::permission::PermissionDeniedError, repository::Repositories};

use crate::context::ContextError;

pub mod create;
pub mod delete_by_id;
pub mod export_by_form_id;
pub mod export_by_owner;
pub mod find_by_id;
pub mod list;

#[derive(Debug, Error)]
pub enum FileUseCaseError {
    #[error("File not found: {0:?}")]
    NotFound(FileId),
    #[error("Project not found: {0:?}")]
    ProjectNotFound(ProjectId),
    #[error("Owner not found")]
    OwnerNotFound,
    #[error("Form not found: {0:?}")]
    FormNotFound(FormId),
    #[error("Form item not found: {0:?}")]
    FormItemNotFound(FormItemId),

    #[error(transparent)]
    FormRepositoryError(#[from] FormRepositoryError),
    #[error(transparent)]
    FormIdError(#[from] FormIdError),
    #[error(transparent)]
    FormAnswerRepositoryError(#[from] FormAnswerRepositoryError),
    #[error(transparent)]
    FileDataRepositoryError(#[from] FileDataRepositoryError),
    #[error(transparent)]
    FileObjectRepositoryError(#[from] FileObjectRepositoryError),
    #[error(transparent)]
    FileIdError(#[from] FileIdError),
    #[error(transparent)]
    PermissionDeniedError(#[from] PermissionDeniedError),
    #[error(transparent)]
    ContextError(#[from] ContextError),
    #[error(transparent)]
    ProjectRepositoryError(#[from] ProjectRepositoryError),
    #[error(transparent)]
    ProjectIdError(#[from] ProjectIdError),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

pub struct FileUseCase<R: Repositories> {
    repositories: Arc<R>,
}

impl<R: Repositories> FileUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }
}
