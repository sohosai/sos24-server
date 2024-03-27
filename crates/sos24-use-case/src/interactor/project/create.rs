use std::sync::Arc;

use sos24_domain::ensure;
use sos24_domain::entity::permission::Permissions;
use sos24_domain::repository::project::ProjectRepository;
use sos24_domain::repository::Repositories;

use crate::context::{Context, OwnedProject};
use crate::dto::project::CreateProjectDto;
use crate::dto::ToEntity;
use crate::interactor::project::{ProjectUseCase, ProjectUseCaseError};

impl<R: Repositories> ProjectUseCase<R> {
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
}

#[cfg(test)]
mod tests {
    use sos24_domain::entity::user::UserRole;
    use sos24_domain::test::fixture;
    use sos24_domain::test::repository::MockRepositories;

    use crate::context::Context;
    use crate::dto::project::{CreateProjectDto, ProjectCategoryDto};
    use crate::dto::FromEntity;
    use crate::interactor::project::{ProjectUseCase, ProjectUseCaseError};

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
        let use_case = ProjectUseCase::new_for_test(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .create(
                &ctx,
                CreateProjectDto::new(
                    fixture::project::title1().value(),
                    fixture::project::kana_title1().value(),
                    fixture::project::group_name1().value(),
                    fixture::project::kana_group_name1().value(),
                    ProjectCategoryDto::from_entity(fixture::project::category1()),
                    Vec::from_entity(fixture::project::attributes1()),
                    fixture::user::id1().value(),
                ),
            )
            .await;
        assert!(matches!(res, Ok(())));
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
        let use_case = ProjectUseCase::new_for_test(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .create(
                &ctx,
                CreateProjectDto::new(
                    fixture::project::title1().value(),
                    fixture::project::kana_title1().value(),
                    fixture::project::group_name1().value(),
                    fixture::project::kana_group_name1().value(),
                    ProjectCategoryDto::from_entity(fixture::project::category1()),
                    Vec::from_entity(fixture::project::attributes1()),
                    fixture::user::id1().value(),
                ),
            )
            .await;
        assert!(matches!(
            res,
            Err(ProjectUseCaseError::AlreadyOwnedProject(_))
        ));
    }

    // TODO: 一般ユーザーは応募期間外に企画を作成できない
    // TODO: 副企画責任者の一般ユーザーは企画を作成できない
}
