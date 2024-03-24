use firebase::firebase_user::FirebaseUserRepositoryImpl;
use firebase::FirebaseAuth;
use mongodb::form::MongoFormRepository;
use mongodb::form_answer::MongoFormAnswerRepository;
use mongodb::MongoDb;
use postgresql::invitation::PgInvitationRepository;
use postgresql::news_attachment::PgNewsAttachmentRepository;
use postgresql::project::PgProjectRepository;
use postgresql::user::PgUserRepository;
use s3::news_attachment::NewsAttachmentRepository;
use s3::S3;
use sos24_domain::repository::Repositories;

use crate::postgresql::news::PgNewsRepository;
use crate::postgresql::Postgresql;

pub mod firebase;
pub mod mongodb;
pub mod postgresql;
pub mod s3;

pub struct DefaultRepositories {
    firebase_user_repository: FirebaseUserRepositoryImpl,
    form_repository: MongoFormRepository,
    form_answer_repository: MongoFormAnswerRepository,
    invitation_repository: PgInvitationRepository,
    news_repository: PgNewsRepository,
    project_repository: PgProjectRepository,
    news_attachment_repository: PgNewsAttachmentRepository,
    user_repository: PgUserRepository,
    news_attachment_object_repository: NewsAttachmentRepository,
}

impl DefaultRepositories {
    pub fn new(postgresql: Postgresql, mongodb: MongoDb, auth: FirebaseAuth, s3: S3) -> Self {
        Self {
            firebase_user_repository: FirebaseUserRepositoryImpl::new(auth),
            form_repository: MongoFormRepository::new(mongodb.clone()),
            form_answer_repository: MongoFormAnswerRepository::new(mongodb.clone()),
            invitation_repository: PgInvitationRepository::new(postgresql.clone()),
            news_repository: PgNewsRepository::new(postgresql.clone()),
            project_repository: PgProjectRepository::new(postgresql.clone()),
            news_attachment_repository: PgNewsAttachmentRepository::new(postgresql.clone()),
            user_repository: PgUserRepository::new(postgresql.clone()),
            news_attachment_object_repository: NewsAttachmentRepository::new(s3.clone()),
        }
    }
}

impl Repositories for DefaultRepositories {
    type FirebaseUserRepositoryImpl = FirebaseUserRepositoryImpl;
    type FormRepositoryImpl = MongoFormRepository;
    type FormAnswerRepositoryImpl = MongoFormAnswerRepository;
    type InvitationRepositoryImpl = PgInvitationRepository;
    type NewsRepositoryImpl = PgNewsRepository;
    type ProjectRepositoryImpl = PgProjectRepository;
    type UserRepositoryImpl = PgUserRepository;
    type NewsAttachmentRepositoryImpl = PgNewsAttachmentRepository;
    type NewsAttachmentObjectRepositoryImpl = NewsAttachmentRepository;

    fn firebase_user_repository(&self) -> &Self::FirebaseUserRepositoryImpl {
        &self.firebase_user_repository
    }

    fn form_repository(&self) -> &Self::FormRepositoryImpl {
        &self.form_repository
    }

    fn form_answer_repository(&self) -> &Self::FormAnswerRepositoryImpl {
        &self.form_answer_repository
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

    fn news_attachment_repository(&self) -> &Self::NewsAttachmentRepositoryImpl {
        &self.news_attachment_repository
    }

    fn user_repository(&self) -> &Self::UserRepositoryImpl {
        &self.user_repository
    }

    fn news_attachment_object_repository(&self) -> &Self::NewsAttachmentObjectRepositoryImpl {
        &self.news_attachment_object_repository
    }
}
