use firebase::firebase_user::FirebaseUserRepositoryImpl;
use firebase::FirebaseAuth;
use mongodb::form::MongoFormRepository;
use mongodb::form_answer::MongoFormAnswerRepository;
use mongodb::MongoDb;
use postgresql::file_data::PgFileDataRepository;
use postgresql::invitation::PgInvitationRepository;
use postgresql::project::PgProjectRepository;
use postgresql::user::PgUserRepository;
use s3::file_object::S3FileObjectRepository;
use s3::S3;
use sendgrid::email::SendGridEmailSender;
use sendgrid::SendGrid;
use sos24_domain::repository::Repositories;
use sos24_use_case::adapter::Adapters;

use crate::postgresql::news::PgNewsRepository;
use crate::postgresql::Postgresql;

pub mod firebase;
pub mod mongodb;
pub mod postgresql;
pub mod s3;
pub mod sendgrid;

pub struct DefaultRepositories {
    firebase_user_repository: FirebaseUserRepositoryImpl,
    form_repository: MongoFormRepository,
    form_answer_repository: MongoFormAnswerRepository,
    invitation_repository: PgInvitationRepository,
    news_repository: PgNewsRepository,
    project_repository: PgProjectRepository,
    file_data_repository: PgFileDataRepository,
    user_repository: PgUserRepository,
    file_object_repository: S3FileObjectRepository,
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
            file_data_repository: PgFileDataRepository::new(postgresql.clone()),
            user_repository: PgUserRepository::new(postgresql.clone()),
            file_object_repository: S3FileObjectRepository::new(s3.clone()),
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
    type FileDataRepositoryImpl = PgFileDataRepository;
    type FileObjectRepositoryImpl = S3FileObjectRepository;
    type UserRepositoryImpl = PgUserRepository;

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

    fn file_data_repository(&self) -> &Self::FileDataRepositoryImpl {
        &self.file_data_repository
    }

    fn file_object_repository(&self) -> &Self::FileObjectRepositoryImpl {
        &self.file_object_repository
    }

    fn user_repository(&self) -> &Self::UserRepositoryImpl {
        &self.user_repository
    }
}

pub struct DefaultAdapters {
    email_sender: SendGridEmailSender,
}

impl DefaultAdapters {
    pub fn new(send_grid: SendGrid) -> Self {
        Self {
            email_sender: SendGridEmailSender::new(send_grid),
        }
    }
}

impl Adapters for DefaultAdapters {
    type EmailSenderImpl = SendGridEmailSender;

    fn email_sender(&self) -> &Self::EmailSenderImpl {
        &self.email_sender
    }
}
