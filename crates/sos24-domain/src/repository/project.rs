use mockall::automock;
use thiserror::Error;

use crate::entity::{
    project::{Project, ProjectId},
    user::UserId,
};

#[derive(Debug, Error)]
pub enum ProjectRepositoryError {
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

#[automock]
#[allow(async_fn_in_trait)]
pub trait ProjectRepository: Send + Sync + 'static {
    async fn list(&self) -> Result<Vec<Project>, ProjectRepositoryError>;
    async fn create(&self, project: Project) -> Result<(), ProjectRepositoryError>;
    async fn find_by_id(&self, id: ProjectId) -> Result<Option<Project>, ProjectRepositoryError>;
    async fn find_by_owner_id(
        &self,
        owner_id: UserId,
    ) -> Result<Option<Project>, ProjectRepositoryError>;
    async fn find_by_sub_owner_id(
        &self,
        sub_owner_id: UserId,
    ) -> Result<Option<Project>, ProjectRepositoryError>;
    async fn update(&self, project: Project) -> Result<(), ProjectRepositoryError>;
    async fn delete_by_id(&self, id: ProjectId) -> Result<(), ProjectRepositoryError>;
}
