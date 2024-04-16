use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension,
};
use sos24_use_case::context::Context;

use crate::{error::AppError, module::Modules};

pub async fn handle(
    Path(id): Path<String>,
    Extension(ctx): Extension<Context>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.user_use_case().delete_by_id(&ctx, id).await;
    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(err) => {
            tracing::error!("Failed to delete user: {err:?}");
            Err(err.into())
        }
    }
}
