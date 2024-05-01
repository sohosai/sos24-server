use mockall::automock;
use thiserror::Error;

use crate::entity::file_data::{FileData, FileId};
use crate::entity::project::ProjectId;

#[derive(Debug, Error)]
pub enum FileDataRepositoryError {
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

#[automock]
#[allow(async_fn_in_trait)]
pub trait FileDataRepository: Send + Sync + 'static {
    async fn list(&self) -> Result<Vec<FileData>, FileDataRepositoryError>;
    async fn create(&self, file_data: FileData) -> Result<(), FileDataRepositoryError>;
    async fn find_by_id(&self, id: FileId) -> Result<Option<FileData>, FileDataRepositoryError>;
    async fn find_by_owner_project(
        &self,
        owner_project: ProjectId,
    ) -> Result<Vec<FileData>, FileDataRepositoryError>;
    async fn delete_by_id(&self, id: FileId) -> Result<(), FileDataRepositoryError>;
    async fn delete_by_owner_project(
        &self,
        owner_project: ProjectId,
    ) -> Result<(), FileDataRepositoryError>;
}
