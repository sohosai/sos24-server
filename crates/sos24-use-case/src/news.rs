use std::sync::Arc;

use sos24_domain::entity::user::UserId;
use sos24_domain::repository::project::ProjectRepositoryError;
use sos24_domain::repository::user::UserRepositoryError;
use thiserror::Error;

use sos24_domain::entity::file_data::{FileId, FileIdError};
use sos24_domain::repository::file_data::FileDataRepositoryError;
use sos24_domain::{
    entity::{
        news::{NewsId, NewsIdError},
        permission::PermissionDeniedError,
    },
    repository::{news::NewsRepositoryError, Repositories},
};

use crate::adapter::Adapters;
use crate::context::ContextError;
use crate::project::ProjectUseCaseError;

pub mod dto;
pub mod interactor;

#[derive(Debug, Error)]
pub enum NewsUseCaseError {
    #[error("News not found: {0:?}")]
    NotFound(NewsId),
    #[error("File not found: {0:?}")]
    FileNotFound(FileId),
    #[error("User not found: {0:?}")]
    UserNotFound(UserId),

    #[error(transparent)]
    ProjectRepositoryError(#[from] ProjectRepositoryError),
    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
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

pub struct NewsUseCase<R: Repositories, A: Adapters> {
    repositories: Arc<R>,
    adapters: Arc<A>,
}

impl<R: Repositories, A: Adapters> NewsUseCase<R, A> {
    pub fn new(repositories: Arc<R>, adapters: Arc<A>) -> Self {
        Self {
            repositories,
            adapters,
        }
    }
}
