use sos24_domain::entity::{
    common::email::EmailError, permission::PermissionDeniedError, user::UserId,
};
use thiserror::Error;

use super::UseCaseError;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("User not found: {0:?}")]
    NotFound(UserId),
    #[error(transparent)]
    InvalidEmail(EmailError),
    #[error("Permission denied")]
    PermissionDenied(PermissionDeniedError),
}

impl From<EmailError> for UseCaseError<UserError> {
    fn from(e: EmailError) -> Self {
        UseCaseError::UseCase(UserError::InvalidEmail(e))
    }
}

impl From<PermissionDeniedError> for UseCaseError<UserError> {
    fn from(e: PermissionDeniedError) -> Self {
        UseCaseError::UseCase(UserError::PermissionDenied(e))
    }
}
