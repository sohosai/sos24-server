use sos24_domain::entity::news::{NewsId, NewsIdError};
use thiserror::Error;

use super::UseCaseError;

#[derive(Debug, Error)]
pub enum NewsError {
    #[error("News(id = {0:?}) not found")]
    NotFound(NewsId),
    #[error(transparent)]
    InvalidNewsId(NewsIdError),
}

impl From<NewsIdError> for UseCaseError<NewsError> {
    fn from(e: NewsIdError) -> Self {
        UseCaseError::UseCase(NewsError::InvalidNewsId(e))
    }
}
