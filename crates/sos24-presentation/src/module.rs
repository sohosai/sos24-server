use std::sync::Arc;

use sos24_infrastructure::{postgresql::Postgresql, DefaultRepositories};
use sos24_use_case::interactor::news::NewsUseCase;

use crate::env;

pub struct Modules {
    news_use_case: NewsUseCase<DefaultRepositories>,
}

impl Modules {
    pub fn news_use_case(&self) -> &NewsUseCase<DefaultRepositories> {
        &self.news_use_case
    }
}

impl Modules {
    pub async fn new() -> anyhow::Result<Self> {
        let db = Postgresql::new(&env::postgres_db_url()).await?;
        let repository = Arc::new(DefaultRepositories::new(db));
        Ok(Self {
            news_use_case: NewsUseCase::new(Arc::clone(&repository)),
        })
    }
}