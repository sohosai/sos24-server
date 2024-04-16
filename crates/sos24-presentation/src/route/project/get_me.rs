use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

use sos24_use_case::context::Context;

use crate::error::AppError;
use crate::module::Modules;

use super::ProjectWithOwners;

pub async fn handle(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.project_use_case().find_owned(&ctx).await;
    match res {
        Ok(Some(raw_project)) => Ok((StatusCode::OK, Json(ProjectWithOwners::from(raw_project)))),
        Ok(None) => Err(AppError::new(
            StatusCode::NOT_FOUND,
            "project/no-project-found".to_string(),
            "Project not found".to_string(),
        )),
        Err(err) => {
            tracing::error!("Failed to find me: {err}");
            Err(err.into())
        }
    }
}
