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
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::{middleware::auth, module::Modules};

pub mod file;
pub mod form;
pub mod form_answer;
pub mod health;
pub mod invitation;
pub mod news;
pub mod project;
pub mod project_application_period;
pub mod user;

pub fn create_app(modules: Arc<Modules>) -> Router {
    let news = Router::new()
        .route("/", get(news::handle_get))
        .route("/", post(news::handle_post))
        .route("/:news_id", get(news::handle_get_id))
        .route("/:news_id", delete(news::handle_delete_id))
        .route("/:news_id", put(news::handle_put_id));

    let file = Router::new()
        .route("/", post(file::handle_post))
        .layer(DefaultBodyLimit::max(modules.config().file_upload_limit))
        .route("/", get(file::handle_get))
        .route("/export", get(file::handle_export))
        .route("/:file_id", get(file::handle_get_id))
        .route("/:file_id", delete(file::handle_delete_id));

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
        .route("/export", get(form_answer::handle_export))
        .route("/:form_answer_id", get(form_answer::handle_get_id))
        .route("/:form_answer_id", put(form_answer::handle_put_id));

    let private_routes = Router::new()
        .nest("/news", news)
        .nest("/files", file)
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
        .route("/users", post(user::handle_post))
        .route(
            "/project-application-period",
            get(project_application_period::handle_get),
        );

    Router::new()
        .nest("/", public_routes)
        .nest("/", private_routes)
        .with_state(modules)
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

use crate::{error, model, route};
#[derive(OpenApi)]
#[openapi(
    info(title = "Sohosai Online System"),
    servers((url = "https://api.sos24.sohosai.com")),
    tags(
        (name = "projects", description = "企画関連の操作"),
        (name = "users", description = "ユーザー関連の操作"),
        (name = "news", description = "お知らせ関連の操作"),
        (name = "files", description = "ファイル関連の操作"),
        (name = "forms", description = "申請関連の操作"),
        (name = "form-answers", description = "申請回答関連の操作"),
        (name = "invitations", description = "招待関連の操作"),
        (name = "meta", description = "状態確認関連の操作"),
    ),
    paths(
        route::file::handle_get,
        route::file::handle_post,
        route::file::handle_export,
        route::file::handle_get_id,
        route::file::handle_delete_id,
        route::form::handle_get,
        route::form::handle_post,
        route::form::handle_get_id,
        route::form::handle_put_id,
        route::form::handle_delete_id,
        route::form_answer::handle_get,
        route::form_answer::handle_post,
        route::form_answer::handle_export,
        route::form_answer::handle_get_id,
        route::form_answer::handle_put_id,
        route::health::handle_get,
        route::invitation::handle_get,
        route::invitation::handle_post,
        route::invitation::handle_get_id,
        route::invitation::handle_post_id,
        route::invitation::handle_delete_id,
        route::news::handle_get,
        route::news::handle_post,
        route::news::handle_get_id,
        route::news::handle_delete_id,
        route::news::handle_put_id,
        route::project::handle_get,
        route::project::handle_post,
        route::project::handle_export,
        route::project::handle_get_me,
        route::project::handle_get_id,
        route::project::handle_delete_id,
        route::project::handle_put_id,
        route::project_application_period::handle_get,
        route::user::handle_get,
        route::user::handle_post,
        route::user::handle_export,
        route::user::handle_get_me,
        route::user::handle_get_id,
        route::user::handle_delete_id,
        route::user::handle_put_id,
    ),
    components(schemas(
        model::file::CreatedFile,
        model::file::File,
        model::file::FileInfo,
        model::form::CreateForm,
        model::form::CreatedForm,
        model::form::UpdateForm,
        model::form::NewFormItem,
        model::form::Form,
        model::form::FormItem,
        model::form::FormItemKind,
        model::form::FormSummary,
        model::form_answer::CreateFormAnswer,
        model::form_answer::CreatedFormAnswer,
        model::form_answer::UpdateFormAnswer,
        model::form_answer::FormAnswer,
        model::form_answer::FormAnswerItem,
        model::form_answer::FormAnswerSummary,
        model::invitation::CreateInvitation,
        model::invitation::CreatedInvitation,
        model::invitation::Invitation,
        model::invitation::InvitationPosition,
        model::news::CreateNews,
        model::news::CreatedNews,
        model::news::UpdateNews,
        model::news::News,
        model::news::NewsSummary,
        model::project::CreateProject,
        model::project::CreatedProject,
        model::project::UpdateProject,
        model::project::Project,
        model::project::ProjectCategory,
        model::project::ProjectAttribute,
        model::project::ProjectSummary,
        model::project_application_period::ProjectApplicationPeriod,
        model::user::CreateUser,
        model::user::CreatedUser,
        model::user::UpdateUser,
        model::user::User,
        model::user::UserRole,
        model::user::UserSummary,
        error::ErrorResponse,
    )),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let Some(schema) = openapi.components.as_mut() else {
            return;
        };
        schema.add_security_scheme(
            "jwt_token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use utoipa::OpenApi;

    use super::ApiDoc;

    #[test]
    fn test_openapi_document() {
        let current_schema = include_str!("../../../schema.yml");
        let new_schema = ApiDoc::openapi().to_yaml().unwrap();
        if current_schema != new_schema {
            panic!("APIエンドポイントが更新されています。\nターミナルで cargo run --bin gen-openapi > schema.yml を実行し、スキーマを更新してください。")
        }
    }
}
