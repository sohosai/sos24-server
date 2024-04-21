use sos24_domain::entity::permission::Permissions;
use sos24_domain::entity::user::UserId;
use sos24_domain::{
    ensure,
    repository::{user::UserRepository, Repositories},
};

use crate::context::ContextProvider;
use crate::interactor::user::{UserUseCase, UserUseCaseError};

impl<R: Repositories> UserUseCase<R> {
    pub async fn delete_by_id(
        &self,
        ctx: &impl ContextProvider,
        id: String,
    ) -> Result<(), UserUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
        ensure!(actor.has_permission(Permissions::DELETE_USER_ALL));

        let id = UserId::new(id);
        self.repositories
            .user_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(UserUseCaseError::NotFound(id.clone()))?;

        self.repositories.user_repository().delete_by_id(id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::entity::permission::PermissionDeniedError;
    use sos24_domain::entity::user::UserRole;
    use sos24_domain::test::{fixture, repository::MockRepositories};

    use crate::context::TestContext;
    use crate::interactor::user::{UserUseCase, UserUseCaseError};

    #[tokio::test]
    async fn 実委人は自分のユーザーを削除できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .user_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::user::user1(
                    UserRole::General,
                ))))
            });
        repositories
            .user_repository_mut()
            .expect_delete_by_id()
            .returning(|_| Ok(()));
        let use_case = UserUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .delete_by_id(&ctx, fixture::user::id1().value())
            .await;
        assert!(matches!(
            res,
            Err(UserUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn 実委人管理者は他人のユーザーを削除できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .user_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::user::user2(
                    UserRole::General,
                ))))
            });
        repositories
            .user_repository_mut()
            .expect_delete_by_id()
            .returning(|_| Ok(()));
        let use_case = UserUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .delete_by_id(&ctx, fixture::user::id2().value())
            .await;
        assert!(matches!(res, Ok(())));
    }
}
