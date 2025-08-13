use email::SendGridEmailSender;
use file_data::PgFileDataRepository;
use file_object::S3FileObjectRepository;
use firebase_user::FirebaseUserRepositoryImpl;
use form::MongoFormRepository;
use form_answer::MongoFormAnswerRepository;
use health::DatabaseHealthChecker;
use invitation::PgInvitationRepository;
use news::PgNewsRepository;
use notification::SlackNotifier;
use project::PgProjectRepository;
use shared::{
    firebase::FirebaseAuth, mongodb::MongoDb, postgresql::Postgresql, s3::S3, sendgrid::SendGrid,
};
use sos24_domain::repository::Repositories;
use sos24_use_case::shared::adapter::Adapters;
use user::PgUserRepository;

pub mod email;
pub mod file_data;
pub mod file_object;
pub mod firebase_user;
pub mod form;
pub mod form_answer;
pub mod health;
pub mod invitation;
pub mod news;
pub mod notification;
pub mod project;
pub mod shared;
pub mod user;

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
    health_checker: DatabaseHealthChecker,
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
            health_checker: DatabaseHealthChecker::new(postgresql, mongodb),
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
    type HealthCheckerImpl = DatabaseHealthChecker;

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

    fn health_checker(&self) -> &Self::HealthCheckerImpl {
        &self.health_checker
    }
}

pub struct DefaultAdapters {
    email_sender: SendGridEmailSender,
    notifier: SlackNotifier,
}

impl DefaultAdapters {
    pub fn new(send_grid: SendGrid, slack_webhook_url: Option<String>) -> Self {
        Self {
            email_sender: SendGridEmailSender::new(send_grid),
            notifier: SlackNotifier::new(slack_webhook_url),
        }
    }
}

impl Adapters for DefaultAdapters {
    type EmailSenderImpl = SendGridEmailSender;
    type NotifierImpl = SlackNotifier;

    fn email_sender(&self) -> &Self::EmailSenderImpl {
        &self.email_sender
    }

    fn notifier(&self) -> &Self::NotifierImpl {
        &self.notifier
    }
}
