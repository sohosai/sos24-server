use std::sync::Arc;

use axum::extract::Query;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};

use sos24_use_case::dto::form::CreateFormDto;

use crate::context::Context;
use crate::model::form::{CreatedForm, Form, FormQuery, FormSummary};
use crate::{
    error::AppError,
    model::form::{ConvertToUpdateFormDto, UpdateForm},
};
use crate::{model::form::CreateForm, module::Modules};

/// 申請一覧の取得
#[utoipa::path(
    get,
    path = "/forms",
    operation_id = "getForms",
    tag = "forms",
    params(FormQuery),
    responses(
        (status = 200, description = "OK", body = Vec<FormSummary>),
        (status = 401, description = "Unauthorized", body = Error),
        (status = 403, description = "Forbidden", body = Error),
        (status = 404, description = "Not Found", body = Error),
        (status = 500, description = "Internal Server Error", body = Error),
    ),
    security(("jwt_token" = [])),
)]
pub async fn handle_get(
    Query(query): Query<FormQuery>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let raw_form_list = match query.project_id {
        Some(project_id) => {
            modules
                .form_use_case()
                .find_by_project_id(&ctx, project_id)
                .await
        }
        None => modules.form_use_case().list(&ctx).await,
    };
    raw_form_list
        .map(|raw_form_list| {
            let form_list: Vec<FormSummary> =
                raw_form_list.into_iter().map(FormSummary::from).collect();
            (StatusCode::OK, Json(form_list)).into_response()
        })
        .map_err(|err| {
            tracing::error!("Failed to find form by project id: {err:?}");
            err.into()
        })
}

/// 申請の作成
#[utoipa::path(
    post,
    path = "/forms",
    operation_id = "postForm",
    tag = "forms",
    request_body(content = CreateForm),
    responses(
        (status = 201, description = "Created", body = CreatedForm),
        (status = 400, description = "Bad Request", body = Error),
        (status = 401, description = "Unauthorized", body = Error),
        (status = 403, description = "Forbidden", body = Error),
        (status = 422, description = "Unprocessable Entity", body = Error),
        (status = 500, description = "Internal Server Error", body = Error),
    ),
    security(("jwt_token" = [])),
)]
pub async fn handle_post(
    State(module): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(raw_form): Json<CreateForm>,
) -> Result<impl IntoResponse, AppError> {
    let form = CreateFormDto::from(raw_form);
    let res = module.form_use_case().create(&ctx, form).await;
    res.map(|id| (StatusCode::CREATED, Json(CreatedForm { id })))
        .map_err(|err| {
            tracing::error!("Failed to create form: {err:?}");
            err.into()
        })
}

/// 特定のIDの申請を取得
#[utoipa::path(
    get,
    path = "/forms/{form_id}",
    operation_id = "getFormById",
    tag = "forms",
    params(("form_id" = String, Path, format="uuid")),
    responses(
        (status = 200, description = "OK", body = Form),
        (status = 401, description = "Unauthorized", body = Error),
        (status = 404, description = "Not Found", body = Error),
        (status = 500, description = "Internal Server Error", body = Error),
    ),
    security(("jwt_token" = [])),
)]
pub async fn handle_get_id(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let raw_form = modules.form_use_case().find_by_id(&ctx, id).await;
    match raw_form {
        Ok(raw_form) => Ok((StatusCode::OK, Json(Form::from(raw_form)))),
        Err(err) => {
            tracing::error!("Failed to find form by id: {err:?}");
            Err(err.into())
        }
    }
}

/// 特定のIDの申請を削除
#[utoipa::path(
    delete,
    path = "/forms/{form_id}",
    operation_id = "deleteFormById",
    tag = "forms",
    params(("form_id" = String, Path, format="uuid")),
    responses(
        (status = 200, description = "OK"),
        (status = 401, description = "Unauthorized", body = Error),
        (status = 403, description = "Forbidden", body = Error),
        (status = 404, description = "Not Found", body = Error),
        (status = 500, description = "Internal Server Error", body = Error),
    ),
    security(("jwt_token" = [])),
)]
pub async fn handle_delete_id(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.form_use_case().delete_by_id(&ctx, id).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to delete form: {err:?}");
        err.into()
    })
}

/// 特定のIDの申請を更新
#[utoipa::path(
    put,
    path = "/forms/{form_id}",
    operation_id = "putFormById",
    tag = "forms",
    params(("form_id" = String, Path, format="uuid")),
    request_body(content = UpdateForm),
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = Error),
        (status = 401, description = "Unauthorized", body = Error),
        (status = 403, description = "Forbidden", body = Error),
        (status = 404, description = "Not Found", body = Error),
        (status = 422, description = "Unprocessable Entity", body = Error),
        (status = 500, description = "Internal Server Error", body = Error),
    ),
    security(("jwt_token" = [])),
)]
pub async fn handle_put_id(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(raw_form): Json<UpdateForm>,
) -> Result<impl IntoResponse, AppError> {
    let form = (id, raw_form).to_update_form_dto();
    let res = modules.form_use_case().update(&ctx, form).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to update form: {err:?}");
        err.into()
    })
}
