use std::sync::Arc;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::{middleware::auth, module::Modules};

pub mod form;
pub mod form_answer;
pub mod health;
pub mod invitation;
pub mod news;
pub mod news_attachment;
pub mod project;
pub mod user;

pub fn create_app(modules: Modules) -> Router {
    let modules = Arc::new(modules);

    let news = Router::new()
        .route("/", get(news::handle_get))
        .route("/", post(news::handle_post))
        .route("/:news_id", get(news::handle_get_id))
        .route("/:news_id", delete(news::handle_delete_id))
        .route("/:news_id", put(news::handle_put_id));

    let news_attachment = Router::new()
        .route("/", get(news_attachment::handle_get))
        .route("/", post(news_attachment::handle_post))
        .route("/:news_attachment_id", get(news_attachment::handle_get_id))
        .route(
            "/:news_attachment_id",
            delete(news_attachment::handle_delete_id),
        );

    let user = Router::new()
        .route("/", get(user::handle_get))
        .route("/export", get(user::handle_export))
        .route("/me", get(user::handle_get_me))
        .route("/:user_id", get(user::handle_get_id))
        .route("/:user_id", delete(user::handle_delete_id))
        .route("/:user_id", put(user::handle_put_id));

    let project = Router::new()
        .route("/", get(project::handle_get))
        .route("/", post(project::handle_post))
        .route("/export", get(project::handle_export))
        .route("/me", get(project::handle_get_me))
        .route("/:project_id", get(project::handle_get_id))
        .route("/:project_id", delete(project::handle_delete_id))
        .route("/:project_id", put(project::handle_put_id));

    let invitation = Router::new()
        .route("/", get(invitation::handle_get))
        .route("/", post(invitation::handle_post))
        .route("/:invitation_id", get(invitation::handle_get_id))
        .route("/:invitation_id", delete(invitation::handle_delete_id))
        .route("/:invitation_id", post(invitation::handle_post_id));

    let form = Router::new()
        .route("/", get(form::handle_get))
        .route("/", post(form::handle_post))
        .route("/:form_id", get(form::handle_get_id))
        .route("/:form_id", delete(form::handle_delete_id))
        .route("/:form_id", put(form::handle_put_id));

    let form_answers = Router::new()
        .route("/", get(form_answer::handle_get))
        .route("/", post(form_answer::handle_post))
        .route("/:form_answer_id", get(form_answer::handle_get_id))
        .route("/:form_answer_id", put(form_answer::handle_put_id));

    let private_routes = Router::new()
        .nest("/news", news)
        .nest("/news_attachments", news_attachment)
        .nest("/users", user)
        .nest("/projects", project)
        .nest("/invitations", invitation)
        .nest("/forms", form)
        .nest("/form-answers", form_answers)
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
