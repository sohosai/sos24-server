// FIXME

use std::sync::Arc;

use crate::config::Config;
use sos24_domain::entity::project_application_period::ProjectApplicationPeriod;
use sos24_use_case::file::FileUseCase;
use sos24_use_case::{
    form::FormUseCase, form_answer::FormAnswerUseCase, invitation::InvitationUseCase,
    news::NewsUseCase, project::ProjectUseCase, user::UserUseCase,
};

#[cfg(not(test))]
mod modules {
    pub type Repositories = sos24_infrastructure::DefaultRepositories;
    pub type Adapters = sos24_infrastructure::DefaultAdapters;
}

#[cfg(test)]
#[allow(clippy::items_after_test_module)]
mod modules {
    pub type Repositories = sos24_domain::test::repository::MockRepositories;
    pub type Adapters = sos24_use_case::shared::adapter::MockAdapters;
}

pub struct Modules {
    config: Config,
    repositories: Arc<modules::Repositories>,
    form_use_case: FormUseCase<modules::Repositories, modules::Adapters>,
    form_answer_use_case: FormAnswerUseCase<modules::Repositories>,
    invitation_use_case: InvitationUseCase<modules::Repositories>,
    news_use_case: NewsUseCase<modules::Repositories, modules::Adapters>,
    file_use_case: FileUseCase<modules::Repositories>,
    project_use_case: ProjectUseCase<modules::Repositories, modules::Adapters>,
    user_use_case: UserUseCase<modules::Repositories>,
}

impl Modules {
    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn repositories(&self) -> &Arc<modules::Repositories> {
        &self.repositories
    }

    pub fn form_use_case(&self) -> &FormUseCase<modules::Repositories, modules::Adapters> {
        &self.form_use_case
    }

    pub fn form_answer_use_case(&self) -> &FormAnswerUseCase<modules::Repositories> {
        &self.form_answer_use_case
    }

    pub fn invitation_use_case(&self) -> &InvitationUseCase<modules::Repositories> {
        &self.invitation_use_case
    }

    pub fn news_use_case(&self) -> &NewsUseCase<modules::Repositories, modules::Adapters> {
        &self.news_use_case
    }

    pub fn file_use_case(&self) -> &FileUseCase<modules::Repositories> {
        &self.file_use_case
    }

    pub fn project_use_case(&self) -> &ProjectUseCase<modules::Repositories, modules::Adapters> {
        &self.project_use_case
    }

    pub fn user_use_case(&self) -> &UserUseCase<modules::Repositories> {
        &self.user_use_case
    }
}

#[cfg(not(test))]
pub async fn new(config: Config) -> anyhow::Result<Modules> {
    use sos24_infrastructure::shared::{
        firebase::FirebaseAuth, mongodb::MongoDb, postgresql::Postgresql, s3::S3,
        sendgrid::SendGrid,
    };

    use crate::env;

    let db = Postgresql::new(&env::postgres_db_url()).await?;
    let mongo_db = MongoDb::new(&env::mongodb_db_url(), &env::mongodb_db_name()).await?;
    let auth = FirebaseAuth::new(&env::firebase_service_account_key()).await?;
    let object_storage = S3::new(
        &env::s3_endpoint(),
        &env::s3_region(),
        &env::s3_access_key_id(),
        &env::s3_secret_access_key(),
    )
    .await;
    let repositories = Arc::new(sos24_infrastructure::DefaultRepositories::new(
        db,
        mongo_db,
        auth,
        object_storage,
    ));

    let send_grid = SendGrid::new(env::send_grid_api_key());
    let adapters = Arc::new(sos24_infrastructure::DefaultAdapters::new(
        send_grid,
        env::slack_webhook_url(),
    ));

    let application_period = ProjectApplicationPeriod::new(
        config.project_application_start_at.clone(),
        config.project_application_end_at.clone(),
    );

    Ok(Modules {
        config,
        repositories: Arc::clone(&repositories),
        form_use_case: FormUseCase::new(Arc::clone(&repositories), Arc::clone(&adapters)),
        form_answer_use_case: FormAnswerUseCase::new(Arc::clone(&repositories)),
        invitation_use_case: InvitationUseCase::new(
            Arc::clone(&repositories),
            application_period.clone(),
        ),
        news_use_case: NewsUseCase::new(Arc::clone(&repositories), Arc::clone(&adapters)),
        file_use_case: FileUseCase::new(Arc::clone(&repositories)),
        project_use_case: ProjectUseCase::new(
            Arc::clone(&repositories),
            Arc::clone(&adapters),
            application_period,
        ),
        user_use_case: UserUseCase::new(Arc::clone(&repositories)),
    })
}

#[cfg(test)]
pub async fn new_test(
    repositories: sos24_domain::test::repository::MockRepositories,
    adapters: sos24_use_case::shared::adapter::MockAdapters,
) -> anyhow::Result<Modules> {
    let repositories = Arc::new(repositories);
    let adapters = Arc::new(adapters);

    let application_period = ProjectApplicationPeriod::default();

    Ok(Modules {
        config: Config::default(),
        repositories: Arc::clone(&repositories),
        form_use_case: FormUseCase::new(Arc::clone(&repositories), Arc::clone(&adapters)),
        form_answer_use_case: FormAnswerUseCase::new(Arc::clone(&repositories)),
        invitation_use_case: InvitationUseCase::new(
            Arc::clone(&repositories),
            application_period.clone(),
        ),
        news_use_case: NewsUseCase::new(Arc::clone(&repositories), Arc::clone(&adapters)),
        file_use_case: FileUseCase::new(Arc::clone(&repositories)),
        project_use_case: ProjectUseCase::new(
            Arc::clone(&repositories),
            Arc::clone(&adapters),
            application_period,
        ),
        user_use_case: UserUseCase::new(Arc::clone(&repositories)),
    })
}
