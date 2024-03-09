use std::convert::Infallible;

use thiserror::Error;

use self::{news::NewsError, user::UserError};

pub mod news;
pub mod news_attachment;
pub mod user;

pub type Result<T, E> = std::result::Result<T, UseCaseError<E>>;

#[derive(Debug, Error)]
pub enum UseCaseError<E: std::error::Error> {
    #[error(transparent)]
    UseCase(E),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

// FIXME
impl From<UseCaseError<Infallible>> for UseCaseError<NewsError> {
    fn from(_: UseCaseError<Infallible>) -> Self {
        unreachable!()
    }
}

impl From<UseCaseError<Infallible>> for UseCaseError<UserError> {
    fn from(_: UseCaseError<Infallible>) -> Self {
        unreachable!()
    }
}
