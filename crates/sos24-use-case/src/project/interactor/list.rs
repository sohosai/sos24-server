use sos24_domain::ensure;
use sos24_domain::entity::permission::Permissions;
use sos24_domain::repository::project::ProjectRepository;
use sos24_domain::repository::Repositories;

use crate::project::dto::ProjectDto;
use crate::project::{ProjectUseCase, ProjectUseCaseError};
use crate::shared::adapter::Adapters;
use crate::shared::context::ContextProvider;

impl<R: Repositories, A: Adapters> ProjectUseCase<R, A> {
    pub async fn list(
        &self,
        ctx: &impl ContextProvider,
    ) -> Result<Vec<ProjectDto>, ProjectUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
        ensure!(actor.has_permission(Permissions::READ_PROJECT_ALL));

        let project_list = self.repositories.project_repository().list().await?;
        Ok(project_list.into_iter().map(ProjectDto::from).collect())
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
    async fn 一般ユーザーは企画一覧を取得できない() {
        let repositories = MockRepositories::default();
        let adapters = MockAdapters::default();
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
            Arc::new(adapters),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case.list(&ctx).await;
        assert!(matches!(
            res,
            Err(ProjectUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn 実委人は企画一覧を取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_list()
            .returning(|| Ok(vec![]));
        let adapters = MockAdapters::default();
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
            Arc::new(adapters),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeViewer));
        let res = use_case.list(&ctx).await;
        assert!(matches!(res, Ok(list) if list.is_empty()));
    }
}
