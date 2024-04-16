use std::sync::Arc;

use sos24_domain::{
    entity::{common::email::EmailError, permission::PermissionDeniedError, user::UserId},
    repository::{
        firebase_user::FirebaseUserRepositoryError, project::ProjectRepositoryError,
        user::UserRepositoryError, Repositories,
    },
};
use thiserror::Error;

use crate::context::ContextError;

pub mod create;
pub mod delete_by_id;
pub mod find_by_id;
pub mod list;
pub mod update;

#[derive(Debug, Error)]
pub enum UserUseCaseError {
    #[error("User not found: {0:?}")]
    NotFound(UserId),

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
