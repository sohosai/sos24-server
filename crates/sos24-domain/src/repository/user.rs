use mockall::automock;

use crate::entity::{
    common::date::WithDate,
    user::{User, UserId},
};

#[automock]
pub trait UserRepository: Send + Sync + 'static {
    async fn create(&self, user: User) -> anyhow::Result<()>;

    async fn find_by_id(&self, id: UserId) -> anyhow::Result<Option<WithDate<User>>>;
}
