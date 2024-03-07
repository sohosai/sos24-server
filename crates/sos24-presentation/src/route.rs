use std::sync::Arc;

use axum::{
    http::StatusCode,
    routing::{delete, get, post, put},
    Router,
};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::module::Modules;

pub mod health;
pub mod news;
pub mod user;

pub trait ToStatusCode {
    fn status_code(&self) -> StatusCode;
}

pub fn create_app(modules: Modules) -> Router {
    let news = Router::new()
        .route("/", get(news::handle_get))
        .route("/", post(news::handle_post))
        .route("/:news_id", get(news::handle_get_id))
        .route("/:news_id", delete(news::handle_delete_id))
        .route("/:news_id", put(news::handle_put_id));

    let user = Router::new().route("", post(user::handle_post));

    Router::new()
        .route("/health", get(health::handle_get))
        .nest("/news", news)
        .nest("/users", user)
        .with_state(Arc::new(modules))
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO)),
            ),
        )
}
