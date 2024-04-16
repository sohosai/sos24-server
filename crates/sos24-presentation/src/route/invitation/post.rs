use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

use serde::{Deserialize, Serialize};
use sos24_use_case::{
    context::Context,
    invitation::{dto::InvitationPositionDto, use_case::find_or_create::CreateInvitationCommand},
};

use crate::{error::AppError, module::Modules};

use super::InvitationPosition;

#[derive(Debug, Deserialize)]
pub struct CreateInvitation {
    project_id: String,
    position: InvitationPosition,
}

impl From<CreateInvitation> for CreateInvitationCommand {
    fn from(raw: CreateInvitation) -> Self {
        Self {
            project_id: raw.project_id,
            position: InvitationPositionDto::from(raw.position),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreatedInvitation {
    pub id: String,
}

pub async fn handle(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(invitation_data): Json<CreateInvitation>,
) -> Result<impl IntoResponse, AppError> {
    let invitation = CreateInvitationCommand::from(invitation_data);
    let res = modules
        .invitation_use_case()
        .find_or_create(&ctx, invitation)
        .await;
    match res {
        Ok(id) => Ok((StatusCode::CREATED, Json(CreatedInvitation { id }))),
        Err(err) => {
            tracing::error!("Failed to create invitation: {err:?}");
            Err(err.into())
        }
    }
}
