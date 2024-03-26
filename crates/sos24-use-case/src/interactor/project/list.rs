use std::sync::Arc;

use sos24_domain::ensure;
use sos24_domain::entity::permission::Permissions;
use sos24_domain::repository::project::ProjectRepository;
use sos24_domain::repository::Repositories;

use crate::context::Context;
use crate::dto::project::ProjectDto;
use crate::dto::FromEntity;
use crate::interactor::project::{ProjectUseCase, ProjectUseCaseError};

impl<R: Repositories> ProjectUseCase<R> {
    pub async fn list(&self, ctx: &Context) -> Result<Vec<ProjectDto>, ProjectUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_PROJECT_ALL));

        let raw_project_list = self.repositories.project_repository().list().await?;
        let project_list = raw_project_list.into_iter().map(ProjectDto::from_entity);
        Ok(project_list.collect())
    }
}

#[cfg(test)]
mod tests {
    use sos24_domain::entity::permission::PermissionDeniedError;
    use sos24_domain::entity::user::UserRole;
    use sos24_domain::test::fixture;
    use sos24_domain::test::repository::MockRepositories;

    use crate::context::Context;
    use crate::interactor::project::{ProjectUseCase, ProjectUseCaseError};

    #[tokio::test]
    async fn list_general_fail() {
        let repositories = MockRepositories::default();
        let use_case = ProjectUseCase::new_for_test(repositories);

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
        let use_case = ProjectUseCase::new_for_test(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case.list(&ctx).await;
        assert!(matches!(res, Ok(list) if list.is_empty()));
    }
}
