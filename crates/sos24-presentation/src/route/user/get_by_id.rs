use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use sos24_use_case::context::Context;

use crate::{error::AppError, module::Modules};

use super::UserWithProject;

pub async fn handle(
    Path(id): Path<String>,
    Extension(ctx): Extension<Context>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.user_use_case().find_by_id(&ctx, id).await;
    match res {
        Ok(raw_user) => Ok((StatusCode::OK, Json(UserWithProject::from(raw_user)))),
        Err(err) => {
            tracing::error!("Failed to find user: {err:?}");
            Err(err.into())
        }
    }
}
