use mockall::automock;

use crate::entity::firebase_user::{FirebaseUserId, NewFirebaseUser};

#[automock]
pub trait FirebaseUserRepository: Send + Sync + 'static {
    async fn create(&self, new_firebase_user: NewFirebaseUser) -> anyhow::Result<FirebaseUserId>;

    async fn delete_by_id(&self, id: FirebaseUserId) -> anyhow::Result<()>;
}
