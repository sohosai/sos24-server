use sos24_domain::ensure;
use sos24_domain::entity::permission::Permissions;
use sos24_domain::entity::project::ProjectId;
use sos24_domain::repository::project::ProjectRepository;
use sos24_domain::repository::Repositories;

use crate::project::dto::ProjectDto;
use crate::project::{ProjectUseCase, ProjectUseCaseError};
use crate::shared::adapter::Adapters;
use crate::shared::context::ContextProvider;

impl<R: Repositories, A: Adapters> ProjectUseCase<R, A> {
    pub async fn find_by_id(
        &self,
        ctx: &impl ContextProvider,
        id: String,
    ) -> Result<ProjectDto, ProjectUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;

        let id = ProjectId::try_from(id)?;
        let project_with_owners = self
            .repositories
            .project_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(ProjectUseCaseError::NotFound(id))?;
        ensure!(project_with_owners.project.is_visible_to(&actor));

        let mut project = ProjectDto::from(project_with_owners);
        if !actor.has_permission(Permissions::READ_PROJECT_ALL) {
            project.remarks = None;
        }

        Ok(project)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::entity::permission::PermissionDeniedError;
    use sos24_domain::entity::user::UserRole;
    use sos24_domain::test::fixture;
    use sos24_domain::test::repository::MockRepositories;

    use crate::project::{ProjectUseCase, ProjectUseCaseError};
    use crate::shared::adapter::MockAdapters;
    use crate::shared::context::TestContext;

    #[tokio::test]
    async fn 一般ユーザーは自分の企画を取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user1(UserRole::General),
                )))
            });
        let adapters = MockAdapters::default();
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
            Arc::new(adapters),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_id(&ctx, fixture::project::id1().value().to_string())
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn 一般ユーザーは他人の企画を取得できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user2(UserRole::General),
                )))
            });
        let adapters = MockAdapters::default();
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
            Arc::new(adapters),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
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
    async fn 実委人は他人の企画を取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user2(UserRole::General),
                )))
            });
        let adapters = MockAdapters::default();
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
            Arc::new(adapters),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeViewer));
        let res = use_case
            .find_by_id(&ctx, fixture::project::id1().value().to_string())
            .await;
        assert!(res.is_ok());
    }

    // TODO: 一般ユーザーは備考を取得できない
    // TODO: 実委人は備考を取得できる
}
