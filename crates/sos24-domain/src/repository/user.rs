use std::future::Future;

use mockall::automock;

use crate::entity::{
    common::date::WithDate,
    user::{User, UserId},
};

#[automock]
pub trait UserRepository: Send + Sync + 'static {
    fn list(&self) -> impl Future<Output = anyhow::Result<Vec<WithDate<User>>>> + Send;

    fn create(&self, user: User) -> impl Future<Output = anyhow::Result<()>> + Send;

    fn find_by_id(
        &self,
        id: UserId,
    ) -> impl Future<Output = anyhow::Result<Option<WithDate<User>>>> + Send;

    fn update(&self, user: User) -> impl Future<Output = anyhow::Result<()>> + Send;

    fn delete_by_id(&self, id: UserId) -> impl Future<Output = anyhow::Result<()>> + Send;
}
