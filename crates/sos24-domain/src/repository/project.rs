use mockall::automock;
use thiserror::Error;

use crate::entity::{
    project::{Project, ProjectId},
    user::{User, UserId},
};

#[derive(Debug, Error)]
pub enum ProjectRepositoryError {
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct ProjectWithOwners {
    pub project: Project,
    pub owner: User,
    pub sub_owner: Option<User>,
}

#[automock]
#[allow(async_fn_in_trait)]
pub trait ProjectRepository: Send + Sync + 'static {
    // command
    async fn create(&self, project: Project) -> Result<(), ProjectRepositoryError>;
    async fn update(&self, project: Project) -> Result<(), ProjectRepositoryError>;
    async fn delete_by_id(&self, id: ProjectId) -> Result<(), ProjectRepositoryError>;

    // query
    async fn list(&self) -> Result<Vec<ProjectWithOwners>, ProjectRepositoryError>;
    async fn find_by_id(
        &self,
        id: ProjectId,
    ) -> Result<Option<ProjectWithOwners>, ProjectRepositoryError>;
    async fn find_by_owner_id(
        &self,
        owner_id: UserId,
    ) -> Result<Option<ProjectWithOwners>, ProjectRepositoryError>;
    async fn find_by_sub_owner_id(
        &self,
        sub_owner_id: UserId,
    ) -> Result<Option<ProjectWithOwners>, ProjectRepositoryError>;
}
