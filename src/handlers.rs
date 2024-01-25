use axum::{routing::get, Router};
use mongodb::Database;
use sqlx::PgPool;

mod health;

pub fn create_app(pool: PgPool, mongo_db: Database) -> Router {
    Router::new()
        .route("/health", get(health::handle_get))
        .with_state(pool)
        .with_state(mongo_db)
}
