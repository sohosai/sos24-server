use sos24_domain::ensure;
use sos24_domain::entity::permission::Permissions;
use sos24_domain::entity::project::{
    ProjectAttributes, ProjectCategory, ProjectGroupName, ProjectId, ProjectKanaGroupName,
    ProjectKanaTitle, ProjectRemarks, ProjectTitle,
};
use sos24_domain::repository::project::ProjectRepository;
use sos24_domain::repository::Repositories;

use crate::project::dto::{ProjectAttributesDto, ProjectCategoryDto};
use crate::project::{ProjectUseCase, ProjectUseCaseError};
use crate::shared::adapter::Adapters;
use crate::shared::context::ContextProvider;

#[derive(Debug)]
pub struct UpdateProjectCommand {
    pub id: String,
    pub title: String,
    pub kana_title: String,
    pub group_name: String,
    pub kana_group_name: String,
    pub category: ProjectCategoryDto,
    pub attributes: ProjectAttributesDto,
    pub remarks: Option<String>,
}

impl<R: Repositories, A: Adapters> ProjectUseCase<R, A> {
    pub async fn update(
        &self,
        ctx: &impl ContextProvider,
        project_data: UpdateProjectCommand,
    ) -> Result<(), ProjectUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;

        let id = ProjectId::try_from(project_data.id)?;
        let project_with_owners = self
            .repositories
            .project_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(ProjectUseCaseError::NotFound(id))?;

        ensure!(project_with_owners.project.is_visible_to(&actor));
        ensure!(project_with_owners.project.is_updatable_by(&actor));

        if !actor.has_permission(Permissions::UPDATE_PROJECT_ALL)
            && !self
                .project_application_period
                .can_create_project(&actor, ctx.requested_at())
        {
            return Err(ProjectUseCaseError::ApplicationsNotAccepted);
        }

        let mut new_project = project_with_owners.project;
        new_project.set_title(
            &actor,
            ProjectTitle::try_from(project_data.title)
                .map_err(ProjectUseCaseError::ProjectTitleError)?,
        )?;
        new_project.set_kana_title(&actor, ProjectKanaTitle::new(project_data.kana_title))?;
        new_project.set_group_name(
            &actor,
            ProjectGroupName::try_from(project_data.group_name)
                .map_err(ProjectUseCaseError::ProjectGroupNameError)?,
        )?;
        new_project.set_kana_group_name(
            &actor,
            ProjectKanaGroupName::new(project_data.kana_group_name),
        )?;
        new_project.set_category(&actor, ProjectCategory::from(project_data.category))?;
        new_project.set_attributes(&actor, ProjectAttributes::from(project_data.attributes))?;
        if let Some(remarks) = project_data.remarks {
            new_project.set_remarks(&actor, ProjectRemarks::new(remarks))?;
        }

        self.repositories
            .project_repository()
            .update(new_project)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::entity::permission::PermissionDeniedError;
    use sos24_domain::entity::user::UserRole;
    use sos24_domain::test::fixture;
    use sos24_domain::test::repository::MockRepositories;

    use crate::project::dto::{ProjectAttributesDto, ProjectCategoryDto};
    use crate::project::interactor::update::UpdateProjectCommand;
    use crate::project::{ProjectUseCase, ProjectUseCaseError};
    use crate::shared::adapter::MockAdapters;
    use crate::shared::context::TestContext;

    #[tokio::test]
    async fn 実委人は企画募集期間内に自分の企画を更新できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user1(UserRole::CommitteeViewer),
                )))
            });
        repositories
            .project_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let adapters = MockAdapters::default();
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
            Arc::new(adapters),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeViewer));
        let res = use_case
            .update(
                &ctx,
                UpdateProjectCommand {
                    id: fixture::project::id2().value().to_string(),
                    title: fixture::project::title2().value(),
                    kana_title: fixture::project::kana_title2().value(),
                    group_name: fixture::project::group_name2().value(),
                    kana_group_name: fixture::project::kana_group_name2().value(),
                    category: ProjectCategoryDto::from(fixture::project::category2()),
                    attributes: ProjectAttributesDto::from(fixture::project::attributes2()),
                    remarks: None,
                },
            )
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn 実委人は企画募集期間外に自分の企画を更新できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user1(UserRole::CommitteeViewer),
                )))
            });
        repositories
            .project_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let adapters = MockAdapters::default();
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
            Arc::new(adapters),
            fixture::project_application_period::not_applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeViewer));
        let res = use_case
            .update(
                &ctx,
                UpdateProjectCommand {
                    id: fixture::project::id2().value().to_string(),
                    title: fixture::project::title2().value(),
                    kana_title: fixture::project::kana_title2().value(),
                    group_name: fixture::project::group_name2().value(),
                    kana_group_name: fixture::project::kana_group_name2().value(),
                    category: ProjectCategoryDto::from(fixture::project::category2()),
                    attributes: ProjectAttributesDto::from(fixture::project::attributes2()),
                    remarks: None,
                },
            )
            .await;
        assert!(matches!(
            res,
            Err(ProjectUseCaseError::ApplicationsNotAccepted)
        ));
    }

    #[tokio::test]
    async fn 実委人は他人の企画を更新できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user2(UserRole::General),
                )))
            });
        repositories
            .project_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let adapters = MockAdapters::default();
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
            Arc::new(adapters),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeViewer));
        let res = use_case
            .update(
                &ctx,
                UpdateProjectCommand {
                    id: fixture::project::id2().value().to_string(),
                    title: fixture::project::title2().value(),
                    kana_title: fixture::project::kana_title2().value(),
                    group_name: fixture::project::group_name2().value(),
                    kana_group_name: fixture::project::kana_group_name2().value(),
                    category: ProjectCategoryDto::from(fixture::project::category2()),
                    attributes: ProjectAttributesDto::from(fixture::project::attributes2()),
                    remarks: None,
                },
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
    async fn 実委人管理者は他人の企画を更新できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user2(UserRole::General),
                )))
            });
        repositories
            .project_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let adapters = MockAdapters::default();
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
            Arc::new(adapters),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .update(
                &ctx,
                UpdateProjectCommand {
                    id: fixture::project::id2().value().to_string(),
                    title: fixture::project::title2().value(),
                    kana_title: fixture::project::kana_title2().value(),
                    group_name: fixture::project::group_name2().value(),
                    kana_group_name: fixture::project::kana_group_name2().value(),
                    category: ProjectCategoryDto::from(fixture::project::category2()),
                    attributes: ProjectAttributesDto::from(fixture::project::attributes2()),
                    remarks: None,
                },
            )
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn 実委人管理者は企画募集期間外に他人の企画を更新できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user2(UserRole::General),
                )))
            });
        repositories
            .project_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let adapters = MockAdapters::default();
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
            Arc::new(adapters),
            fixture::project_application_period::not_applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .update(
                &ctx,
                UpdateProjectCommand {
                    id: fixture::project::id2().value().to_string(),
                    title: fixture::project::title2().value(),
                    kana_title: fixture::project::kana_title2().value(),
                    group_name: fixture::project::group_name2().value(),
                    kana_group_name: fixture::project::kana_group_name2().value(),
                    category: ProjectCategoryDto::from(fixture::project::category2()),
                    attributes: ProjectAttributesDto::from(fixture::project::attributes2()),
                    remarks: None,
                },
            )
            .await;
        assert!(res.is_ok());
    }

    // TODO: 実委人は自分の企画の備考を更新できない
}
