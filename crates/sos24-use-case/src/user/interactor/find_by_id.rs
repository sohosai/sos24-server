use sos24_domain::ensure;
use sos24_domain::entity::project::Project;
use sos24_domain::entity::user::UserId;
use sos24_domain::repository::project::ProjectRepository;
use sos24_domain::repository::{user::UserRepository, Repositories};

use crate::shared::context::ContextProvider;
use crate::user::dto::UserDto;
use crate::user::{UserUseCase, UserUseCaseError};

impl<R: Repositories> UserUseCase<R> {
    pub async fn find_by_id(
        &self,
        ctx: &impl ContextProvider,
        id: String,
    ) -> Result<UserDto, UserUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;

        let user_id = UserId::new(id);
        let raw_user = self
            .repositories
            .user_repository()
            .find_by_id(user_id.clone())
            .await?
            .ok_or(UserUseCaseError::NotFound(user_id.clone()))?;
        ensure!(raw_user.is_visible_to(&actor));

        let raw_project = find_owned_project(user_id.clone(), &*self.repositories).await?;
        Ok(UserDto::from((raw_user, raw_project)))
    }
}

async fn find_owned_project(
    user_id: UserId,
    repositories: &impl Repositories,
) -> Result<Option<Project>, UserUseCaseError> {
    if let Some(project_with_owners) = repositories
        .project_repository()
        .find_by_owner_id(user_id.clone())
        .await?
    {
        return Ok(Some(project_with_owners.project));
    }

    if let Some(project_with_owners) = repositories
        .project_repository()
        .find_by_sub_owner_id(user_id.clone())
        .await?
    {
        return Ok(Some(project_with_owners.project));
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::entity::permission::PermissionDeniedError;
    use sos24_domain::entity::user::UserRole;
    use sos24_domain::test::{fixture, repository::MockRepositories};

    use crate::shared::context::TestContext;
    use crate::user::{UserUseCase, UserUseCaseError};

    #[tokio::test]
    async fn 実委人は自分のユーザーを取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .user_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::user::user1(UserRole::CommitteeViewer))));
        repositories
            .project_repository_mut()
            .expect_find_by_owner_id()
            .returning(|_| Ok(None));
        repositories
            .project_repository_mut()
            .expect_find_by_sub_owner_id()
            .returning(|_| Ok(None));
        let use_case = UserUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeViewer));
        let res = use_case
            .find_by_id(&ctx, fixture::user::id1().value())
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn 実委人は他人のユーザーを取得できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .user_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::user::user2(UserRole::General))));
        repositories
            .project_repository_mut()
            .expect_find_by_owner_id()
            .returning(|_| Ok(None));
        repositories
            .project_repository_mut()
            .expect_find_by_sub_owner_id()
            .returning(|_| Ok(None));
        let use_case = UserUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeViewer));
        let res = use_case
            .find_by_id(&ctx, fixture::user::id2().value())
            .await;
        assert!(matches!(
            res,
            Err(UserUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn 実委人管理者は他人のユーザーを取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .user_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::user::user2(UserRole::General))));
        repositories
            .project_repository_mut()
            .expect_find_by_owner_id()
            .returning(|_| Ok(None));
        repositories
            .project_repository_mut()
            .expect_find_by_sub_owner_id()
            .returning(|_| Ok(None));
        let use_case = UserUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .find_by_id(&ctx, fixture::user::id2().value())
            .await;
        assert!(res.is_ok());
    }
}
