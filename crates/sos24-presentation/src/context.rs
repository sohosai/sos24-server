use sos24_use_case::context::{self, ContextProvider};

use crate::config::Config;

#[derive(Clone)]
pub struct Context {
    user_id: String,
    requested_at: chrono::DateTime<chrono::Utc>,
    config: context::Config,
}

impl From<Config> for context::Config {
    fn from(config: Config) -> Self {
        context::Config {
            email_sender_address: config.email_sender_address,
            email_reply_to_address: config.email_reply_to_address,
            app_url: config.app_url,
        }
    }
}

impl Context {
    pub fn new(user_id: String, config: context::Config) -> Self {
        Self {
            user_id,
            requested_at: chrono::Utc::now(),
            config,
        }
    }

    pub fn new_system(config: context::Config) -> Self {
        Self {
            user_id: String::from("system"), // FIXME
            requested_at: chrono::Utc::now(),
            config,
        }
    }
}

impl ContextProvider for Context {
    fn user_id(&self) -> String {
        self.user_id.clone()
    }

    fn requested_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.requested_at
    }

    fn config(&self) -> &context::Config {
        &self.config
    }
}
