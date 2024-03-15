use std::sync::Arc;

use axum::{Extension, extract::State, http::StatusCode, Json, response::IntoResponse};

use sos24_use_case::{context::Context, dto::form::CreateFormDto};

use crate::{model::form::CreateForm, module::Modules};
use crate::error::AppError;

pub async fn handle_post(
    State(module): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(raw_form): Json<CreateForm>,
) -> Result<impl IntoResponse, AppError> {
    let form = CreateFormDto::from(raw_form);
    let res = module.form_use_case().create(&ctx, form).await;
    res.map(|_| StatusCode::CREATED).map_err(|err| {
        tracing::error!("Failed to create form: {err:?}");
        err.into()
    })
}
