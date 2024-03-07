use std::sync::Arc;

use sos24_infrastructure::{firebase::FirebaseAuth, postgresql::Postgresql, DefaultRepositories};
use sos24_use_case::interactor::{news::NewsUseCase, user::UserUseCase};

use crate::env;

pub struct Modules {
    news_use_case: NewsUseCase<DefaultRepositories>,
    user_use_case: UserUseCase<DefaultRepositories>,
}

impl Modules {
    pub fn news_use_case(&self) -> &NewsUseCase<DefaultRepositories> {
        &self.news_use_case
    }

    pub fn user_use_case(&self) -> &UserUseCase<DefaultRepositories> {
        &self.user_use_case
    }
}

impl Modules {
    pub async fn new() -> anyhow::Result<Self> {
        let db = Postgresql::new(&env::postgres_db_url()).await?;
        let auth = FirebaseAuth::new(&env::firebase_service_account_key()).await?;
        let repository = Arc::new(DefaultRepositories::new(db, auth));
        Ok(Self {
            news_use_case: NewsUseCase::new(Arc::clone(&repository)),
            user_use_case: UserUseCase::new(Arc::clone(&repository)),
        })
    }
}
