use axum::{routing::get, Router};
use sqlx::PgPool;

mod health;

pub fn create_app(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health::handle_get))
        .with_state(pool)
}
