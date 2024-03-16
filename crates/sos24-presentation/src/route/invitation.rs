use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use sos24_use_case::context::Context;

use crate::{
    model::invitation::{ConvertToCreateInvitationDto, CreateInvitation, Invitation},
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

pub async fn handle_get_id(
    Path(id): Path<String>,
    Extension(ctx): Extension<Context>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let raw_invitation = modules.invitation_use_case().find_by_id(&ctx, id).await;
    match raw_invitation {
        Ok(raw_invitation) => Ok((StatusCode::OK, Json(Invitation::from(raw_invitation)))),
        Err(err) => {
            tracing::error!("Failed to find invitation: {err:?}");
            Err(err.status_code())
        }
    }
}

pub async fn handle_post_id(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.invitation_use_case().receive(&ctx, id).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to receive invitation: {err:?}");
        err.status_code()
    })
}
