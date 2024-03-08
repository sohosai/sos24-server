use std::future::Future;

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
pub trait UserRepository: Send + Sync + 'static {
    fn list(&self)
        -> impl Future<Output = Result<Vec<WithDate<User>>, UserRepositoryError>> + Send;

    fn create(&self, user: User) -> impl Future<Output = Result<(), UserRepositoryError>> + Send;

    fn find_by_id(
        &self,
        id: UserId,
    ) -> impl Future<Output = Result<Option<WithDate<User>>, UserRepositoryError>> + Send;

    fn update(&self, user: User) -> impl Future<Output = Result<(), UserRepositoryError>> + Send;

    fn delete_by_id(
        &self,
        id: UserId,
    ) -> impl Future<Output = Result<(), UserRepositoryError>> + Send;
}
