use std::sync::Arc;

use axum::http::{header, Method};
use axum::{
    extract::DefaultBodyLimit,
    routing::{delete, get, post, put},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::module::Modules;

use self::shared::auth;

pub mod file;
pub mod form;
pub mod form_answer;
pub mod health;
pub mod invitation;
pub mod news;
pub mod project;
pub mod project_application_period;
pub mod shared;
pub mod user;

pub fn create_app(modules: Modules) -> Router {
    let modules = Arc::new(modules);

    let news = Router::new()
        .route("/", get(news::get::handle))
        .route("/", post(news::post::handle))
        .route("/:news_id", get(news::get_by_id::handle))
        .route("/:news_id", delete(news::delete_by_id::handle))
        .route("/:news_id", put(news::put_by_id::handle));

    let file = Router::new()
        .route("/", post(file::post::handle))
        .layer(DefaultBodyLimit::max(modules.config().file_upload_limit))
        .route("/", get(file::get::handle))
        .route("/export", get(file::export::handle))
        .route("/:file_id", get(file::get_by_id::handle))
        .route("/:file_id", delete(file::delete_by_id::handle));

    let user = Router::new()
        .route("/", get(user::get::handle))
        .route("/export", get(user::export::handle))
        .route("/me", get(user::get_me::handle))
        .route("/:user_id", get(user::get_by_id::handle))
        .route("/:user_id", delete(user::delete_by_id::handle))
        .route("/:user_id", put(user::put_by_id::handle));

    let project = Router::new()
        .route("/", get(project::get::handle))
        .route("/", post(project::post::handle))
        .route("/export", get(project::export::handle))
        .route("/me", get(project::get_me::handle))
        .route("/:project_id", get(project::get_by_id::handle))
        .route("/:project_id", delete(project::delete_by_id::handle))
        .route("/:project_id", put(project::put_by_id::handle));

    let invitation = Router::new()
        .route("/", get(invitation::get::handle))
        .route("/", post(invitation::post::handle))
        .route("/:invitation_id", get(invitation::get_by_id::handle))
        .route("/:invitation_id", delete(invitation::delete_by_id::handle))
        .route("/:invitation_id", post(invitation::post_by_id::handle));

    let form = Router::new()
        .route("/", get(form::get::handle))
        .route("/", post(form::post::handle))
        .route("/:form_id", get(form::get_by_id::handle))
        .route("/:form_id", delete(form::delete_by_id::handle))
        .route("/:form_id", put(form::put_by_id::handle));

    let form_answers = Router::new()
        .route("/", get(form_answer::get::handle))
        .route("/", post(form_answer::post::handle))
        .route("/export", get(form_answer::export::handle))
        .route("/:form_answer_id", get(form_answer::get_by_id::handle))
        .route("/:form_answer_id", put(form_answer::put_by_id::handle));

    let project_application_period =
        Router::new().route("/", get(project_application_period::get::handle));

    let private_routes = Router::new()
        .nest("/news", news)
        .nest("/files", file)
        .nest("/users", user)
        .nest("/projects", project)
        .nest("/invitations", invitation)
        .nest("/forms", form)
        .nest("/form-answers", form_answers)
        .nest("/project-application-period", project_application_period)
        .route_layer(axum::middleware::from_fn_with_state(
            Arc::clone(&modules),
            auth::jwt_auth,
        ));

    let public_routes = Router::new()
        .route("/health", get(health::handle_get))
        .route("/users", post(user::post::handle));

    Router::new()
        .nest("/", public_routes)
        .nest("/", private_routes)
        .with_state(Arc::clone(&modules))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(
            CorsLayer::new()
                .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
                .expose_headers([header::CONTENT_DISPOSITION])
                .allow_methods([Method::GET, Method::PUT, Method::POST, Method::DELETE])
                .allow_origin(Any),
        )
}
