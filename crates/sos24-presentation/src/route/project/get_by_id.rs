use std::sync::Arc;

use axum::extract::Path;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

use sos24_use_case::context::Context;

use crate::error::AppError;
use crate::module::Modules;

use super::ProjectWithOwners;

pub async fn handle(
    Path(id): Path<String>,
    Extension(ctx): Extension<Context>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.project_use_case().find_by_id(&ctx, id).await;
    match res {
        Ok(raw_project) => Ok((StatusCode::OK, Json(ProjectWithOwners::from(raw_project)))),
        Err(err) => {
            tracing::error!("Failed to find project: {err:?}");
            Err(err.into())
        }
    }
}
