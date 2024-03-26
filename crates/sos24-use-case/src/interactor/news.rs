use std::sync::Arc;

use thiserror::Error;

use sos24_domain::{
    entity::{
        news::{NewsId, NewsIdError},
        permission::PermissionDeniedError,
    },
    repository::{news::NewsRepositoryError, Repositories},
};

use crate::context::ContextError;
use crate::interactor::project::ProjectUseCaseError;

pub mod create;
pub mod delete_by_id;
pub mod find_by_id;
pub mod list;
pub mod update;

#[derive(Debug, Error)]
pub enum NewsUseCaseError {
    #[error("News not found: {0:?}")]
    NotFound(NewsId),

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
