use std::sync::Arc;

use anyhow::Context;
use sos24_domain::entity::firebase_user::{
    FirebaseUserEmail, FirebaseUserPassword, NewFirebaseUser,
};
use sos24_domain::entity::user::{UserEmail, UserId, UserKanaName, UserName, UserPhoneNumber};
use sos24_domain::repository::firebase_user::FirebaseUserRepository;
use sos24_domain::repository::{user::UserRepository, Repositories};

use crate::dto::user::{UpdateUserDto, UserDto};
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

    pub async fn list(&self) -> Result<Vec<UserDto>, UserError> {
        // TODO: 権限チェック
        let raw_user_list = self
            .repositories
            .user_repository()
            .list()
            .await
            .context("Failed to list user")?;

        let news_list = raw_user_list
            .into_iter()
            .map(UserDto::from_entity)
            .collect();
        Ok(news_list)
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

    pub async fn find_by_id(&self, id: String) -> Result<UserDto, UserError> {
        // TODO: 権限チェック
        let id = UserId::new(id);
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

    pub async fn update(&self, user_data: UpdateUserDto) -> Result<(), UserError> {
        // TODO: 権限チェック
        let id = UserId::new(user_data.id);
        let user = self
            .repositories
            .user_repository()
            .find_by_id(id.clone())
            .await
            .context("Failed to find user")?
            .ok_or(UseCaseError::UseCase(UserError::NotFound(id)))?;

        let mut new_user = user.value;
        new_user.set_name(UserName::new(user_data.name));
        new_user.set_kana_name(UserKanaName::new(user_data.kana_name));
        new_user.set_email(UserEmail::try_from(user_data.email)?);
        new_user.set_phone_number(UserPhoneNumber::new(user_data.phone_number));
        new_user.set_role(user_data.role.into_entity()?);
        new_user.set_category(user_data.category.into_entity()?);

        self.repositories
            .user_repository()
            .update(new_user)
            .await
            .context("Failed to update user")?;

        Ok(())
    }

    pub async fn delete_by_id(&self, id: String) -> Result<(), UserError> {
        let id = UserId::new(id);

        self.repositories
            .user_repository()
            .find_by_id(id.clone())
            .await
            .context("Failed to find user")?
            .ok_or(UseCaseError::UseCase(UserError::NotFound(id.clone())))?;

        self.repositories
            .user_repository()
            .delete_by_id(id)
            .await
            .context("Failed to delete user")?;

        Ok(())
    }
}
