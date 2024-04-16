use std::sync::Arc;

use axum::extract::Path;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension};

use sos24_use_case::context::Context;

use crate::error::AppError;
use crate::module::Modules;

pub async fn handle(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.form_use_case().delete_by_id(&ctx, id).await;
    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(err) => {
            tracing::error!("Failed to delete form: {err:?}");
            Err(err.into())
        }
    }
}
