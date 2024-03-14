use mockall::automock;
use thiserror::Error;

use crate::entity::{common::date::WithDate, project::Project};

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
}
