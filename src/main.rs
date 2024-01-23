use std::env;

use anyhow::Result;

mod handlers;

const DEFAULT_HOST: &str = "localhost";
const DEFAULT_PORT: usize = 3000;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let host = env::var("HOST").unwrap_or({
        tracing::debug!(
            "The HOST environment variable is not set. Using the default value instead."
        );
        DEFAULT_HOST.to_string()
    });
    let port = env::var("PORT").unwrap_or({
        tracing::debug!(
            "The PORT environment variable is not set. Using the default value instead."
        );
        DEFAULT_PORT.to_string()
    });

    let addr = format!("{host}:{port}");
    let app = handlers::create_app();
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Listening on http://{addr}/health");
    axum::serve(listener, app).await?;

    Ok(())
}
