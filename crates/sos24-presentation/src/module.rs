// FIXME

use std::sync::Arc;

use sos24_use_case::interactor::{news::NewsUseCase, user::UserUseCase};

#[cfg(not(test))]
use sos24_infrastructure::DefaultRepositories;
#[cfg(not(test))]
pub type Repositories = DefaultRepositories;

#[cfg(test)]
use sos24_domain::test::MockRepositories;
#[cfg(test)]
pub type Repositories = MockRepositories;

pub struct Modules {
    news_use_case: NewsUseCase<Repositories>,
    user_use_case: UserUseCase<Repositories>,
}

impl Modules {
    pub fn news_use_case(&self) -> &NewsUseCase<Repositories> {
        &self.news_use_case
    }

    pub fn user_use_case(&self) -> &UserUseCase<Repositories> {
        &self.user_use_case
    }
}

#[cfg(not(test))]
pub async fn new() -> anyhow::Result<Modules> {
    use crate::env;
    use sos24_infrastructure::{firebase::FirebaseAuth, postgresql::Postgresql};

    let db = Postgresql::new(&env::postgres_db_url()).await?;
    let auth = FirebaseAuth::new(&env::firebase_service_account_key()).await?;
    let repository = Arc::new(DefaultRepositories::new(db, auth));
    Ok(Modules {
        news_use_case: NewsUseCase::new(Arc::clone(&repository)),
        user_use_case: UserUseCase::new(Arc::clone(&repository)),
    })
}

#[cfg(test)]
pub async fn new_test(repositories: MockRepositories) -> anyhow::Result<Modules> {
    let repositories = Arc::new(repositories);
    Ok(Modules {
        news_use_case: NewsUseCase::new(Arc::clone(&repositories)),
        user_use_case: UserUseCase::new(Arc::clone(&repositories)),
    })
}
