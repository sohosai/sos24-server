use thiserror::Error;

pub type Result<T, E> = std::result::Result<T, UseCaseError<E>>;

#[derive(Debug, Error)]
pub enum UseCaseError<E: std::error::Error> {
    #[error(transparent)]
    UseCase(E),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}
