use std::env;

use anyhow::Result;
use axum::{middleware, routing::get, Router};
use mongodb::Database;
use sqlx::PgPool;

mod auth;
mod handlers;
mod repository;

const DEFAULT_HOST: &str = "localhost";
const DEFAULT_PORT: usize = 3000;

pub fn create_app(pool: PgPool, mongo_db: Database) -> Router {
    Router::new()
        .route("/health", get(handlers::health::handle_get))
        .merge(
            Router::new()
                .route("/private", get(|| async { "YOU'RE AUTHORIZED!" }))
                .route_layer(middleware::from_fn(auth::jwt_auth)),
        )
        .with_state(pool)
        .with_state(mongo_db)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    if let Err(e) = dotenvy::dotenv() {
        tracing::info!(".env file doesn't exist. skipped: {e}");
    }

    // DB
    let pg_pool = repository::get_pg_pool().await?;

    // MongoDB
    let mongo_db = repository::get_mongo_db().await?;

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
    let app = create_app(pg_pool, mongo_db);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Listening on http://{addr}/health");
    axum::serve(listener, app).await?;

    Ok(())
}
