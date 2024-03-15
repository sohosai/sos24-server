use mockall::automock;
use thiserror::Error;

use crate::entity::{
    common::date::WithDate,
    project::{Project, ProjectId},
};

#[derive(Debug, Error)]
pub enum ProjectRepositoryError {
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

#[automock]
#[allow(async_fn_in_trait)]
pub trait ProjectRepository: Send + Sync + 'static {
    async fn list(&self) -> Result<Vec<WithDate<Project>>, ProjectRepositoryError>;
    async fn create(&self, project: Project) -> Result<(), ProjectRepositoryError>;
    async fn find_by_id(
        &self,
        id: ProjectId,
    ) -> Result<Option<WithDate<Project>>, ProjectRepositoryError>;
    async fn update(&self, project: Project) -> Result<(), ProjectRepositoryError>;
    async fn delete_by_id(&self, id: ProjectId) -> Result<(), ProjectRepositoryError>;
}
