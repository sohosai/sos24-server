use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

use sos24_use_case::context::Context;

use crate::{error::AppError, module::Modules};

use super::Invitation;

pub async fn handle(
    Extension(ctx): Extension<Context>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.invitation_use_case().list(&ctx).await;
    match res {
        Ok(raw_invitation_list) => {
            let invitation_list: Vec<_> = raw_invitation_list
                .into_iter()
                .map(Invitation::from)
                .collect();
            Ok((StatusCode::OK, Json(invitation_list)))
        }
        Err(err) => {
            tracing::error!("Failed to list invitations: {err:?}");
            Err(err.into())
        }
    }
}
