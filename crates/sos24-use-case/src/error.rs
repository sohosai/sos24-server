pub type Result<T, E> = std::result::Result<T, UseCaseError<E>>;

#[derive(Debug)]
pub enum UseCaseError<E> {
    UseCase(E),
    Internal(anyhow::Error),
}

impl<E> From<anyhow::Error> for UseCaseError<E> {
    fn from(error: anyhow::Error) -> Self {
        UseCaseError::Internal(error)
    }
}
