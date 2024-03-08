use std::future::Future;

use mockall::automock;

use crate::entity::{
    common::date::WithDate,
    news_attachment::{NewsAttachment, NewsAttachmentId},
};

#[automock]
pub trait NewsAttachmentRepository: Send + Sync + 'static {
    fn list(&self) -> impl Future<Output = anyhow::Result<Vec<WithDate<NewsAttachment>>>> + Send;
    fn create(
        &self,
        news_attachment: NewsAttachment,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;

    fn find_by_id(
        &self,
        id: NewsAttachmentId,
    ) -> impl Future<Output = anyhow::Result<Option<WithDate<NewsAttachment>>>> + Send;

    fn delete_by_id(&self, id: NewsAttachmentId)
        -> impl Future<Output = anyhow::Result<()>> + Send;
}
