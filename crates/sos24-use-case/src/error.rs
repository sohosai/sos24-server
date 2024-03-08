use std::convert::Infallible;

use thiserror::Error;

use self::{news::NewsError, user::UserError};


pub type Result<T, E> = std::result::Result<T, UseCaseError<E>>;

#[derive(Debug, Error)]
pub enum UseCaseError<E: std::error::Error> {
    #[error(transparent)]
    UseCase(E),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

// FIXME
