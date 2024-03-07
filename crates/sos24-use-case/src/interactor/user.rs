use std::sync::Arc;

use anyhow::Context;
use sos24_domain::entity::firebase_user::{
    FirebaseUserEmail, FirebaseUserPassword, NewFirebaseUser,
};
use sos24_domain::entity::user::UserId;
use sos24_domain::repository::firebase_user::FirebaseUserRepository;
use sos24_domain::repository::{user::UserRepository, Repositories};

use crate::dto::user::UserDto;
use crate::dto::FromEntity;
use crate::error::{Result, UseCaseError};
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

    pub async fn find_by_id(&self, id: &str) -> Result<UserDto, UserError> {
        // TODO: 権限チェック
        let id = UserId::new(id.to_string());
        let raw_user = self
            .repositories
            .user_repository()
            .find_by_id(id.clone())
            .await
            .context("Failed to find user")?;

        match raw_user {
            Some(raw_user) => Ok(UserDto::from_entity(raw_user)),
            None => Err(UseCaseError::UseCase(UserError::NotFound(id))),
        }
    }
}
