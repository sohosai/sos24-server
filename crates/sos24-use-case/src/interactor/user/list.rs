use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::permission::Permissions,
    repository::{user::UserRepository, Repositories},
};

use crate::{
    context::ContextProvider,
    dto::{user::UserDto, FromEntity},
    interactor::user::{UserUseCase, UserUseCaseError},
};

impl<R: Repositories> UserUseCase<R> {
    pub async fn list(&self, ctx: &impl ContextProvider) -> Result<Vec<UserDto>, UserUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_USER_ALL));

        let raw_user_list = self.repositories.user_repository().list().await?;
        let user_list = raw_user_list
            .into_iter()
            .map(|raw_user| UserDto::from_entity((raw_user, None)));
        Ok(user_list.collect())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::{
        entity::{permission::PermissionDeniedError, user::UserRole},
        test::{fixture, repository::MockRepositories},
    };

    use crate::{
        context::TestContext,
        interactor::user::{UserUseCase, UserUseCaseError},
    };

    #[tokio::test]
    async fn 実委人はユーザー一覧を取得できない() {
        let repositories = MockRepositories::default();
        let use_case = UserUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::Committee));
        let res = use_case.list(&ctx).await;
        assert!(matches!(
            res,
            Err(UserUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn 実委人管理者はユーザー一覧を取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .user_repository_mut()
            .expect_list()
            .returning(|| Ok(vec![]));
        let use_case = UserUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case.list(&ctx).await;
        assert!(matches!(res, Ok(list) if list.is_empty()));
    }
}
