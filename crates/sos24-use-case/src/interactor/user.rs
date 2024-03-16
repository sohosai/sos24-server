use std::sync::Arc;

use sos24_domain::ensure;
use sos24_domain::entity::common::email::EmailError;
use sos24_domain::entity::firebase_user::{
    FirebaseUserEmail, FirebaseUserPassword, NewFirebaseUser,
};
use sos24_domain::entity::permission::{PermissionDeniedError, Permissions};
use sos24_domain::entity::user::{UserEmail, UserId, UserKanaName, UserName, UserPhoneNumber};
use sos24_domain::repository::firebase_user::{
    FirebaseUserRepository, FirebaseUserRepositoryError,
};
use sos24_domain::repository::user::UserRepositoryError;
use sos24_domain::repository::{user::UserRepository, Repositories};
use thiserror::Error;

use crate::context::{Context, ContextError};
use crate::dto::user::{UpdateUserDto, UserDto};
use crate::dto::FromEntity;
use crate::dto::{user::CreateUserDto, ToEntity};

#[derive(Debug, Error)]
pub enum UserUseCaseError {
    #[error("User not found: {0:?}")]
    NotFound(UserId),

    #[error(transparent)]
    ContextError(#[from] ContextError),
    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
    #[error(transparent)]
    FirebaseUserRepositoryError(#[from] FirebaseUserRepositoryError),
    #[error(transparent)]
    EmailError(#[from] EmailError),
    #[error(transparent)]
    PermissionDenied(#[from] PermissionDeniedError),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

pub struct UserUseCase<R: Repositories> {
    repositories: Arc<R>,
}

impl<R: Repositories> UserUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }

    pub async fn list(&self, ctx: &Context) -> Result<Vec<UserDto>, UserUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_USER_ALL));

        let raw_user_list = self.repositories.user_repository().list().await?;
        let news_list = raw_user_list.into_iter().map(UserDto::from_entity);
        Ok(news_list.collect())
    }

    pub async fn create(&self, raw_user: CreateUserDto) -> Result<(), UserUseCaseError> {
        let firebase_user = NewFirebaseUser::new(
            FirebaseUserEmail::try_from(raw_user.email.clone())?,
            FirebaseUserPassword::new(raw_user.password.clone()),
        );
        let firebase_user_id = self
            .repositories
            .firebase_user_repository()
            .create(firebase_user)
            .await?;

        let user = (firebase_user_id.clone().value(), raw_user).into_entity()?;
        let res = self.repositories.user_repository().create(user).await;

        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                self.repositories
                    .firebase_user_repository()
                    .delete_by_id(firebase_user_id)
                    .await?;
                Err(e.into())
            }
        }
    }

    pub async fn find_by_id(&self, ctx: &Context, id: String) -> Result<UserDto, UserUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;

        let id = UserId::new(id);
        let raw_user = self
            .repositories
            .user_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(UserUseCaseError::NotFound(id.clone()))?;

        if raw_user.value.is_visible_to(&actor) {
            Ok(UserDto::from_entity(raw_user))
        } else {
            Err(UserUseCaseError::NotFound(id))
        }
    }

    pub async fn update(
        &self,
        ctx: &Context,
        user_data: UpdateUserDto,
    ) -> Result<(), UserUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;

        let id = UserId::new(user_data.id);
        let user = self
            .repositories
            .user_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(UserUseCaseError::NotFound(id.clone()))?;

        if !user.value.is_visible_to(&actor) {
            return Err(UserUseCaseError::NotFound(id));
        }
        if !user.value.is_updatable_by(&actor) {
            return Err(PermissionDeniedError.into());
        }

        let mut new_user = user.value;

        let new_name = UserName::new(user_data.name);
        if new_user.name() != &new_name {
            new_user.set_name(&actor, new_name)?;
        }

        let new_kana_name = UserKanaName::new(user_data.kana_name);
        if new_user.kana_name() != &new_kana_name {
            new_user.set_kana_name(&actor, new_kana_name)?;
        }

        let new_email = UserEmail::try_from(user_data.email)?;
        if new_user.email() != &new_email {
            new_user.set_email(&actor, new_email)?;
        }

        let new_phone_number = UserPhoneNumber::new(user_data.phone_number);
        if new_user.phone_number() != &new_phone_number {
            new_user.set_phone_number(&actor, new_phone_number)?;
        }

        let new_role = user_data.role.into_entity()?;
        if new_user.role() != &new_role {
            new_user.set_role(&actor, new_role)?;
        }

        let new_category = user_data.category.into_entity()?;
        if new_user.category() != &new_category {
            new_user.set_category(&actor, new_category)?;
        }

        self.repositories.user_repository().update(new_user).await?;
        Ok(())
    }

    pub async fn delete_by_id(&self, ctx: &Context, id: String) -> Result<(), UserUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
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

    use anyhow::anyhow;
    use sos24_domain::{
        entity::{permission::PermissionDeniedError, user::UserRole},
        repository::user::UserRepositoryError,
        test::{fixture, repository::MockRepositories},
    };

    use crate::{
        context::Context,
        dto::{
            user::{CreateUserDto, UpdateUserDto, UserCategoryDto, UserRoleDto},
            FromEntity,
        },
        interactor::user::{UserUseCase, UserUseCaseError},
    };

    #[tokio::test]
    async fn list_general_fail() {
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
    async fn list_committee_success() {
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

    #[tokio::test]
    async fn create_success() {
        let mut repositories = MockRepositories::default();
        repositories
            .firebase_user_repository_mut()
            .expect_create()
            .returning(|_| Ok(fixture::firebase_user::id1()));
        repositories
            .user_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = UserUseCase::new(Arc::new(repositories));

        let res = use_case
            .create(CreateUserDto::new(
                fixture::user::name1().value(),
                fixture::user::kana_name1().value(),
                fixture::user::email1().value(),
                fixture::firebase_user::password1().value(),
                fixture::user::phone_number1().value(),
                UserCategoryDto::from_entity(fixture::user::category1()),
            ))
            .await;
        assert!(matches!(res, Ok(())));
    }

    // UserRepositoryでのユーザー作成が失敗した場合にFirebaseRepositoryのユーザーが削除されることを検証する
    #[tokio::test]
    async fn create_fail() {
        let mut repositories = MockRepositories::default();
        repositories
            .firebase_user_repository_mut()
            .expect_create()
            .returning(|_| Ok(fixture::firebase_user::id1()));
        repositories
            .user_repository_mut()
            .expect_create()
            .returning(|_| Err(UserRepositoryError::InternalError(anyhow!("error"))));
        repositories
            .firebase_user_repository_mut()
            .expect_delete_by_id()
            .times(1)
            .returning(|_| Ok(()));
        let use_case = UserUseCase::new(Arc::new(repositories));

        let res = use_case
            .create(CreateUserDto::new(
                fixture::user::name1().value(),
                fixture::user::kana_name1().value(),
                fixture::user::email1().value(),
                fixture::firebase_user::password1().value(),
                fixture::user::phone_number1().value(),
                UserCategoryDto::from_entity(fixture::user::category1()),
            ))
            .await;
        assert!(matches!(
            res,
            Err(UserUseCaseError::UserRepositoryError(
                UserRepositoryError::InternalError(_)
            ))
        ));
    }

    #[tokio::test]
    async fn find_by_id_general_success() {
        let mut repositories = MockRepositories::default();
        repositories
            .user_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::user::user1(
                    UserRole::General,
                ))))
            });
        let use_case = UserUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_id(&ctx, fixture::user::id1().value())
            .await;
        assert!(matches!(res, Ok(_)));
    }

    #[tokio::test]
    async fn find_by_id_general_fail() {
        let mut repositories = MockRepositories::default();
        repositories
            .user_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::user::user2(
                    UserRole::General,
                ))))
            });
        let use_case = UserUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_id(&ctx, fixture::user::id2().value())
            .await;
        assert!(matches!(res, Err(UserUseCaseError::NotFound(_))));
    }

    #[tokio::test]
    async fn find_by_id_committee_success() {
        let mut repositories = MockRepositories::default();
        repositories
            .user_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::user::user2(
                    UserRole::General,
                ))))
            });
        let use_case = UserUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .find_by_id(&ctx, fixture::user::id2().value())
            .await;
        assert!(matches!(res, Ok(_)));
    }

    #[tokio::test]
    async fn update_committee_success() {
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
            .expect_update()
            .returning(|_| Ok(()));
        let use_case = UserUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .update(
                &ctx,
                UpdateUserDto::new(
                    fixture::user::id1().value(),
                    fixture::user::name2().value(),
                    fixture::user::kana_name2().value(),
                    fixture::user::email2().value(),
                    fixture::user::phone_number2().value(),
                    UserRoleDto::from_entity(UserRole::General),
                    UserCategoryDto::from_entity(fixture::user::category2()),
                ),
            )
            .await;
        assert!(matches!(res, Ok(())));
    }

    #[tokio::test]
    async fn update_committee_fail1() {
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
            .expect_update()
            .returning(|_| Ok(()));
        let use_case = UserUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .update(
                &ctx,
                UpdateUserDto::new(
                    fixture::user::id1().value(),
                    fixture::user::name1().value(),
                    fixture::user::kana_name1().value(),
                    fixture::user::email1().value(),
                    fixture::user::phone_number1().value(),
                    UserRoleDto::from_entity(UserRole::Administrator),
                    UserCategoryDto::from_entity(fixture::user::category1()),
                ),
            )
            .await;
        assert!(matches!(
            res,
            Err(UserUseCaseError::PermissionDenied(PermissionDeniedError))
        ));
    }

    #[tokio::test]
    async fn update_committee_fail2() {
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
            .expect_update()
            .returning(|_| Ok(()));
        let use_case = UserUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .update(
                &ctx,
                UpdateUserDto::new(
                    fixture::user::id2().value(),
                    fixture::user::name2().value(),
                    fixture::user::kana_name2().value(),
                    fixture::user::email2().value(),
                    fixture::user::phone_number2().value(),
                    UserRoleDto::from_entity(UserRole::General),
                    UserCategoryDto::from_entity(fixture::user::category2()),
                ),
            )
            .await;
        assert!(matches!(
            res,
            Err(UserUseCaseError::PermissionDenied(PermissionDeniedError))
        ));
    }

    #[tokio::test]
    async fn update_operator_success() {
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
            .expect_update()
            .returning(|_| Ok(()));
        let use_case = UserUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .update(
                &ctx,
                UpdateUserDto::new(
                    fixture::user::id1().value(),
                    fixture::user::name2().value(),
                    fixture::user::kana_name2().value(),
                    fixture::user::email2().value(),
                    fixture::user::phone_number2().value(),
                    UserRoleDto::from_entity(UserRole::Administrator),
                    UserCategoryDto::from_entity(fixture::user::category2()),
                ),
            )
            .await;
        assert!(matches!(res, Ok(())));
    }

    #[tokio::test]
    async fn delete_by_id_committee_fail1() {
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

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .delete_by_id(&ctx, fixture::user::id1().value())
            .await;
        assert!(matches!(
            res,
            Err(UserUseCaseError::PermissionDenied(PermissionDeniedError))
        ));
    }

    #[tokio::test]
    async fn delete_by_id_committee_fail2() {
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

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .delete_by_id(&ctx, fixture::user::id2().value())
            .await;
        assert!(matches!(
            res,
            Err(UserUseCaseError::PermissionDenied(PermissionDeniedError))
        ));
    }

    #[tokio::test]
    async fn delete_by_id_operator_success() {
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

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .delete_by_id(&ctx, fixture::user::id2().value())
            .await;
        assert!(matches!(res, Ok(())));
    }
}
