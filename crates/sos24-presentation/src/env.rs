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
