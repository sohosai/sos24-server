use postgresql::user::PgUserRepository;
use sos24_domain::repository::Repositories;

use crate::postgresql::news::PgNewsRepository;
use crate::postgresql::Postgresql;

pub mod postgresql;

pub struct DefaultRepositories {
    news_repository: PgNewsRepository,
    user_repository: PgUserRepository,
}

impl DefaultRepositories {
    pub fn new(postgresql: Postgresql) -> Self {
        Self {
            news_repository: PgNewsRepository::new(postgresql.clone()),
            user_repository: PgUserRepository::new(postgresql.clone()),
        }
    }
}

impl Repositories for DefaultRepositories {
    type NewsRepositoryImpl = PgNewsRepository;
    type UserRepositoryImpl = PgUserRepository;

    fn news_repository(&self) -> &Self::NewsRepositoryImpl {
        &self.news_repository
    }

    fn user_repository(&self) -> &Self::UserRepositoryImpl {
        &self.user_repository
    }
}
