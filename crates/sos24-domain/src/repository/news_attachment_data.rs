use mockall::automock;
use thiserror::Error;

use crate::entity::{
    common::date::WithDate,
    news_attachment_data::{NewsAttachmentData, NewsAttachmentId},
};

#[derive(Debug, Error)]
pub enum NewsAttachmentRepositoryError {
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

#[automock]
#[allow(async_fn_in_trait)]
pub trait NewsAttachmentRepository: Send + Sync + 'static {
    async fn list(
        &self,
    ) -> Result<Vec<WithDate<NewsAttachmentData>>, NewsAttachmentRepositoryError>;
    async fn create(
        &self,
        news_attachment: NewsAttachmentData,
    ) -> Result<(), NewsAttachmentRepositoryError>;
    async fn find_by_id(
        &self,
        id: NewsAttachmentId,
    ) -> Result<Option<WithDate<NewsAttachmentData>>, NewsAttachmentRepositoryError>;
    async fn delete_by_id(&self, id: NewsAttachmentId)
        -> Result<(), NewsAttachmentRepositoryError>;
}
