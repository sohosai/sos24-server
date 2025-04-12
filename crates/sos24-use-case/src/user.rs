use std::sync::Arc;

use thiserror::Error;

use sos24_domain::entity::common::email::EmailError;
use sos24_domain::entity::permission::PermissionDeniedError;
use sos24_domain::entity::user::UserId;
use sos24_domain::repository::firebase_user::FirebaseUserRepositoryError;
use sos24_domain::repository::project::ProjectRepositoryError;
use sos24_domain::repository::user::UserRepositoryError;
use sos24_domain::repository::Repositories;

use crate::shared::context::ContextError;

pub mod dto;
pub mod interactor;

#[derive(Debug, Error)]
pub enum UserUseCaseError {
    #[error("User not found: {0:?}")]
    NotFound(UserId),

    #[error("Users already exist")]
    UsersAlreadyExist,

    #[error(transparent)]
    ProjectRepositoryError(#[from] ProjectRepositoryError),
    #[error(transparent)]
    ContextError(#[from] ContextError),
    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
    #[error(transparent)]
    FirebaseUserRepositoryError(#[from] FirebaseUserRepositoryError),
    #[error(transparent)]
    EmailError(#[from] EmailError),
    #[error(transparent)]
    PermissionDeniedError(#[from] PermissionDeniedError),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

pub struct UserUseCase<R: Repositories> {
    repositories: Arc<R>,
}

impl<R: Repositories> UserUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }
}
