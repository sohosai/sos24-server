use std::env;

pub fn host() -> String {
    env::var("HOST").unwrap_or({
        tracing::debug!(
            "The HOST environment variable is not set. Using the default value instead."
        );
        "127.0.0.1".to_string()
    })
}

pub fn port() -> String {
    env::var("PORT").unwrap_or({
        tracing::debug!(
            "The PORT environment variable is not set. Using the default value instead."
        );
        "3000".to_string()
    })
}

pub fn postgres_db_url() -> String {
    env::var("POSTGRES_DB_URL").expect("Env `POSTGRES_DB_URL` must be set")
}

pub fn firebase_service_account_key() -> String {
    env::var("FIREBASE_SERVICE_ACCOUNT_KEY")
        .expect("Env `FIREBASE_SERVICE_ACCOUNT_KEY` must be set")
}

pub fn firebase_project_id() -> String {
    env::var("FIREBASE_PROJECT_ID").expect("Env `FIREBASE_PROJECT_ID` must be set")
}
