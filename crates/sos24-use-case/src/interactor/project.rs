use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::{
        actor::Actor,
        permission::{PermissionDeniedError, Permissions},
        project::{
            ProjectAttributes, ProjectGroupName, ProjectId, ProjectIdError, ProjectKanaGroupName,
            ProjectKanaTitle, ProjectRemarks, ProjectTitle,
        },
    },
    repository::{
        project::{ProjectRepository, ProjectRepositoryError},
        Repositories,
    },
};
use thiserror::Error;

use crate::dto::{
    project::{CreateProjectDto, ProjectDto, UpdateProjectDto},
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

        let mut project = ProjectDto::from_entity(raw_project);
        if !actor.has_permission(Permissions::READ_PROJECT_ALL) {
            project.remarks = None;
        }

        Ok(project)
    }

    pub async fn update(
        &self,
        actor: &Actor,
        project_data: UpdateProjectDto,
    ) -> Result<(), ProjectUseCaseError> {
        let id = ProjectId::try_from(project_data.id)?;
        let project = self
            .repositories
            .project_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(ProjectUseCaseError::NotFound(id))?;

        ensure!(project.value.is_visible_to(actor));
        ensure!(project.value.is_updatable_by(actor));

        // TODO: roleがgeneralの場合、企画募集期間かを確認する

        let mut new_project = project.value;
        new_project.set_title(actor, ProjectTitle::new(project_data.title))?;
        new_project.set_kana_title(actor, ProjectKanaTitle::new(project_data.kana_title))?;
        new_project.set_group_name(actor, ProjectGroupName::new(project_data.group_name))?;
        new_project.set_kana_group_name(
            actor,
            ProjectKanaGroupName::new(project_data.kana_group_name),
        )?;
        new_project.set_category(actor, project_data.category.into_entity()?)?;
        new_project.set_attributes(actor, ProjectAttributes::new(project_data.attributes))?;
        if let Some(remarks) = project_data.remarks {
            new_project.set_remarks(actor, ProjectRemarks::new(remarks))?;
        }

        self.repositories
            .project_repository()
            .update(new_project)
            .await?;
        Ok(())
    }

    pub async fn delete_by_id(&self, actor: &Actor, id: String) -> Result<(), ProjectUseCaseError> {
        ensure!(actor.has_permission(Permissions::DELETE_PROJECT_ALL));

        let id = ProjectId::try_from(id)?;
        self.repositories
            .project_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(ProjectUseCaseError::NotFound(id.clone()))?;

        self.repositories
            .project_repository()
            .delete_by_id(id)
            .await?;
        Ok(())
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
        dto::project::{CreateProjectDto, ProjectCategoryDto, UpdateProjectDto},
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
                    fixture::project::title1().value(),
                    fixture::project::kana_title1().value(),
                    fixture::project::group_name1().value(),
                    fixture::project::kana_group_name1().value(),
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
                Ok(Some(fixture::date::with(fixture::project::project1(
                    ProjectCategory::General,
                    ProjectAttributes::new(0),
                    fixture::user::id1(),
                ))))
            });
        let use_case = ProjectUseCase::new(Arc::new(repositories));

        let actor = fixture::actor::actor1(UserRole::General);
        let res = use_case
            .find_by_id(&actor, fixture::project::id1().value().to_string())
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
                Ok(Some(fixture::date::with(fixture::project::project1(
                    ProjectCategory::General,
                    ProjectAttributes::new(0),
                    fixture::user::id2(),
                ))))
            });
        let use_case = ProjectUseCase::new(Arc::new(repositories));

        let actor = fixture::actor::actor1(UserRole::General);
        let res = use_case
            .find_by_id(&actor, fixture::project::id1().value().to_string())
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
                Ok(Some(fixture::date::with(fixture::project::project1(
                    ProjectCategory::General,
                    ProjectAttributes::new(0),
                    fixture::user::id2(),
                ))))
            });
        let use_case = ProjectUseCase::new(Arc::new(repositories));

        let actor = fixture::actor::actor1(UserRole::Committee);
        let res = use_case
            .find_by_id(&actor, fixture::project::id1().value().to_string())
            .await;
        assert!(matches!(res, Ok(_)));
    }

    #[tokio::test]
    async fn delete_by_id_committee_fail() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project1(
                    ProjectCategory::General,
                    ProjectAttributes::new(0),
                    fixture::user::id1(),
                ))))
            });
        let use_case = ProjectUseCase::new(Arc::new(repositories));

        let actor = fixture::actor::actor1(UserRole::Committee);
        let res = use_case
            .delete_by_id(&actor, fixture::project::id1().value().to_string())
            .await;
        assert!(matches!(
            res,
            Err(ProjectUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn delete_by_id_operator_success() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project1(
                    ProjectCategory::General,
                    ProjectAttributes::new(0),
                    fixture::user::id2(),
                ))))
            });
        repositories
            .project_repository_mut()
            .expect_delete_by_id()
            .returning(|_| Ok(()));
        let use_case = ProjectUseCase::new(Arc::new(repositories));

        let actor = fixture::actor::actor1(UserRole::CommitteeOperator);
        let res = use_case
            .delete_by_id(&actor, fixture::project::id1().value().to_string())
            .await;
        assert!(matches!(res, Ok(())));
    }

    #[tokio::test]
    async fn update_committee_success() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project1(
                    ProjectCategory::General,
                    ProjectAttributes::new(0),
                    fixture::user::id1(),
                ))))
            });
        repositories
            .project_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let use_case = ProjectUseCase::new(Arc::new(repositories));

        let actor = fixture::actor::actor1(UserRole::Committee);
        let res = use_case
            .update(
                &actor,
                UpdateProjectDto::new(
                    fixture::project::id2().value().to_string(),
                    fixture::project::title2().value(),
                    fixture::project::kana_title2().value(),
                    fixture::project::group_name2().value(),
                    fixture::project::kana_group_name2().value(),
                    ProjectCategoryDto::Stage1A,
                    1,
                    None,
                ),
            )
            .await;
        assert!(matches!(res, Ok(())));
    }

    #[tokio::test]
    async fn update_committee_fail() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project1(
                    ProjectCategory::General,
                    ProjectAttributes::new(0),
                    fixture::user::id2(),
                ))))
            });
        repositories
            .project_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let use_case = ProjectUseCase::new(Arc::new(repositories));

        let actor = fixture::actor::actor1(UserRole::Committee);
        let res = use_case
            .update(
                &actor,
                UpdateProjectDto::new(
                    fixture::project::id2().value().to_string(),
                    fixture::project::title2().value(),
                    fixture::project::kana_title2().value(),
                    fixture::project::group_name2().value(),
                    fixture::project::kana_group_name2().value(),
                    ProjectCategoryDto::Stage1A,
                    1,
                    None,
                ),
            )
            .await;
        assert!(matches!(
            res,
            Err(ProjectUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn update_operator_success() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project1(
                    ProjectCategory::General,
                    ProjectAttributes::new(0),
                    fixture::user::id2(),
                ))))
            });
        repositories
            .project_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let use_case = ProjectUseCase::new(Arc::new(repositories));

        let actor = fixture::actor::actor1(UserRole::CommitteeOperator);
        let res = use_case
            .update(
                &actor,
                UpdateProjectDto::new(
                    fixture::project::id2().value().to_string(),
                    fixture::project::title2().value(),
                    fixture::project::kana_title2().value(),
                    fixture::project::group_name2().value(),
                    fixture::project::kana_group_name2().value(),
                    ProjectCategoryDto::Stage1A,
                    1,
                    None,
                ),
            )
            .await;
        assert!(matches!(res, Ok(())));
    }
}
