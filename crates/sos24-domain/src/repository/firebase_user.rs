use std::future::Future;

use mockall::automock;

use crate::entity::firebase_user::{FirebaseUserId, NewFirebaseUser};

#[automock]
pub trait FirebaseUserRepository: Send + Sync + 'static {
    fn create(
        &self,
        new_firebase_user: NewFirebaseUser,
    ) -> impl Future<Output = anyhow::Result<FirebaseUserId>> + Send;

    fn delete_by_id(&self, id: FirebaseUserId) -> impl Future<Output = anyhow::Result<()>> + Send;
}
