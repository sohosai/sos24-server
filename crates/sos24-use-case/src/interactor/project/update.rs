use std::sync::Arc;

use sos24_domain::ensure;
use sos24_domain::entity::permission::Permissions;
use sos24_domain::entity::project::{
    ProjectGroupName, ProjectId, ProjectKanaGroupName, ProjectKanaTitle, ProjectRemarks,
    ProjectTitle,
};
use sos24_domain::repository::project::ProjectRepository;
use sos24_domain::repository::Repositories;

use crate::context::Context;
use crate::dto::project::UpdateProjectDto;
use crate::dto::ToEntity;
use crate::interactor::project::{ProjectUseCase, ProjectUseCaseError};

impl<R: Repositories> ProjectUseCase<R> {
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
}

#[cfg(test)]
mod tests {
    use sos24_domain::entity::permission::PermissionDeniedError;
    use sos24_domain::entity::user::UserRole;
    use sos24_domain::test::fixture;
    use sos24_domain::test::repository::MockRepositories;

    use crate::context::Context;
    use crate::dto::project::{ProjectCategoryDto, UpdateProjectDto};
    use crate::dto::FromEntity;
    use crate::interactor::project::{ProjectUseCase, ProjectUseCaseError};

    #[tokio::test]
    async fn update_committee_success() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project1(
                    fixture::user::id1(),
                ))))
            });
        repositories
            .project_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let use_case = ProjectUseCase::new_for_test(repositories);

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
                    ProjectCategoryDto::from_entity(fixture::project::category2()),
                    Vec::from_entity(fixture::project::attributes2()),
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
                    fixture::user::id2(),
                ))))
            });
        repositories
            .project_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let use_case = ProjectUseCase::new_for_test(repositories);

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
                    ProjectCategoryDto::from_entity(fixture::project::category2()),
                    Vec::from_entity(fixture::project::attributes2()),
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
                    fixture::user::id2(),
                ))))
            });
        repositories
            .project_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let use_case = ProjectUseCase::new_for_test(repositories);

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
                    ProjectCategoryDto::from_entity(fixture::project::category2()),
                    Vec::from_entity(fixture::project::attributes2()),
                    None,
                ),
            )
            .await;
        assert!(matches!(res, Ok(())));
    }
}