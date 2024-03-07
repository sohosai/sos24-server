use sos24_domain::entity::{common::email::EmailError, user::UserId};
use thiserror::Error;

use super::UseCaseError;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("User(id = {0:?}) not found")]
    NotFound(UserId),
    #[error(transparent)]
    InvalidEmail(EmailError),
}

impl From<EmailError> for UseCaseError<UserError> {
    fn from(e: EmailError) -> Self {
        UseCaseError::UseCase(UserError::InvalidEmail(e))
    }
}
