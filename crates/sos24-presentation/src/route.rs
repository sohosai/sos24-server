use axum::{routing::get, Router};

pub mod health;

pub fn create_app() -> Router {
    Router::new().route("/health", get(health::handle_get))
}
