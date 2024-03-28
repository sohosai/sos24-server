use std::sync::Arc;

use sos24_domain::entity::file_data::{FileId, FileIdError};
use sos24_domain::entity::project::{ProjectId, ProjectIdError};
use sos24_domain::repository::file_data::FileDataRepositoryError;
use sos24_domain::repository::file_object::FileObjectRepositoryError;
use sos24_domain::repository::project::ProjectRepositoryError;
use sos24_domain::{entity::permission::PermissionDeniedError, repository::Repositories};
use thiserror::Error;

use crate::context::ContextError;

pub mod create;
pub mod delete_by_id;
pub mod find_by_id;
pub mod list;

#[derive(Debug, Error)]
pub enum FileUseCaseError {
    #[error("File not found: {0:?}")]
    NotFound(FileId),
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
    #[error("Project not found: {0:?}")]
    ProjectNotFound(ProjectId),
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
