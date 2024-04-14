use mockall::automock;
use thiserror::Error;

use crate::entity::firebase_user::{FirebaseUserEmail, FirebaseUserId, NewFirebaseUser};

#[derive(Debug, Error)]
pub enum FirebaseUserRepositoryError {
    #[error("Email already exists: {0:?}")]
    EmailExists(FirebaseUserEmail),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

#[automock]
#[allow(async_fn_in_trait)]
pub trait FirebaseUserRepository: Send + Sync + 'static {
    async fn create(
        &self,
        new_firebase_user: NewFirebaseUser,
    ) -> Result<FirebaseUserId, FirebaseUserRepositoryError>;

    async fn update_email_by_id(
        &self,
        id: FirebaseUserId,
        email: FirebaseUserEmail,
    ) -> Result<(), FirebaseUserRepositoryError>;

    async fn delete_by_id(&self, id: FirebaseUserId) -> Result<(), FirebaseUserRepositoryError>;
}
