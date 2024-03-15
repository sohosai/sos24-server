use firebase::firebase_user::FirebaseUserRepositoryImpl;
use firebase::FirebaseAuth;
use mongodb::form::MongoFormRepository;
use mongodb::MongoDb;
use postgresql::invitation::PgInvitationRepository;
use postgresql::project::PgProjectRepository;
use postgresql::user::PgUserRepository;
use sos24_domain::repository::Repositories;

use crate::postgresql::news::PgNewsRepository;
use crate::postgresql::Postgresql;

pub mod firebase;
pub mod mongodb;
pub mod postgresql;

pub struct DefaultRepositories {
    firebase_user_repository: FirebaseUserRepositoryImpl,
    form_repository: MongoFormRepository,
    invitation_repository: PgInvitationRepository,
    news_repository: PgNewsRepository,
    project_repository: PgProjectRepository,
    user_repository: PgUserRepository,
}

impl DefaultRepositories {
    pub fn new(postgresql: Postgresql, mongodb: MongoDb, auth: FirebaseAuth) -> Self {
        Self {
            firebase_user_repository: FirebaseUserRepositoryImpl::new(auth),
            form_repository: MongoFormRepository::new(mongodb.clone()),
            invitation_repository: PgInvitationRepository::new(postgresql.clone()),
            news_repository: PgNewsRepository::new(postgresql.clone()),
            project_repository: PgProjectRepository::new(postgresql.clone()),
            user_repository: PgUserRepository::new(postgresql.clone()),
        }
    }
}

impl Repositories for DefaultRepositories {
    type FirebaseUserRepositoryImpl = FirebaseUserRepositoryImpl;
    type FormRepositoryImpl = MongoFormRepository;
    type InvitationRepositoryImpl = PgInvitationRepository;
    type NewsRepositoryImpl = PgNewsRepository;
    type ProjectRepositoryImpl = PgProjectRepository;
    type UserRepositoryImpl = PgUserRepository;

    fn firebase_user_repository(&self) -> &Self::FirebaseUserRepositoryImpl {
        &self.firebase_user_repository
    }

    fn form_repository(&self) -> &Self::FormRepositoryImpl {
        &self.form_repository
    }

    fn invitation_repository(&self) -> &Self::InvitationRepositoryImpl {
        &self.invitation_repository
    }

    fn news_repository(&self) -> &Self::NewsRepositoryImpl {
        &self.news_repository
    }

    fn project_repository(&self) -> &Self::ProjectRepositoryImpl {
        &self.project_repository
    }

    fn user_repository(&self) -> &Self::UserRepositoryImpl {
        &self.user_repository
    }
}
