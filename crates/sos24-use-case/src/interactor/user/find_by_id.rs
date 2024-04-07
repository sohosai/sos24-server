use std::sync::Arc;

use sos24_domain::entity::permission::PermissionDeniedError;
use sos24_domain::entity::user::UserId;
use sos24_domain::repository::{user::UserRepository, Repositories};

use crate::context::{Context, OwnedProject};
use crate::dto::user::UserDto;
use crate::dto::FromEntity;
use crate::interactor::user::{UserUseCase, UserUseCaseError};

impl<R: Repositories> UserUseCase<R> {
    pub async fn find_by_id(&self, ctx: &Context, id: String) -> Result<UserDto, UserUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;

        let id = UserId::new(id);
        let raw_user = self
            .repositories
            .user_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(UserUseCaseError::NotFound(id.clone()))?;

        let raw_project = match ctx.project(Arc::clone(&self.repositories)).await? {
            Some(OwnedProject::Owner(project)) => Some(project),
            Some(OwnedProject::SubOwner(project)) => Some(project),
            None => None,
        };

        if raw_user.value.is_visible_to(&actor) {
            Ok(UserDto::from_entity((raw_user, raw_project)))
        } else {
            Err(UserUseCaseError::PermissionDeniedError(
                PermissionDeniedError,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::entity::permission::PermissionDeniedError;
    use sos24_domain::entity::user::UserRole;
    use sos24_domain::test::{fixture, repository::MockRepositories};

    use crate::context::Context;
    use crate::interactor::user::{UserUseCase, UserUseCaseError};

    #[tokio::test]
    async fn 一般ユーザーは自分のユーザーを取得できる() {
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
            .project_repository_mut()
            .expect_find_by_owner_id()
            .returning(|_| Ok(None));
        repositories
            .project_repository_mut()
            .expect_find_by_sub_owner_id()
            .returning(|_| Ok(None));
        let use_case = UserUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_id(&ctx, fixture::user::id1().value())
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn 一般ユーザーは他人のユーザーを取得できない() {
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
            .project_repository_mut()
            .expect_find_by_owner_id()
            .returning(|_| Ok(None));
        repositories
            .project_repository_mut()
            .expect_find_by_sub_owner_id()
            .returning(|_| Ok(None));
        let use_case = UserUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
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
    async fn 実委人は他人のユーザーを取得できる() {
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
            .project_repository_mut()
            .expect_find_by_owner_id()
            .returning(|_| Ok(None));
        repositories
            .project_repository_mut()
            .expect_find_by_sub_owner_id()
            .returning(|_| Ok(None));
        let use_case = UserUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .find_by_id(&ctx, fixture::user::id2().value())
            .await;
        assert!(res.is_ok());
    }
}
