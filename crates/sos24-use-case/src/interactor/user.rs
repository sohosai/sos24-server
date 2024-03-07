use std::sync::Arc;

use sos24_domain::entity::firebase_user::{
    FirebaseUserEmail, FirebaseUserPassword, NewFirebaseUser,
};
use sos24_domain::repository::firebase_user::FirebaseUserRepository;
use sos24_domain::repository::{user::UserRepository, Repositories};

use crate::error::Result;
use crate::{
    dto::{user::CreateUserDto, ToEntity},
    error::user::UserError,
};

pub struct UserUseCase<R: Repositories> {
    repositories: Arc<R>,
}

impl<R: Repositories> UserUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }

    pub async fn create(&self, raw_user: CreateUserDto) -> Result<(), UserError> {
        // TODO: 権限チェック
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

        if let Err(e) = res {
            self.repositories
                .firebase_user_repository()
                .delete_by_id(firebase_user_id)
                .await?;
            return Err(e.into());
        };

        Ok(())
    }
}
