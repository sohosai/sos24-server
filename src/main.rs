use std::env;

use anyhow::Result;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use mongodb::Database;
use sqlx::PgPool;

mod auth;
mod handlers;
mod repository;

const DEFAULT_HOST: &str = "localhost";
const DEFAULT_PORT: usize = 3000;

#[derive(Debug, Clone)]
struct AppState {
    config: Config,
    pool: PgPool,
}

#[derive(Debug, Clone)]
struct Config {
    firebase_admin_api_key: String,
}

pub fn create_app(pool: PgPool, mongo_db: Database) -> Router {
    let firebase_admin_api_key =
        env::var("FIREBASE_ADMIN_API_KEY").expect("env `FIREBASE_ADMIN_API_KEY` must be set");
    if firebase_admin_api_key.is_empty() {
        panic!("env `FIREBASE_ADMIN_API_KEY` must not be empty");
    }

    Router::new()
        .route("/health", get(handlers::health::handle_get))
        .merge(Router::new().route("/users", post(handlers::users::handle_post_users)))
        .route_layer(middleware::from_fn(auth::jwt_auth))
        .with_state(AppState {
            pool,
            config: Config {
                firebase_admin_api_key,
            },
        })
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
