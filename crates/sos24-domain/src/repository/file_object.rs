use mockall::automock;
use thiserror::Error;
use tokio::io::DuplexStream;

use crate::entity::file_object::{
    ArchiveEntry, ContentDisposition, FileObject, FileObjectKey, FileSignedUrl,
};

#[derive(Debug, Error)]
pub enum FileObjectRepositoryError {
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

#[automock]
#[allow(async_fn_in_trait)]
pub trait FileObjectRepository: Send + Sync + 'static {
    async fn create(
        &self,
        bucket: String,
        file_object: FileObject,
    ) -> Result<(), FileObjectRepositoryError>;
    async fn generate_url(
        &self,
        bucket: String,
        key: FileObjectKey,
        content_disposition: Option<ContentDisposition>,
    ) -> Result<FileSignedUrl, FileObjectRepositoryError>;
    // TODO: 返り値をラッピングしておくと内部仕様が露出しなくてよい
    async fn create_archive(
        &self,
        bucket: String,
        entry_list: Vec<ArchiveEntry>,
    ) -> Result<DuplexStream, FileObjectRepositoryError>;
}
