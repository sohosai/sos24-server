use sos24_domain::entity::common::datetime::DateTime;
use sos24_domain::entity::firebase_user::FirebaseUserEmail;
use sos24_domain::entity::user::{
    User, UserEmail, UserId, UserKanaName, UserName, UserPhoneNumber, UserRole,
};
use sos24_domain::{
    entity::firebase_user::{FirebaseUserPassword, NewFirebaseUser},
    repository::{firebase_user::FirebaseUserRepository, user::UserRepository, Repositories},
};

use crate::user::{UserUseCase, UserUseCaseError};

use super::create::CreateUserCommand;

impl<R: Repositories> UserUseCase<R> {
    pub async fn create_admin(
        &self,
        raw_user: CreateUserCommand,
    ) -> Result<String, UserUseCaseError> {
        if !self.repositories.user_repository().list().await?.is_empty() {
            return Err(UserUseCaseError::UsersAlreadyExist);
        }

        let firebase_user = NewFirebaseUser::new(
            FirebaseUserEmail::try_from(raw_user.email.clone())?,
            FirebaseUserPassword::new(raw_user.password.clone()),
        );
        let firebase_user_id = self
            .repositories
            .firebase_user_repository()
            .create(firebase_user)
            .await?;
        let now = DateTime::now();
        let user = User::new(
            UserId::from(firebase_user_id.clone()),
            UserName::new(raw_user.name),
            UserKanaName::new(raw_user.kana_name),
            UserEmail::try_from(raw_user.email)?,
            UserPhoneNumber::new(raw_user.phone_number),
            UserRole::Administrator,
            now.clone(),
            now,
        );

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
mod tests {} // TODO
