use std::sync::Arc;

use sos24_domain::ensure;
use sos24_domain::entity::permission::Permissions;
use sos24_domain::entity::project::ProjectId;
use sos24_domain::repository::project::ProjectRepository;
use sos24_domain::repository::user::UserRepository;
use sos24_domain::repository::Repositories;

use crate::context::ContextProvider;
use crate::dto::project::ProjectDto;
use crate::dto::FromEntity;
use crate::interactor::project::{ProjectUseCase, ProjectUseCaseError};

impl<R: Repositories> ProjectUseCase<R> {
    pub async fn find_by_id(
        &self,
        ctx: &impl ContextProvider,
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

        let mut project = ProjectDto::from_entity((raw_project, raw_owner, raw_sub_owner));
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

    use crate::context::TestContext;
    use crate::interactor::project::{ProjectUseCase, ProjectUseCaseError};

    #[tokio::test]
    async fn 一般ユーザーは自分の企画を取得できる() {
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
            .user_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::user::user1(
                    UserRole::General,
                ))))
            });
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
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
                Ok(Some(fixture::date::with(fixture::project::project1(
                    fixture::user::id2(),
                ))))
            });
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
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
                Ok(Some(fixture::date::with(fixture::project::project1(
                    fixture::user::id2(),
                ))))
            });
        repositories
            .user_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::user::user1(
                    UserRole::Committee,
                ))))
            });
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .find_by_id(&ctx, fixture::project::id1().value().to_string())
            .await;
        assert!(res.is_ok());
    }

    // TODO: 一般ユーザーは備考を取得できない
    // TODO: 実委人は備考を取得できる
}
