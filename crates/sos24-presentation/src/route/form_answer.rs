use std::sync::Arc;

use axum::body::Body;
use axum::response::Response;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use percent_encoding::NON_ALPHANUMERIC;
use sos24_use_case::form_answer::interactor::create::CreateFormAnswerCommand;

use crate::context::Context;
use crate::csv::{serialize_to_csv, CsvSerializationError};
use crate::model::form_answer::{
    CreatedFormAnswer, ExportFormAnswerQuery, FormAnswerSummary, UpdateFormAnswer,
};
use crate::{
    error::AppError,
    model::form_answer::{
        ConvertToUpdateFormAnswerDto, CreateFormAnswer, FormAnswer, FormAnswerQuery,
    },
    module::Modules,
};

/// 申請回答一覧を取得
#[utoipa::path(
    get,
    path = "/form-answers",
    operation_id = "getFormAnswers",
    tag = "form-answers",
    params(FormAnswerQuery),
    responses(
        (status = 200, description = "OK", body = Vec<FormAnswerSummary>),
        (status = 401, description = "Unauthorized", body = Error),
        (status = 403, description = "Forbidden", body = Error),
        (status = 500, description = "Internal Server Error", body = Error),
    ),
    security(("jwt_token" = [])),
)]
pub async fn handle_get(
    State(modules): State<Arc<Modules>>,
    Query(query): Query<FormAnswerQuery>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let raw_form_answer_list = match (query.project_id, query.form_id) {
        (None, None) => modules.form_answer_use_case().list(&ctx).await,
        (Some(project_id), None) => {
            modules
                .form_answer_use_case()
                .find_by_project_id(&ctx, project_id)
                .await
        }
        (None, Some(form_id)) => {
            modules
                .form_answer_use_case()
                .find_by_form_id(&ctx, form_id)
                .await
        }
        _ => {
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                "form-answer/invalid-query".to_string(),
                "Invalid query".to_string(),
            ));
        }
    };

    raw_form_answer_list
        .map(|raw_form_answer_list| {
            let form_answer_list: Vec<FormAnswerSummary> = raw_form_answer_list
                .into_iter()
                .map(FormAnswerSummary::from)
                .collect();
            (StatusCode::OK, Json(form_answer_list))
        })
        .map_err(|err| {
            tracing::error!("Failed to list form answer: {err:?}");
            err.into()
        })
}

/// 申請回答を作成
#[utoipa::path(
    post,
    path = "/form-answers",
    operation_id = "postFormAnswer",
    tag = "form-answers",
    request_body(
        content = CreateFormAnswer,
    ),
    responses(
        (status = 201, description = "Created", body = CreatedFormAnswer),
        (status = 400, description = "Bad Request", body = Error),
        (status = 401, description = "Unauthorized", body = Error),
        (status = 403, description = "Forbidden", body = Error),
        (status = 422, description = "Unprocessable Entity", body = Error),
        (status = 500, description = "Internal Server Error", body = Error),
    ),
    security(("jwt_token" = [])),
)]
pub async fn handle_post(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(raw_form_answer): Json<CreateFormAnswer>,
) -> Result<impl IntoResponse, AppError> {
    let form_answer = CreateFormAnswerCommand::from(raw_form_answer);
    let res = modules
        .form_answer_use_case()
        .create(&ctx, form_answer)
        .await;
    res.map(|id| (StatusCode::CREATED, Json(CreatedFormAnswer { id })))
        .map_err(|err| {
            tracing::error!("Failed to create form answer: {err:?}");
            err.into()
        })
}

/// 申請回答一覧のエクスポート
#[utoipa::path(
    get,
    path = "/form-answers/export",
    operation_id = "getFormAnswersExport",
    tag = "form-answers",
    params(ExportFormAnswerQuery),
    responses(
        (status = 200, description = "OK", content_type = "text/csv", body = String),
        (status = 401, description = "Unauthorized", body = Error),
        (status = 403, description = "Forbidden", body = Error),
        (status = 500, description = "Internal Server Error", body = Error),
    ),
    security(("jwt_token" = [])),
)]
pub async fn handle_export(
    State(modules): State<Arc<Modules>>,
    Query(query): Query<ExportFormAnswerQuery>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let Some(form_id) = query.form_id else {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "form-answer/invalid-query".to_string(),
            "Invalid query".to_string(),
        ));
    };

    let form_answer_list = modules
        .form_answer_use_case()
        .export_by_form_id(&ctx, form_id)
        .await
        .map_err(|err| {
            tracing::error!("Failed to export form answer: {err:?}");
            AppError::from(err)
        })?;

    let data = (|| -> Result<String, CsvSerializationError> {
        let mut csv_data = vec![];

        let header: Vec<String> = ["企画番号", "企画名", "企画団体名", "回答日時"]
            .into_iter()
            .map(ToString::to_string)
            .chain(form_answer_list.form_item_names.into_iter())
            .collect();
        csv_data.push(header);

        for form_answer in form_answer_list.form_answers {
            let record = [
                form_answer.project_index.to_string(),
                form_answer.project_title,
                form_answer.project_group_name,
                form_answer.created_at,
            ]
            .into_iter()
            .chain(
                form_answer
                    .form_answer_item_values
                    .into_iter()
                    .map(|it| it.unwrap_or_default()),
            )
            .collect();
            csv_data.push(record);
        }

        serialize_to_csv(csv_data)
    })()
    .map_err(|err| {
        tracing::error!("Failed to serialize to csv: {err:?}");
        AppError::from(err)
    })?;

    let filename = format!("{}_回答一覧.csv", form_answer_list.form_title);
    let encoded_filename = percent_encoding::percent_encode(filename.as_bytes(), NON_ALPHANUMERIC);
    Response::builder()
        .header("Content-Type", "text/csv")
        .header(
            "Content-Disposition",
            format!(
                "attachment; filename=\"{}\" filename*=UTF-8''{}",
                filename, encoded_filename
            ),
        )
        .body(Body::from(data))
        .map_err(|err| {
            tracing::error!("Failed to create response: {err:?}");
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "csv/failed-to-create-response".to_string(),
                format!("{err:?}"),
            )
        })
}

/// 特定のIDの申請回答を取得
#[utoipa::path(
    get,
    path = "/form-answers/{form_answer_id}",
    operation_id = "getFormAnswerById",
    tag = "form-answers",
    params(("form_answer_id" = String, Path, format="uuid")),
    responses(
        (status = 200, description = "OK", body = FormAnswer),
        (status = 401, description = "Unauthorized", body = Error),
        (status = 403, description = "Forbidden", body = Error),
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
    let raw_form = modules.form_answer_use_case().find_by_id(&ctx, id).await;
    match raw_form {
        Ok(raw_form) => Ok((StatusCode::OK, Json(FormAnswer::from(raw_form)))),
        Err(err) => {
            tracing::error!("Failed to find form answer by id: {err:?}");
            Err(err.into())
        }
    }
}

/// 特定のIDの申請回答を更新
#[utoipa::path(
    put,
    path = "/form-answers/{form_answer_id}",
    operation_id = "putFormAnswerById",
    tag = "form-answers",
    params(("form_answer_id" = String, Path, format="uuid")),
    request_body(content = UpdateFormAnswer),
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
    Json(raw_form_answer): Json<UpdateFormAnswer>,
) -> Result<impl IntoResponse, AppError> {
    let form = (raw_form_answer, id).to_update_form_answer_dto();
    let res = modules.form_answer_use_case().update(&ctx, form).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to update form: {err:?}");
        err.into()
    })
}
