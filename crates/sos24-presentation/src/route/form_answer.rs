use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use sos24_use_case::{context::Context, dto::form_answer::CreateFormAnswerDto};

use crate::{
    error::AppError,
    model::{
        form::{ConvertToUpdateFormDto, UpdateForm},
        form_answer::{CreateFormAnswer, FormAnswer, FormAnswerQuery},
    },
    module::Modules,
};

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
            ))
        }
    };

    raw_form_answer_list
        .map(|raw_form_answer_list| {
            let form_answer_list: Vec<FormAnswer> = raw_form_answer_list
                .into_iter()
                .map(FormAnswer::from)
                .collect();
            (StatusCode::OK, Json(form_answer_list))
        })
        .map_err(|err| {
            tracing::error!("Failed to list form answer: {err:?}");
            err.into()
        })
}

pub async fn handle_post(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(raw_form_answer): Json<CreateFormAnswer>,
) -> Result<impl IntoResponse, AppError> {
    let form_answer = CreateFormAnswerDto::from(raw_form_answer);
    let res = modules
        .form_answer_use_case()
        .create(&ctx, form_answer)
        .await;
    res.map(|_| StatusCode::CREATED).map_err(|err| {
        tracing::error!("Failed to create form answer: {err:?}");
        err.into()
    })
}

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
