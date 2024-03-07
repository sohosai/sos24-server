use mockall::automock;

use crate::entity::{
    common::date::WithDate,
    news::{News, NewsId},
};

#[automock]
pub trait NewsRepository: Send + Sync + 'static {
    async fn list(&self) -> anyhow::Result<Vec<WithDate<News>>>;

    async fn create(&self, news: News) -> anyhow::Result<()>;

    async fn find_by_id(&self, id: NewsId) -> anyhow::Result<Option<WithDate<News>>>;

    async fn update(&self, news: News) -> anyhow::Result<()>;

    async fn delete_by_id(&self, id: NewsId) -> anyhow::Result<()>;
}
