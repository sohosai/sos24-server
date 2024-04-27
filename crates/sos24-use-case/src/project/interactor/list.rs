use sos24_domain::ensure;
use sos24_domain::entity::permission::Permissions;
use sos24_domain::repository::project::ProjectRepository;
use sos24_domain::repository::user::UserRepository;
use sos24_domain::repository::Repositories;

use crate::project::dto::ProjectDto;
use crate::project::{ProjectUseCase, ProjectUseCaseError};
use crate::shared::context::ContextProvider;

impl<R: Repositories> ProjectUseCase<R> {
    pub async fn list(
        &self,
        ctx: &impl ContextProvider,
    ) -> Result<Vec<ProjectDto>, ProjectUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
        ensure!(actor.has_permission(Permissions::READ_PROJECT_ALL));

        let raw_project_list = self.repositories.project_repository().list().await?;

        let mut project_list = Vec::new();
        for raw_project in raw_project_list {
            let owner_id = raw_project.value.owner_id();
            let raw_owner = self
                .repositories
                .user_repository()
                .find_by_id(owner_id.clone())
                .await?
                .ok_or(ProjectUseCaseError::UserNotFound(owner_id.clone()))?;

            let sub_owner_id = raw_project.value.sub_owner_id();
            let raw_sub_owner = match sub_owner_id {
                Some(sub_owner_id) => Some(
                    self.repositories
                        .user_repository()
                        .find_by_id(sub_owner_id.clone())
                        .await?
                        .ok_or(ProjectUseCaseError::UserNotFound(sub_owner_id.clone()))?,
                ),
                None => None,
            };

            project_list.push(ProjectDto::from((raw_project, raw_owner, raw_sub_owner)));
        }
        Ok(project_list)
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
    use crate::shared::context::TestContext;

    #[tokio::test]
    async fn 一般ユーザーは企画一覧を取得できない() {
        let repositories = MockRepositories::default();
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
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
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::Committee));
        let res = use_case.list(&ctx).await;
        assert!(matches!(res, Ok(list) if list.is_empty()));
    }
}
