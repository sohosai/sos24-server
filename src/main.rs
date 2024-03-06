use std::env;

use anyhow::Result;
use axum::routing::{delete, put};
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
    mongo_db: Database,
}

#[derive(Debug, Clone)]
struct Config {
    firebase_service_account_key: String,
    firebase_project_id: String,
}

fn get_config() -> Config {
    let firebase_project_id = std::env::var("FIREBASE_PROJECT_ID")
        .expect("The FIREBASE_PROJECT_ID environment variable is not set.");
    if firebase_project_id.is_empty() {
        panic!("The FIREBASE_PROJECT_ID environment variable must not be empty.");
    }

    let firebase_service_account_key = env::var("FIREBASE_SERVICE_ACCOUNT_KEY")
        .expect("env `FIREBASE_SERVICE_ACCOUNT_KEY` must be set");
    if firebase_service_account_key.is_empty() {
        panic!("env `FIREBASE_SERVICE_ACCOUNT_KEY` must not be empty");
    }

    Config {
        firebase_project_id,
        firebase_service_account_key,
    }
}

pub fn create_app(pool: PgPool, mongo_db: Database) -> Router {
    let app_state = AppState {
        config: get_config(),
        pool,
        mongo_db,
    };

    Router::new()
        // private
        .merge(
            Router::new()
                .route("/private", get(handlers::private::handle_get))
                .route_layer(middleware::from_fn_with_state(
                    app_state.clone(),
                    auth::jwt_auth,
                ))
                .route("/news", get(handlers::news::handle_get_news))
                .route_layer(middleware::from_fn_with_state(
                    app_state.clone(),
                    auth::jwt_auth,
                ))
                .route("/news", post(handlers::news::handle_post_news))
                .route_layer(middleware::from_fn_with_state(
                    app_state.clone(),
                    auth::jwt_auth,
                ))
                .route("/news/:news_id", get(handlers::news::handle_get_news_by_id))
                .route_layer(middleware::from_fn_with_state(
                    app_state.clone(),
                    auth::jwt_auth,
                ))
                .route(
                    "/news/:news_id",
                    delete(handlers::news::handle_delete_news_by_id),
                )
                .route_layer(middleware::from_fn_with_state(
                    app_state.clone(),
                    auth::jwt_auth,
                ))
                .route("/news/:news_id", put(handlers::news::handle_put_news_by_id))
                .route_layer(middleware::from_fn_with_state(
                    app_state.clone(),
                    auth::jwt_auth,
                )),
        )
        // public
        .merge(
            Router::new()
                .route("/health", get(handlers::health::handle_get))
                .route("/users", post(handlers::users::handle_post_users)),
        )
        .with_state(app_state.clone())
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
