use std::sync::Arc;

use sos24_domain::ensure;
use sos24_domain::entity::permission::Permissions;
use sos24_domain::entity::project::{
    Project, ProjectAttributes, ProjectCategory, ProjectGroupName, ProjectKanaGroupName,
    ProjectKanaTitle, ProjectTitle,
};
use sos24_domain::entity::user::UserId;
use sos24_domain::repository::project::ProjectRepository;
use sos24_domain::repository::Repositories;

use crate::context::{Context, OwnedProject};
use crate::project::dto::{ProjectAttributesDto, ProjectCategoryDto};
use crate::project::use_case::ProjectUseCaseError;

use super::ProjectUseCase;

pub struct CreateProjectCommand {
    pub title: String,
    pub kana_title: String,
    pub group_name: String,
    pub kana_group_name: String,
    pub category: ProjectCategoryDto,
    pub attributes: ProjectAttributesDto,
}

impl<R: Repositories> ProjectUseCase<R> {
    pub async fn create(
        &self,
        ctx: &Context,
        project_data: CreateProjectCommand,
    ) -> Result<String, ProjectUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::CREATE_PROJECT));

        if !self
            .project_application_period
            .can_create_project(&actor, ctx.requested_at())
        {
            return Err(ProjectUseCaseError::ApplicationsNotAccepted);
        }

        if let Some(project) = ctx.project(Arc::clone(&self.repositories)).await? {
            let project_id = match project {
                OwnedProject::Owner(project) => project.value.id().clone(),
                OwnedProject::SubOwner(project) => project.value.id().clone(),
            };
            return Err(ProjectUseCaseError::AlreadyOwnedProject(project_id));
        }

        let owner_id = actor.user_id().clone().value();
        let project = Project::create(
            ProjectTitle::try_from(project_data.title)
                .map_err(ProjectUseCaseError::ProjectTitleError)?,
            ProjectKanaTitle::new(project_data.kana_title),
            ProjectGroupName::try_from(project_data.group_name)
                .map_err(ProjectUseCaseError::ProjectGroupNameError)?,
            ProjectKanaGroupName::new(project_data.kana_group_name),
            ProjectCategory::from(project_data.category),
            ProjectAttributes::from(project_data.attributes),
            UserId::new(owner_id),
        );
        let project_id = project.id().clone();
        self.repositories
            .project_repository()
            .create(project)
            .await?;

        Ok(project_id.value().to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::entity::user::UserRole;
    use sos24_domain::test::fixture;
    use sos24_domain::test::repository::MockRepositories;

    use crate::context::Context;
    use crate::project::dto::{ProjectAttributesDto, ProjectCategoryDto};
    use crate::project::use_case::create::CreateProjectCommand;
    use crate::project::use_case::{ProjectUseCase, ProjectUseCaseError};

    #[tokio::test]
    async fn 一般ユーザーは企画を作成できる() {
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
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .create(
                &ctx,
                CreateProjectCommand {
                    title: fixture::project::title1().value(),
                    kana_title: fixture::project::kana_title1().value(),
                    group_name: fixture::project::group_name1().value(),
                    kana_group_name: fixture::project::kana_group_name1().value(),
                    category: ProjectCategoryDto::from(fixture::project::category1()),
                    attributes: ProjectAttributesDto::from(fixture::project::attributes1()),
                },
            )
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn 企画責任者の一般ユーザーは企画を作成できない() {
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
                    fixture::user::id1(),
                ))))
            });
        repositories
            .project_repository_mut()
            .expect_find_by_sub_owner_id()
            .returning(|_| Ok(None));
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .create(
                &ctx,
                CreateProjectCommand {
                    title: fixture::project::title1().value(),
                    kana_title: fixture::project::kana_title1().value(),
                    group_name: fixture::project::group_name1().value(),
                    kana_group_name: fixture::project::kana_group_name1().value(),
                    category: ProjectCategoryDto::from(fixture::project::category1()),
                    attributes: ProjectAttributesDto::from(fixture::project::attributes1()),
                },
            )
            .await;
        assert!(matches!(
            res,
            Err(ProjectUseCaseError::AlreadyOwnedProject(_))
        ));
    }

    #[tokio::test]
    async fn 実委人は応募期間外に企画を作成できない() {
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
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
            fixture::project_application_period::not_applicable_period(),
        );

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .create(
                &ctx,
                CreateProjectCommand {
                    title: fixture::project::title1().value(),
                    kana_title: fixture::project::kana_title1().value(),
                    group_name: fixture::project::group_name1().value(),
                    kana_group_name: fixture::project::kana_group_name1().value(),
                    category: ProjectCategoryDto::from(fixture::project::category1()),
                    attributes: ProjectAttributesDto::from(fixture::project::attributes1()),
                },
            )
            .await;
        assert!(matches!(
            res,
            Err(ProjectUseCaseError::ApplicationsNotAccepted)
        ));
    }

    #[tokio::test]
    async fn 実委人管理者は応募期間外に企画を作成できる() {
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
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
            fixture::project_application_period::not_applicable_period(),
        );

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .create(
                &ctx,
                CreateProjectCommand {
                    title: fixture::project::title1().value(),
                    kana_title: fixture::project::kana_title1().value(),
                    group_name: fixture::project::group_name1().value(),
                    kana_group_name: fixture::project::kana_group_name1().value(),
                    category: ProjectCategoryDto::from(fixture::project::category1()),
                    attributes: ProjectAttributesDto::from(fixture::project::attributes1()),
                },
            )
            .await;
        assert!(res.is_ok());
    }

    // TODO: 副企画責任者の一般ユーザーは企画を作成できない
}
