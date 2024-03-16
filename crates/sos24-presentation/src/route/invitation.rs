use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use sos24_use_case::context::Context;

use crate::{
    model::invitation::{ConvertToCreateInvitationDto, CreateInvitation},
    module::Modules,
    status_code::ToStatusCode,
};

pub async fn handle_post(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(raw_invitation): Json<CreateInvitation>,
) -> Result<impl IntoResponse, StatusCode> {
    let user_id = ctx.user_id().clone().value();
    let invitation = (raw_invitation, user_id).to_create_invitation_dto();
    let res = modules.invitation_use_case().create(&ctx, invitation).await;
    res.map(|_| StatusCode::CREATED).map_err(|err| {
        tracing::error!("Failed to create invitation: {err:?}");
        err.status_code()
    })
}
