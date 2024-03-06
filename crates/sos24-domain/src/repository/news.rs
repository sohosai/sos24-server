use crate::entity::news::{News, NewsId};

pub trait NewsRepository: Send + Sync + 'static {
    async fn list(&self) -> anyhow::Result<Vec<News>>;

    async fn create(&self, news: News) -> anyhow::Result<()>;

    async fn find_by_id(&self, id: NewsId) -> anyhow::Result<Option<News>>;

    async fn update(&self, news: News) -> anyhow::Result<()>;

    async fn delete_by_id(&self, id: NewsId) -> anyhow::Result<()>;
}