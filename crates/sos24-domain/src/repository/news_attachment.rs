use mockall::automock;
use thiserror::Error;

use crate::entity::{
    common::date::WithDate,
    news_attachment::{NewsAttachment, NewsAttachmentId},
};

#[derive(Debug, Error)]
pub enum NewsAttachmentRepositoryError {
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

#[automock]
#[allow(async_fn_in_trait)]
pub trait NewsAttachmentRepository: Send + Sync + 'static {
    async fn list(&self) -> Result<Vec<WithDate<NewsAttachment>>, NewsAttachmentRepositoryError>;
    async fn create(
        &self,
        news_attachment: NewsAttachment,
    ) -> Result<(), NewsAttachmentRepositoryError>;
    async fn find_by_id(
        &self,
        id: NewsAttachmentId,
    ) -> Result<Option<WithDate<NewsAttachment>>, NewsAttachmentRepositoryError>;
    async fn delete_by_id(&self, id: NewsAttachmentId)
        -> Result<(), NewsAttachmentRepositoryError>;
}
