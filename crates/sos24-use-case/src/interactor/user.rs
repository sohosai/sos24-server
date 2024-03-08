use std::sync::Arc;

use sos24_domain::ensure;
use sos24_domain::entity::actor::Actor;
use sos24_domain::entity::common::email::EmailError;
use sos24_domain::entity::firebase_user::{
    FirebaseUserEmail, FirebaseUserPassword, NewFirebaseUser,
};
use sos24_domain::entity::permission::{PermissionDeniedError, Permissions};
use sos24_domain::entity::user::{UserEmail, UserId, UserKanaName, UserName, UserPhoneNumber};
use sos24_domain::repository::firebase_user::FirebaseUserRepository;
use sos24_domain::repository::user::UserRepositoryError;
use sos24_domain::repository::{user::UserRepository, Repositories};
use thiserror::Error;

use crate::dto::user::{UpdateUserDto, UserDto};
use crate::dto::FromEntity;
use crate::dto::{user::CreateUserDto, ToEntity};

#[derive(Debug, Error)]
pub enum UserUseCaseError {
    #[error("User not found: {0:?}")]
    NotFound(UserId),

    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
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

    pub async fn list(&self, actor: &Actor) -> Result<Vec<UserDto>, UserUseCaseError> {
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

    pub async fn find_by_id(&self, actor: &Actor, id: String) -> Result<UserDto, UserUseCaseError> {
        let id = UserId::new(id);
        let raw_user = self
            .repositories
            .user_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(UserUseCaseError::NotFound(id.clone()))?;

        if raw_user.value.is_visible_to(actor) {
            Ok(UserDto::from_entity(raw_user))
        } else {
            Err(UserUseCaseError::NotFound(id))
        }
    }

    pub async fn find_by_id_as_actor(&self, id: String) -> Result<Actor, UserUseCaseError> {
        let id = UserId::new(id);
        let raw_user = self
            .repositories
            .user_repository()
            .find_by_id(id.clone())
            .await?;

        match raw_user {
            Some(raw_user) => Ok(raw_user.value.into_actor()),
            _ => Err(UserUseCaseError::NotFound(id)),
        }
    }

    pub async fn update(
        &self,
        actor: &Actor,
        user_data: UpdateUserDto,
    ) -> Result<(), UserUseCaseError> {
        let id = UserId::new(user_data.id);
        let user = self
            .repositories
            .user_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(UserUseCaseError::NotFound(id.clone()))?;

        if !user.value.is_visible_to(actor) {
            return Err(UserUseCaseError::NotFound(id));
        }

        let mut new_user = user.value;
        new_user.set_name(actor, UserName::new(user_data.name))?;
        new_user.set_kana_name(actor, UserKanaName::new(user_data.kana_name))?;
        new_user.set_email(actor, UserEmail::try_from(user_data.email)?)?;
        new_user.set_phone_number(actor, UserPhoneNumber::new(user_data.phone_number))?;
        new_user.set_role(actor, user_data.role.into_entity()?)?;
        new_user.set_category(actor, user_data.category.into_entity()?)?;

        self.repositories.user_repository().update(new_user).await?;
        Ok(())
    }

    pub async fn delete_by_id(&self, actor: &Actor, id: String) -> Result<(), UserUseCaseError> {
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
