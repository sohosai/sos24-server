use std::env;

use anyhow::Result;
use sqlx::postgres::PgPoolOptions;

mod handlers;

const DEFAULT_HOST: &str = "localhost";
const DEFAULT_PORT: usize = 3000;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    if let Err(_) = dotenvy::dotenv() {
        tracing::info!(".env file doesn't exist. skipped.");
    }

    // DB
    let database_url = env::var("DATABASE_URL").expect("env `DATABASE_URL` must be set");
    println!("{database_url}");

    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&database_url)
        .await
        .expect("Couldn't connect to the DB");

    // API
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
    let app = handlers::create_app(pool);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Listening on http://{addr}/health");
    axum::serve(listener, app).await?;

    Ok(())
}
