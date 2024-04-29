use mockall::automock;
use thiserror::Error;

use crate::entity::news::{News, NewsId};

#[derive(Debug, Error)]
pub enum NewsRepositoryError {
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

#[automock]
#[allow(async_fn_in_trait)]
pub trait NewsRepository: Send + Sync + 'static {
    async fn list(&self) -> Result<Vec<News>, NewsRepositoryError>;
    async fn create(&self, news: News) -> Result<(), NewsRepositoryError>;
    async fn find_by_id(&self, id: NewsId) -> Result<Option<News>, NewsRepositoryError>;
    async fn update(&self, news: News) -> Result<(), NewsRepositoryError>;
    async fn delete_by_id(&self, id: NewsId) -> Result<(), NewsRepositoryError>;
}
