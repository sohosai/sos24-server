use sos24_domain::entity::firebase_user::FirebaseUserEmail;
use sos24_domain::entity::user::{
    User, UserEmail, UserId, UserKanaName, UserName, UserPhoneNumber,
};
use sos24_domain::{
    entity::firebase_user::{FirebaseUserPassword, NewFirebaseUser},
    repository::{firebase_user::FirebaseUserRepository, user::UserRepository, Repositories},
};

use crate::user::{UserUseCase, UserUseCaseError};
use crate::ToEntity;

#[derive(Debug)]
pub struct CreateUserCommand {
    pub name: String,
    pub kana_name: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
}

impl ToEntity for (String, CreateUserCommand) {
    type Entity = User;
    type Error = UserUseCaseError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        let (id, user) = self;
        Ok(User::new_general(
            UserId::new(id),
            UserName::new(user.name),
            UserKanaName::new(user.kana_name),
            UserEmail::try_from(user.email)?,
            UserPhoneNumber::new(user.phone_number),
        ))
    }
}

impl<R: Repositories> UserUseCase<R> {
    pub async fn create(&self, raw_user: CreateUserCommand) -> Result<String, UserUseCaseError> {
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
        let user_id = user.id().clone();
        let res = self.repositories.user_repository().create(user).await;

        match res {
            Ok(_) => Ok(user_id.value().to_string()),
            Err(e) => {
                self.repositories
                    .firebase_user_repository()
                    .delete_by_id(firebase_user_id)
                    .await?;
                Err(e.into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use anyhow::anyhow;

    use sos24_domain::{
        repository::user::UserRepositoryError,
        test::{fixture, repository::MockRepositories},
    };

    use crate::user::{interactor::create::CreateUserCommand, UserUseCase, UserUseCaseError};

    #[tokio::test]
    async fn 誰でもユーザーを作成できる() {
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
            .create(CreateUserCommand {
                name: fixture::user::name1().value(),
                kana_name: fixture::user::kana_name1().value(),
                email: fixture::user::email1().value(),
                password: fixture::firebase_user::password1().value(),
                phone_number: fixture::user::phone_number1().value(),
            })
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn ユーザー作成が失敗した場合にfirebaseのユーザーが削除される() {
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
            .create(CreateUserCommand {
                name: fixture::user::name1().value(),
                kana_name: fixture::user::kana_name1().value(),
                email: fixture::user::email1().value(),
                password: fixture::firebase_user::password1().value(),
                phone_number: fixture::user::phone_number1().value(),
            })
            .await;
        assert!(matches!(
            res,
            Err(UserUseCaseError::UserRepositoryError(
                UserRepositoryError::InternalError(_)
            ))
        ));
    }
}
