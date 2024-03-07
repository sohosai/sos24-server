use std::sync::Arc;

use anyhow::Context;
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
        let user = raw_user.into_entity()?;
        self.repositories
            .user_repository()
            .create(user)
            .await
            .context("Failed to create user")?;
        Ok(())
    }
}
