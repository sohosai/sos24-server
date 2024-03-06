use std::sync::Arc;

use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::module::Modules;

pub mod health;
pub mod news;

pub fn create_app(modules: Modules) -> Router {
    let news = Router::new()
        .route("/", get(news::handle_get))
        .route("/", post(news::handle_post))
        .route("/:news_id", get(news::handle_get_id))
        .route("/:news_id", delete(news::handle_delete_id))
        .route("/:news_id", put(news::handle_put_id));

    Router::new()
        .route("/health", get(health::handle_get))
        .nest("/news", news)
        .with_state(Arc::new(modules))
}
