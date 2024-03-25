use std::sync::Arc;

use thiserror::Error;

use sos24_domain::entity::common::email::EmailError;
use sos24_domain::entity::permission::PermissionDeniedError;
use sos24_domain::entity::user::UserId;
use sos24_domain::repository::firebase_user::FirebaseUserRepositoryError;
use sos24_domain::repository::Repositories;
use sos24_domain::repository::user::UserRepositoryError;

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
    ContextError(#[from] ContextError),
    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
    #[error(transparent)]
    FirebaseUserRepositoryError(#[from] FirebaseUserRepositoryError),
    #[error(transparent)]
    EmailError(#[from] EmailError),
    #[error(transparent)]
    PermissionDenied(#[from] PermissionDeniedError),
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

