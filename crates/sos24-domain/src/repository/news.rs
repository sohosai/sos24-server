use std::future::Future;

use mockall::automock;

use crate::entity::{
    common::date::WithDate,
    news::{News, NewsId},
};

#[automock]
pub trait NewsRepository: Send + Sync + 'static {
    fn list(&self) -> impl Future<Output = anyhow::Result<Vec<WithDate<News>>>> + Send;

    fn create(&self, news: News) -> impl Future<Output = anyhow::Result<()>> + Send;

    fn find_by_id(
        &self,
        id: NewsId,
    ) -> impl Future<Output = anyhow::Result<Option<WithDate<News>>>> + Send;

    fn update(&self, news: News) -> impl Future<Output = anyhow::Result<()>> + Send;

    fn delete_by_id(&self, id: NewsId) -> impl Future<Output = anyhow::Result<()>> + Send;
}
