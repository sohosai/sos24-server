use mockall::automock;
use thiserror::Error;

use crate::entity::{
    common::date::WithDate,
    user::{User, UserEmail, UserId, UserPhoneNumber},
};

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("Email already used: {0:?}")]
    EmailAlreadyUsed(UserEmail),
    #[error("Phone number already used: {0:?}")]
    PhoneNumberAlreadyUsed(UserPhoneNumber),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

#[automock]
#[allow(async_fn_in_trait)]
pub trait UserRepository: Send + Sync + 'static {
    async fn list(&self) -> Result<Vec<WithDate<User>>, UserRepositoryError>;
    async fn create(&self, user: User) -> Result<(), UserRepositoryError>;
    async fn find_by_id(&self, id: UserId) -> Result<Option<WithDate<User>>, UserRepositoryError>;
    async fn update(&self, user: User) -> Result<(), UserRepositoryError>;
    async fn delete_by_id(&self, id: UserId) -> Result<(), UserRepositoryError>;
}
