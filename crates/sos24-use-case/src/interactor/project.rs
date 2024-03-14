use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::{
        actor::Actor,
        permission::{PermissionDeniedError, Permissions},
    },
    repository::{
        project::{ProjectRepository, ProjectRepositoryError},
        Repositories,
    },
};
use thiserror::Error;

use crate::dto::{project::CreateProjectDto, ToEntity};

#[derive(Debug, Error)]
pub enum ProjectUseCaseError {
    #[error(transparent)]
    ProjectRepositoryError(#[from] ProjectRepositoryError),
    #[error(transparent)]
    PermissionDeniedError(#[from] PermissionDeniedError),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

pub struct ProjectUseCase<R: Repositories> {
    repositories: Arc<R>,
}

impl<R: Repositories> ProjectUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }

    pub async fn create(
        &self,
        actor: &Actor,
        raw_project: CreateProjectDto,
    ) -> Result<(), ProjectUseCaseError> {
        ensure!(actor.has_permission(Permissions::CREATE_PROJECT));

        // TODO: 企画募集期間かを確認する
        // TODO: すでに別の企画の責任者でないかを確認する

        let project = raw_project.into_entity()?;
        self.repositories
            .project_repository()
            .create(project)
            .await?;
        Ok(())
    }
}
