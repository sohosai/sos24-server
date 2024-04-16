use std::sync::Arc;

use axum::extract::Path;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

use sos24_use_case::context::Context;

use crate::error::AppError;
use crate::module::Modules;

use super::Form;

pub async fn handle(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.form_use_case().find_by_id(&ctx, id).await;
    match res {
        Ok(raw_form) => Ok((StatusCode::OK, Json(Form::from(raw_form)))),
        Err(err) => {
            tracing::error!("Failed to find form by id: {err:?}");
            Err(err.into())
        }
    }
}
