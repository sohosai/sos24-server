use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use sos24_use_case::context::Context;

use crate::{error::AppError, module::Modules};

use super::UserWithProject;

pub async fn handle(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules
        .user_use_case()
        .find_by_id(&ctx, ctx.user_id().clone().value())
        .await;
    match res {
        Ok(user) => Ok((StatusCode::OK, Json(UserWithProject::from(user)))),
        Err(err) => {
            tracing::error!("Failed to find me: {err}");
            Err(err.into())
        }
    }
}
