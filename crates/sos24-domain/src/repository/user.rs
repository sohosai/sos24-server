use mockall::automock;

use crate::entity::user::User;

#[automock]
pub trait UserRepository: Send + Sync + 'static {
    async fn create(&self, user: User) -> anyhow::Result<()>;
}
