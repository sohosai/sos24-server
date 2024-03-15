use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::{
        actor::Actor,
        permission::{PermissionDeniedError, Permissions},
        project::{ProjectId, ProjectIdError},
    },
    repository::{
        project::{ProjectRepository, ProjectRepositoryError},
        Repositories,
    },
};
use thiserror::Error;

use crate::dto::{
    project::{CreateProjectDto, ProjectDto},
    FromEntity, ToEntity,
};

#[derive(Debug, Error)]
pub enum ProjectUseCaseError {
    #[error("Project not found: {0:?}")]
    NotFound(ProjectId),

    #[error(transparent)]
    ProjectRepositoryError(#[from] ProjectRepositoryError),
    #[error(transparent)]
    ProjectIdError(#[from] ProjectIdError),
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

    pub async fn list(&self, actor: &Actor) -> Result<Vec<ProjectDto>, ProjectUseCaseError> {
        ensure!(actor.has_permission(Permissions::READ_PROJECT_ALL));

        let raw_project_list = self.repositories.project_repository().list().await?;
        let project_list = raw_project_list.into_iter().map(ProjectDto::from_entity);
        Ok(project_list.collect())
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

    pub async fn find_by_id(
        &self,
        actor: &Actor,
        id: String,
    ) -> Result<ProjectDto, ProjectUseCaseError> {
        let id = ProjectId::try_from(id)?;
        let raw_project = self
            .repositories
            .project_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(ProjectUseCaseError::NotFound(id))?;

        ensure!(raw_project.value.is_visible_to(actor));

        Ok(ProjectDto::from_entity(raw_project))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::{
        entity::{
            permission::PermissionDeniedError,
            project::{ProjectAttributes, ProjectCategory},
            user::UserRole,
        },
        test::{fixture, repository::MockRepositories},
    };

    use crate::{
        dto::project::{CreateProjectDto, ProjectCategoryDto},
        interactor::project::{ProjectUseCase, ProjectUseCaseError},
    };

    #[tokio::test]
    async fn list_general_fail() {
        let repositories = MockRepositories::default();
        let use_case = ProjectUseCase::new(Arc::new(repositories));

        let actor = fixture::actor::actor1(UserRole::General);
        let res = use_case.list(&actor).await;
        assert!(matches!(
            res,
            Err(ProjectUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn list_committee_success() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_list()
            .returning(|| Ok(vec![]));
        let use_case = ProjectUseCase::new(Arc::new(repositories));

        let actor = fixture::actor::actor1(UserRole::Committee);
        let res = use_case.list(&actor).await;
        assert!(matches!(res, Ok(list) if list.is_empty()));
    }

    #[tokio::test]
    async fn create_success() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = ProjectUseCase::new(Arc::new(repositories));

        let actor = fixture::actor::actor1(UserRole::General);
        let res = use_case
            .create(
                &actor,
                CreateProjectDto::new(
                    fixture::project::title().value(),
                    fixture::project::kana_title().value(),
                    fixture::project::group_name().value(),
                    fixture::project::kana_group_name().value(),
                    ProjectCategoryDto::General,
                    0,
                    fixture::user::id1().value(),
                ),
            )
            .await;
        assert!(matches!(res, Ok(())));
    }

    #[tokio::test]
    async fn find_by_id_general_success() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project(
                    ProjectCategory::General,
                    ProjectAttributes::new(0),
                    fixture::user::id1(),
                ))))
            });
        let use_case = ProjectUseCase::new(Arc::new(repositories));

        let actor = fixture::actor::actor1(UserRole::General);
        let res = use_case
            .find_by_id(&actor, fixture::project::id().value().to_string())
            .await;
        assert!(matches!(res, Ok(_)));
    }

    #[tokio::test]
    async fn find_by_id_general_fail() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project(
                    ProjectCategory::General,
                    ProjectAttributes::new(0),
                    fixture::user::id2(),
                ))))
            });
        let use_case = ProjectUseCase::new(Arc::new(repositories));

        let actor = fixture::actor::actor1(UserRole::General);
        let res = use_case
            .find_by_id(&actor, fixture::project::id().value().to_string())
            .await;
        assert!(matches!(
            res,
            Err(ProjectUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn find_by_id_committee_success() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project(
                    ProjectCategory::General,
                    ProjectAttributes::new(0),
                    fixture::user::id2(),
                ))))
            });
        let use_case = ProjectUseCase::new(Arc::new(repositories));

        let actor = fixture::actor::actor1(UserRole::Committee);
        let res = use_case
            .find_by_id(&actor, fixture::project::id().value().to_string())
            .await;
        assert!(matches!(res, Ok(_)));
    }
}
