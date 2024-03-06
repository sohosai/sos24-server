use std::sync::Arc;

use axum::{routing::get, Router};

use crate::module::Modules;

pub mod health;

pub fn create_app(modules: Modules) -> Router {
    Router::new()
        .route("/health", get(health::handle_get))
        .with_state(Arc::new(modules))
}
