use std::sync::Arc;

use sos24_domain::entity::permission::PermissionDeniedError;
use sos24_domain::entity::user::{UserEmail, UserId, UserKanaName, UserName, UserPhoneNumber};
use sos24_domain::repository::{user::UserRepository, Repositories};

use crate::context::Context;
use crate::dto::user::UpdateUserDto;
use crate::dto::ToEntity;
use crate::interactor::user::{UserUseCase, UserUseCaseError};

impl<R: Repositories> UserUseCase<R> {
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
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::entity::permission::PermissionDeniedError;
    use sos24_domain::entity::user::UserRole;
    use sos24_domain::test::{fixture, repository::MockRepositories};

    use crate::context::Context;
    use crate::dto::user::{UpdateUserDto, UserCategoryDto, UserRoleDto};
    use crate::dto::FromEntity;
    use crate::interactor::user::{UserUseCase, UserUseCaseError};

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
            Err(UserUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
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
            Err(UserUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
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
}
