use std::sync::Arc;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::{middleware::auth, module::Modules};

pub mod health;
pub mod news;
pub mod user;

pub fn create_app(modules: Modules) -> Router {
    let modules = Arc::new(modules);

    let news = Router::new()
        .route("/", get(news::handle_get))
        .route("/", post(news::handle_post))
        .route("/:news_id", get(news::handle_get_id))
        .route("/:news_id", delete(news::handle_delete_id))
        .route("/:news_id", put(news::handle_put_id));

    let user = Router::new()
        .route("/", get(user::handle_get))
        .route("/:user_id", get(user::handle_get_id))
        .route("/:user_id", delete(user::handle_delete_id))
        .route("/:user_id", put(user::handle_put_id));

    let private_routes = Router::new()
        .nest("/news", news)
        .nest("/users", user)
        .route_layer(axum::middleware::from_fn_with_state(
            Arc::clone(&modules),
            auth::jwt_auth,
        ));

    let public_routes = Router::new()
        .route("/health", get(health::handle_get))
        .route("/users", post(user::handle_post));

    Router::new()
        .nest("/", public_routes)
        .nest("/", private_routes)
        .with_state(Arc::clone(&modules))
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO)),
            ),
        )
}
