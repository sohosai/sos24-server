use mockall::automock;
use thiserror::Error;

use crate::entity::news_attachment_object::{
    NewsAttachmentObject, NewsAttachmentObjectKey, NewsAttachmentSignedUrl,
};

#[derive(Debug, Error)]
pub enum NewsAttachmentObjectRepositoryError {
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

#[automock]
#[allow(async_fn_in_trait)]
pub trait NewsAttachmentObjectRepository: Send + Sync + 'static {
    async fn create(
        &self,
        bucket: String,
        news_attachment_object: NewsAttachmentObject,
    ) -> Result<(), NewsAttachmentObjectRepositoryError>;
    async fn generate_url(
        &self,
        bucket: String,
        key: NewsAttachmentObjectKey,
    ) -> Result<NewsAttachmentSignedUrl, NewsAttachmentObjectRepositoryError>;
}
