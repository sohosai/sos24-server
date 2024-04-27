use sos24_domain::ensure;
use sos24_domain::entity::firebase_user::FirebaseUserId;
use sos24_domain::entity::user::{
    UserEmail, UserId, UserKanaName, UserName, UserPhoneNumber, UserRole,
};
use sos24_domain::repository::firebase_user::FirebaseUserRepository;
use sos24_domain::repository::{user::UserRepository, Repositories};

use crate::shared::context::ContextProvider;
use crate::user::dto::UserRoleDto;
use crate::user::{UserUseCase, UserUseCaseError};

#[derive(Debug)]
pub struct UpdateUserCommand {
    pub id: String,
    pub name: String,
    pub kana_name: String,
    pub email: String,
    pub phone_number: String,
    pub role: UserRoleDto,
}

impl<R: Repositories> UserUseCase<R> {
    pub async fn update(
        &self,
        ctx: &impl ContextProvider,
        user_data: UpdateUserCommand,
    ) -> Result<(), UserUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;

        let id = UserId::new(user_data.id);
        let user = self
            .repositories
            .user_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(UserUseCaseError::NotFound(id.clone()))?;
        ensure!(user.value.is_updatable_by(&actor));

        let mut new_user = user.value;

        new_user.set_name(&actor, UserName::new(user_data.name))?;
        new_user.set_kana_name(&actor, UserKanaName::new(user_data.kana_name))?;
        new_user.set_phone_number(&actor, UserPhoneNumber::new(user_data.phone_number))?;
        new_user.set_role(&actor, UserRole::from(user_data.role))?;

        let firebase_user_id: FirebaseUserId = new_user.id().clone().into();

        let old_email = new_user.email().clone();
        let new_email = UserEmail::try_from(user_data.email)?;
        if old_email != new_email {
            let firebase_user_new_email = new_email.clone().into();
            new_user.set_email(&actor, new_email)?;
            self.repositories
                .firebase_user_repository()
                .update_email_by_id(firebase_user_id.clone(), firebase_user_new_email)
                .await?;
        }

        let res = self.repositories.user_repository().update(new_user).await;
        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                let firebase_user_old_email = old_email.clone().into();
                self.repositories
                    .firebase_user_repository()
                    .update_email_by_id(firebase_user_id, firebase_user_old_email)
                    .await?;
                Err(e.into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::entity::permission::PermissionDeniedError;
    use sos24_domain::entity::user::UserRole;
    use sos24_domain::test::{fixture, repository::MockRepositories};

    use crate::shared::context::TestContext;
    use crate::user::dto::UserRoleDto;
    use crate::user::interactor::update::UpdateUserCommand;
    use crate::user::{UserUseCase, UserUseCaseError};

    #[tokio::test]
    async fn 実委人は自分のユーザーを更新できる() {
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
        repositories
            .firebase_user_repository_mut()
            .expect_update_email_by_id()
            .returning(|_, _| Ok(()));
        let use_case = UserUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .update(
                &ctx,
                UpdateUserCommand {
                    id: fixture::user::id1().value(),
                    name: fixture::user::name2().value(),
                    kana_name: fixture::user::kana_name2().value(),
                    email: fixture::user::email2().value(),
                    phone_number: fixture::user::phone_number2().value(),
                    role: UserRoleDto::from(UserRole::General),
                },
            )
            .await;
        assert!(matches!(res, Ok(())));
    }

    #[tokio::test]
    async fn 実委人は自分のロールを更新できない() {
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

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .update(
                &ctx,
                UpdateUserCommand {
                    id: fixture::user::id1().value(),
                    name: fixture::user::name1().value(),
                    kana_name: fixture::user::kana_name1().value(),
                    email: fixture::user::email1().value(),
                    phone_number: fixture::user::phone_number1().value(),
                    role: UserRoleDto::from(UserRole::Administrator),
                },
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
    async fn 実委人は他人のユーザーを更新できない() {
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

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .update(
                &ctx,
                UpdateUserCommand {
                    id: fixture::user::id2().value(),
                    name: fixture::user::name2().value(),
                    kana_name: fixture::user::kana_name2().value(),
                    email: fixture::user::email2().value(),
                    phone_number: fixture::user::phone_number2().value(),
                    role: UserRoleDto::from(UserRole::General),
                },
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
    async fn 実委人管理者はロールを含む自分のユーザーを更新できる() {
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

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .update(
                &ctx,
                UpdateUserCommand {
                    id: fixture::user::id1().value(),
                    name: fixture::user::name2().value(),
                    kana_name: fixture::user::kana_name2().value(),
                    email: fixture::user::email2().value(),
                    phone_number: fixture::user::phone_number2().value(),
                    role: UserRoleDto::from(UserRole::Administrator),
                },
            )
            .await;
        assert!(matches!(res, Ok(())));
    }
}
