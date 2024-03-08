use std::future::Future;

use mockall::automock;
use thiserror::Error;

use crate::entity::firebase_user::{FirebaseUserId, NewFirebaseUser};

#[derive(Debug, Error)]
pub enum FirebaseUserRepositoryError {
    #[error("Email already exists")]
    EmailExists,
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

#[automock]
pub trait FirebaseUserRepository: Send + Sync + 'static {
    fn create(
        &self,
        new_firebase_user: NewFirebaseUser,
    ) -> impl Future<Output = Result<FirebaseUserId, FirebaseUserRepositoryError>> + Send;

    fn delete_by_id(
        &self,
        id: FirebaseUserId,
    ) -> impl Future<Output = Result<(), FirebaseUserRepositoryError>> + Send;
}
