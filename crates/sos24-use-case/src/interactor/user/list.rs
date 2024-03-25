use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::permission::Permissions,
    repository::{Repositories, user::UserRepository},
};

use crate::{
    context::Context,
    dto::{FromEntity, user::UserDto},
    interactor::user::{UserUseCase, UserUseCaseError},
};

impl<R: Repositories> UserUseCase<R> {
    pub async fn list(&self, ctx: &Context) -> Result<Vec<UserDto>, UserUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_USER_ALL));

        let raw_user_list = self.repositories.user_repository().list().await?;
        let news_list = raw_user_list.into_iter().map(UserDto::from_entity);
        Ok(news_list.collect())
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
        context::Context,
        interactor::user::{UserUseCase, UserUseCaseError},
    };

    #[tokio::test]
    async fn 一般ユーザーはユーザー一覧を取得できない() {
        let repositories = MockRepositories::default();
        let use_case = UserUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case.list(&ctx).await;
        assert!(matches!(
            res,
            Err(UserUseCaseError::PermissionDenied(PermissionDeniedError))
        ));
    }

    #[tokio::test]
    async fn 実委人はユーザー一覧を取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .user_repository_mut()
            .expect_list()
            .returning(|| Ok(vec![]));
        let use_case = UserUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case.list(&ctx).await;
        assert!(matches!(res, Ok(list) if list.is_empty()));
    }
}
