use std::sync::Arc;

use thiserror::Error;

use sos24_domain::{
    ensure,
    entity::{
        permission::{PermissionDeniedError, Permissions},
        project::{
            ProjectGroupName, ProjectId, ProjectIdError, ProjectKanaGroupName, ProjectKanaTitle,
            ProjectRemarks, ProjectTitle,
        },
        project_application_period::ProjectApplicationPeriod,
    },
    repository::{
        project::{ProjectRepository, ProjectRepositoryError},
        Repositories,
    },
};

use crate::{
    context::{Context, ContextError, OwnedProject},
    dto::{
        project::{CreateProjectDto, ProjectDto, UpdateProjectDto},
        FromEntity, ToEntity,
    },
};

#[derive(Debug, Error)]
pub enum ProjectUseCaseError {
    #[error("Project not found: {0:?}")]
    NotFound(ProjectId),
    #[error("User already owned project: {0:?}")]
    AlreadyOwnedProject(ProjectId),
    #[error("Project applications are not being accepted")]
    ApplicationsNotAccepted,

    #[error(transparent)]
    ContextError(#[from] ContextError),
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
    project_application_period: ProjectApplicationPeriod, // TODO
}

impl<R: Repositories> ProjectUseCase<R> {
    pub fn new(repositories: Arc<R>, project_application_period: ProjectApplicationPeriod) -> Self {
        Self {
            repositories,
            project_application_period,
        }
    }

    pub async fn list(&self, ctx: &Context) -> Result<Vec<ProjectDto>, ProjectUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_PROJECT_ALL));

        let raw_project_list = self.repositories.project_repository().list().await?;
        let project_list = raw_project_list.into_iter().map(ProjectDto::from_entity);
        Ok(project_list.collect())
    }

    pub async fn create(
        &self,
        ctx: &Context,
        raw_project: CreateProjectDto,
    ) -> Result<(), ProjectUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::CREATE_PROJECT));

        if !self.project_application_period.contains(ctx.requested_at()) {
            return Err(ProjectUseCaseError::ApplicationsNotAccepted);
        }

        if let Some(project) = ctx.project(Arc::clone(&self.repositories)).await? {
            let project_id = match project {
                OwnedProject::Owner(project) => project.value.id().clone(),
                OwnedProject::SubOwner(project) => project.value.id().clone(),
            };
            return Err(ProjectUseCaseError::AlreadyOwnedProject(project_id));
        }

        let project = raw_project.into_entity()?;
        self.repositories
            .project_repository()
            .create(project)
            .await?;
        Ok(())
    }

    pub async fn find_by_id(
        &self,
        ctx: &Context,
        id: String,
    ) -> Result<ProjectDto, ProjectUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;

        let id = ProjectId::try_from(id)?;
        let raw_project = self
            .repositories
            .project_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(ProjectUseCaseError::NotFound(id))?;

        ensure!(raw_project.value.is_visible_to(&actor));

        let mut project = ProjectDto::from_entity(raw_project);
        if !actor.has_permission(Permissions::READ_PROJECT_ALL) {
            project.remarks = None;
        }

        Ok(project)
    }

    pub async fn find_owned(
        &self,
        ctx: &Context,
    ) -> Result<Option<ProjectDto>, ProjectUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        let project = ctx.project(Arc::clone(&self.repositories)).await?;

        let project = match project {
            Some(OwnedProject::Owner(project)) => project,
            Some(OwnedProject::SubOwner(project)) => project,
            None => return Ok(None),
        };

        ensure!(project.value.is_visible_to(&actor));

        let mut project = ProjectDto::from_entity(project);
        if !actor.has_permission(Permissions::READ_PROJECT_ALL) {
            project.remarks = None;
        }

        Ok(Some(project))
    }

    pub async fn update(
        &self,
        ctx: &Context,
        project_data: UpdateProjectDto,
    ) -> Result<(), ProjectUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;

        let id = ProjectId::try_from(project_data.id)?;
        let project = self
            .repositories
            .project_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(ProjectUseCaseError::NotFound(id))?;

        ensure!(project.value.is_visible_to(&actor));
        ensure!(project.value.is_updatable_by(&actor));

        if !actor.has_permission(Permissions::UPDATE_PROJECT_ALL)
            && !self.project_application_period.contains(ctx.requested_at())
        {
            return Err(ProjectUseCaseError::ApplicationsNotAccepted);
        }

        let mut new_project = project.value;
        new_project.set_title(&actor, ProjectTitle::new(project_data.title))?;
        new_project.set_kana_title(&actor, ProjectKanaTitle::new(project_data.kana_title))?;
        new_project.set_group_name(&actor, ProjectGroupName::new(project_data.group_name))?;
        new_project.set_kana_group_name(
            &actor,
            ProjectKanaGroupName::new(project_data.kana_group_name),
        )?;
        new_project.set_category(&actor, project_data.category.into_entity()?)?;
        new_project.set_attributes(&actor, project_data.attributes.into_entity()?)?;
        if let Some(remarks) = project_data.remarks {
            new_project.set_remarks(&actor, ProjectRemarks::new(remarks))?;
        }

        self.repositories
            .project_repository()
            .update(new_project)
            .await?;
        Ok(())
    }

    pub async fn delete_by_id(&self, ctx: &Context, id: String) -> Result<(), ProjectUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
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
            project_application_period::ProjectApplicationPeriod,
            user::UserRole,
        },
        repository::Repositories,
        test::{fixture, repository::MockRepositories},
    };

    use crate::{
        context::Context,
        dto::project::{CreateProjectDto, ProjectCategoryDto, UpdateProjectDto},
        interactor::project::{ProjectUseCase, ProjectUseCaseError},
    };

    fn new_project_use_case<R: Repositories>(repositories: R) -> ProjectUseCase<R> {
        let application_period = ProjectApplicationPeriod::new(
            chrono::Utc::now()
                .checked_sub_days(chrono::Days::new(1))
                .unwrap()
                .to_rfc3339(),
            chrono::Utc::now()
                .checked_add_days(chrono::Days::new(1))
                .unwrap()
                .to_rfc3339(),
        );
        ProjectUseCase::new(Arc::new(repositories), application_period)
    }

    #[tokio::test]
    async fn list_general_fail() {
        let repositories = MockRepositories::default();
        let use_case = new_project_use_case(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case.list(&ctx).await;
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
        let use_case = new_project_use_case(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case.list(&ctx).await;
        assert!(matches!(res, Ok(list) if list.is_empty()));
    }

    #[tokio::test]
    async fn create_success() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        repositories
            .project_repository_mut()
            .expect_find_by_owner_id()
            .returning(|_| Ok(None));
        repositories
            .project_repository_mut()
            .expect_find_by_sub_owner_id()
            .returning(|_| Ok(None));
        let use_case = new_project_use_case(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .create(
                &ctx,
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
    async fn create_fail() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        repositories
            .project_repository_mut()
            .expect_find_by_owner_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project1(
                    ProjectCategory::General,
                    ProjectAttributes::new(0),
                    fixture::user::id1(),
                ))))
            });
        repositories
            .project_repository_mut()
            .expect_find_by_sub_owner_id()
            .returning(|_| Ok(None));
        let use_case = new_project_use_case(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .create(
                &ctx,
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
        assert!(matches!(
            res,
            Err(ProjectUseCaseError::AlreadyOwnedProject(_))
        ));
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
        let use_case = new_project_use_case(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_id(&ctx, fixture::project::id1().value().to_string())
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
        let use_case = new_project_use_case(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_id(&ctx, fixture::project::id1().value().to_string())
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
        let use_case = new_project_use_case(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .find_by_id(&ctx, fixture::project::id1().value().to_string())
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
        let use_case = new_project_use_case(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .delete_by_id(&ctx, fixture::project::id1().value().to_string())
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
        let use_case = new_project_use_case(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .delete_by_id(&ctx, fixture::project::id1().value().to_string())
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
        let use_case = new_project_use_case(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .update(
                &ctx,
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
        let use_case = new_project_use_case(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .update(
                &ctx,
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
        let use_case = new_project_use_case(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .update(
                &ctx,
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
