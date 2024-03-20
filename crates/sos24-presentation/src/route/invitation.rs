use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use sos24_use_case::context::Context;

use crate::{
    error::AppError,
    model::invitation::{ConvertToCreateInvitationDto, CreateInvitation, Invitation},
    module::Modules,
    status_code::ToAppError,
};

pub async fn handle_get(
    Extension(ctx): Extension<Context>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, AppError> {
    let raw_invitation_list = modules.invitation_use_case().list(&ctx).await;
    raw_invitation_list
        .map(|raw_invitation_list| {
            let invitation_list: Vec<Invitation> = raw_invitation_list
                .into_iter()
                .map(Invitation::from)
                .collect();
            (StatusCode::OK, Json(invitation_list))
        })
        .map_err(|err| {
            tracing::error!("Failed to list invitations: {err:?}");
            err.to_app_error()
        })
}

pub async fn handle_post(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(raw_invitation): Json<CreateInvitation>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = ctx.user_id().clone().value();
    let invitation = (raw_invitation, user_id).to_create_invitation_dto();
    let res = modules.invitation_use_case().create(&ctx, invitation).await;
    res.map(|_| StatusCode::CREATED).map_err(|err| {
        tracing::error!("Failed to create invitation: {err:?}");
        err.to_app_error()
    })
}

pub async fn handle_get_id(
    Path(id): Path<String>,
    Extension(ctx): Extension<Context>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, AppError> {
    let raw_invitation = modules.invitation_use_case().find_by_id(&ctx, id).await;
    match raw_invitation {
        Ok(raw_invitation) => Ok((StatusCode::OK, Json(Invitation::from(raw_invitation)))),
        Err(err) => {
            tracing::error!("Failed to find invitation: {err:?}");
            Err(err.to_app_error())
        }
    }
}

pub async fn handle_post_id(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.invitation_use_case().receive(&ctx, id).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to receive invitation: {err:?}");
        err.to_app_error()
    })
}

pub async fn handle_delete_id(
    Path(id): Path<String>,
    Extension(ctx): Extension<Context>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.invitation_use_case().delete_by_id(&ctx, id).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to delete invitation: {err:?}");
        err.to_app_error()
    })
}
