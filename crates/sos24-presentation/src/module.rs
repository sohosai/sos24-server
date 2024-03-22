// FIXME

use std::sync::Arc;

use crate::config::Config;
use sos24_domain::entity::project_application_period::ProjectApplicationPeriod;
use sos24_use_case::interactor::news_attachment::NewsAttachmentUseCase;
use sos24_use_case::interactor::{
    form::FormUseCase, form_answer::FormAnswerUseCase, invitation::InvitationUseCase,
    news::NewsUseCase, project::ProjectUseCase, user::UserUseCase,
};

#[cfg(not(test))]
use sos24_infrastructure::DefaultRepositories;
#[cfg(not(test))]
pub type Repositories = DefaultRepositories;

#[cfg(test)]
use sos24_domain::test::repository::MockRepositories;
#[cfg(test)]
pub type Repositories = MockRepositories;

pub struct Modules {
    config: Config,
    form_use_case: FormUseCase<Repositories>,
    form_answer_use_case: FormAnswerUseCase<Repositories>,
    invitation_use_case: InvitationUseCase<Repositories>,
    news_use_case: NewsUseCase<Repositories>,
    news_attachment_use_case: NewsAttachmentUseCase<Repositories>,
    project_use_case: ProjectUseCase<Repositories>,
    user_use_case: UserUseCase<Repositories>,
}

impl Modules {
    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn form_use_case(&self) -> &FormUseCase<Repositories> {
        &self.form_use_case
    }

    pub fn form_answer_use_case(&self) -> &FormAnswerUseCase<Repositories> {
        &self.form_answer_use_case
    }

    pub fn invitation_use_case(&self) -> &InvitationUseCase<Repositories> {
        &self.invitation_use_case
    }

    pub fn news_use_case(&self) -> &NewsUseCase<Repositories> {
        &self.news_use_case
    }

    pub fn news_attachment_use_case(&self) -> &NewsAttachmentUseCase<Repositories> {
        &self.news_attachment_use_case
    }

    pub fn project_use_case(&self) -> &ProjectUseCase<Repositories> {
        &self.project_use_case
    }

    pub fn user_use_case(&self) -> &UserUseCase<Repositories> {
        &self.user_use_case
    }
}

#[cfg(not(test))]
pub async fn new(config: Config) -> anyhow::Result<Modules> {
    use crate::env;
    use sos24_infrastructure::{firebase::FirebaseAuth, mongodb::MongoDb, postgresql::Postgresql};

    let db = Postgresql::new(&env::postgres_db_url()).await?;
    let mongo_db = MongoDb::new(&env::mongodb_db_url(), &env::mongodb_db_name()).await?;
    let auth = FirebaseAuth::new(&env::firebase_service_account_key()).await?;
    let repository = Arc::new(DefaultRepositories::new(db, mongo_db, auth));

    let application_period = ProjectApplicationPeriod::new(
        config.project_application_start_at.clone(),
        config.project_application_end_at.clone(),
    );

    Ok(Modules {
        config,
        form_use_case: FormUseCase::new(Arc::clone(&repository)),
        form_answer_use_case: FormAnswerUseCase::new(Arc::clone(&repository)),
        invitation_use_case: InvitationUseCase::new(
            Arc::clone(&repository),
            application_period.clone(),
        ),
        news_use_case: NewsUseCase::new(Arc::clone(&repository)),
        news_attachment_use_case: NewsAttachmentUseCase::new(Arc::clone(&repository)),
        project_use_case: ProjectUseCase::new(Arc::clone(&repository), application_period),
        user_use_case: UserUseCase::new(Arc::clone(&repository)),
    })
}

#[cfg(test)]
pub async fn new_test(repositories: MockRepositories) -> anyhow::Result<Modules> {
    let repositories = Arc::new(repositories);

    let application_period = ProjectApplicationPeriod::default();

    Ok(Modules {
        config: Config::default(),
        form_use_case: FormUseCase::new(Arc::clone(&repositories)),
        form_answer_use_case: FormAnswerUseCase::new(Arc::clone(&repositories)),
        invitation_use_case: InvitationUseCase::new(
            Arc::clone(&repositories),
            application_period.clone(),
        ),
        news_use_case: NewsUseCase::new(Arc::clone(&repositories)),
        news_attachment_use_case: NewsAttachmentUseCase::new(Arc::clone(&repositories)),
        project_use_case: ProjectUseCase::new(Arc::clone(&repositories), application_period),
        user_use_case: UserUseCase::new(Arc::clone(&repositories)),
    })
}
