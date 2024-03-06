use sos24_domain::repository::Repositories;

use crate::postgresql::news::PgNewsRepository;
use crate::postgresql::Postgresql;

pub mod postgresql;

pub struct DefaultRepositories {
    news_repository: PgNewsRepository,
}

impl DefaultRepositories {
    pub fn new(postgresql: Postgresql) -> Self {
        Self {
            news_repository: PgNewsRepository::new(postgresql),
        }
    }
}

impl Repositories for DefaultRepositories {
    type NewsRepositoryImpl = PgNewsRepository;

    fn news_repository(&self) -> &Self::NewsRepositoryImpl {
        &self.news_repository
    }
}
