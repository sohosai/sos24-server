use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use sos24_use_case::{context::Context, dto::form_answer::CreateFormAnswerDto};

use crate::{
    error::AppError,
    model::form_answer::{CreateFormAnswer, FormAnswer},
    module::Modules,
};

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
