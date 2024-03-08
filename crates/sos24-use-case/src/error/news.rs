use sos24_domain::entity::{
    news::{NewsId, NewsIdError},
    permission::PermissionDeniedError,
};
use thiserror::Error;

use super::UseCaseError;

#[derive(Debug, Error)]
pub enum NewsError {
    #[error("News not found: {0:?}")]
    NotFound(NewsId),
    #[error(transparent)]
    InvalidNewsId(NewsIdError),
    #[error("Permission denied")]
    PermissionDenied(PermissionDeniedError),
}

impl From<NewsIdError> for UseCaseError<NewsError> {
    fn from(e: NewsIdError) -> Self {
        UseCaseError::UseCase(NewsError::InvalidNewsId(e))
    }
}

impl From<PermissionDeniedError> for UseCaseError<NewsError> {
    fn from(e: PermissionDeniedError) -> Self {
        UseCaseError::UseCase(NewsError::PermissionDenied(e))
    }
}
