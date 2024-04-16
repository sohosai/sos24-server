use std::sync::Arc;

use sos24_domain::{
    entity::{
        file_data::{FileId, FileIdError},
        news::{NewsId, NewsIdError},
        permission::PermissionDeniedError,
    },
    repository::{file_data::FileDataRepositoryError, news::NewsRepositoryError, Repositories},
};
use thiserror::Error;

use crate::{context::ContextError, project::use_case::ProjectUseCaseError};

pub mod create;
pub mod delete_by_id;
pub mod find_by_id;
pub mod list;
pub mod update;

#[derive(Debug, Error)]
pub enum NewsUseCaseError {
    #[error("News not found: {0:?}")]
    NotFound(NewsId),
    #[error("File not found: {0:?}")]
    FileNotFound(FileId),

    #[error(transparent)]
    FileDataRepositoryError(#[from] FileDataRepositoryError),
    #[error(transparent)]
    FileIdError(#[from] FileIdError),
    #[error(transparent)]
    ProjectUseCaseError(#[from] ProjectUseCaseError),
    #[error(transparent)]
    ContextError(#[from] ContextError),
    #[error(transparent)]
    NewsRepositoryError(#[from] NewsRepositoryError),
    #[error(transparent)]
    NewsIdError(#[from] NewsIdError),
    #[error(transparent)]
    PermissionDeniedError(#[from] PermissionDeniedError),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

pub struct NewsUseCase<R: Repositories> {
    repositories: Arc<R>,
}

impl<R: Repositories> NewsUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }
}
