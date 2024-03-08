use firebase::firebase_user::FirebaseUserRepositoryImpl;
use firebase::FirebaseAuth;
use postgresql::news_attachment::PgNewsAttachmentRepository;
use postgresql::user::PgUserRepository;
use sos24_domain::repository::Repositories;

use crate::postgresql::news::PgNewsRepository;
use crate::postgresql::Postgresql;

pub mod firebase;
pub mod postgresql;

pub struct DefaultRepositories {
    firebase_user_repository: FirebaseUserRepositoryImpl,
    news_repository: PgNewsRepository,
    news_attachment_repository: PgNewsAttachmentRepository,
    user_repository: PgUserRepository,
}

impl DefaultRepositories {
    pub fn new(postgresql: Postgresql, auth: FirebaseAuth) -> Self {
        Self {
            firebase_user_repository: FirebaseUserRepositoryImpl::new(auth),
            news_repository: PgNewsRepository::new(postgresql.clone()),
            news_attachment_repository: PgNewsAttachmentRepository::new(postgresql.clone()),
            user_repository: PgUserRepository::new(postgresql.clone()),
        }
    }
}

impl Repositories for DefaultRepositories {
    type FirebaseUserRepositoryImpl = FirebaseUserRepositoryImpl;
    type NewsRepositoryImpl = PgNewsRepository;
    type UserRepositoryImpl = PgUserRepository;
    type NewsAttachmentRepositoryImpl = PgNewsAttachmentRepository;

    fn firebase_user_repository(&self) -> &Self::FirebaseUserRepositoryImpl {
        &self.firebase_user_repository
    }

    fn news_repository(&self) -> &Self::NewsRepositoryImpl {
        &self.news_repository
    }

    fn news_attachment_repository(&self) -> &Self::NewsAttachmentRepositoryImpl {
        &self.news_attachment_repository
    }

    fn user_repository(&self) -> &Self::UserRepositoryImpl {
        &self.user_repository
    }
}
