use std::env;

use anyhow::Result;
use mongodb::{options::ClientOptions, Client};
use sqlx::postgres::PgPoolOptions;

mod handlers;

const DEFAULT_HOST: &str = "localhost";
const DEFAULT_PORT: usize = 3000;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    if let Err(e) = dotenvy::dotenv() {
        tracing::info!(".env file doesn't exist. skipped: {e}");
    }

    // DB
    let pg_db_url = env::var("POSTGRES_DB_URL").expect("env `POSTGRES_DB_URL` must be set");

    let pg_pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&pg_db_url)
        .await
        .expect("Couldn't connect to the DB");

    // MongoDB
    let mongo_database_url = env::var("MONGO_DB_URL").expect("env `MONGO_DB_URL` doesn't set");
    let client_options = ClientOptions::parse(mongo_database_url).await?;
    let client = Client::with_options(client_options)?;

    let mongo_db_name = env::var("MONGO_DB").expect("env `MONGO_DB` must be set");
    let mongo_db = client.database(&mongo_db_name);

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
    let app = handlers::create_app(pg_pool, mongo_db);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Listening on http://{addr}/health");
    axum::serve(listener, app).await?;

    Ok(())
}
