use mockall::automock;
use thiserror::Error;

use crate::entity::file_object::{
    FileObject, FileObjectKey, FileSignedUrl,
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
    ) -> Result<FileSignedUrl, FileObjectRepositoryError>;
}
