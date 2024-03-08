use std::future::Future;

use mockall::automock;
use thiserror::Error;

use crate::entity::{
    common::date::WithDate,
    user::{User, UserId},
};

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("Email already used")]
    EmailAlreadyUsed,
    #[error("Phone number already used")]
    PhoneNumberAlreadyUsed,
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
