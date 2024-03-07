use sos24_domain::entity::news::{NewsId, NewsIdError};
use thiserror::Error;

use super::UseCaseError;

#[derive(Debug, Error)]
pub enum NewsError {
    #[error("News not found: {0:?}")]
    NotFound(NewsId),
    #[error(transparent)]
    InvalidNewsId(NewsIdError),
}

impl From<NewsIdError> for UseCaseError<NewsError> {
    fn from(e: NewsIdError) -> Self {
        UseCaseError::UseCase(NewsError::InvalidNewsId(e))
    }
}
