use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};

use sos24_use_case::context::Context;

use crate::{error::AppError, module::Modules};

use super::Invitation;

pub async fn handle(
    Path(id): Path<String>,
    Extension(ctx): Extension<Context>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.invitation_use_case().find_by_id(&ctx, id).await;
    match res {
        Ok(raw_invitation) => Ok((StatusCode::OK, Json(Invitation::from(raw_invitation)))),
        Err(err) => {
            tracing::error!("Failed to find invitation: {err:?}");
            Err(err.into())
        }
    }
}
