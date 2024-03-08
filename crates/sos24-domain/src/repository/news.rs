use std::future::Future;

use mockall::automock;
use thiserror::Error;

use crate::entity::{
    common::date::WithDate,
    news::{News, NewsId},
};

#[derive(Debug, Error)]
pub enum NewsRepositoryError {
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

#[automock]
pub trait NewsRepository: Send + Sync + 'static {
    fn list(&self)
        -> impl Future<Output = Result<Vec<WithDate<News>>, NewsRepositoryError>> + Send;

    fn create(&self, news: News) -> impl Future<Output = Result<(), NewsRepositoryError>> + Send;

    fn find_by_id(
        &self,
        id: NewsId,
    ) -> impl Future<Output = Result<Option<WithDate<News>>, NewsRepositoryError>> + Send;

    fn update(&self, news: News) -> impl Future<Output = Result<(), NewsRepositoryError>> + Send;

    fn delete_by_id(
        &self,
        id: NewsId,
    ) -> impl Future<Output = Result<(), NewsRepositoryError>> + Send;
}
