use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use sos24_use_case::shared::context::ContextProvider;

use crate::{
    context::Context,
    error::ErrorResponse,
    model::invitation::{ConvertToCreateInvitationDto, CreatedInvitation},
};
use crate::{
    error::AppError,
    model::invitation::{CreateInvitation, Invitation},
    module::Modules,
};

/// 招待一覧の取得
#[utoipa::path(
    get,
    path = "/invitations",
    operation_id = "getInvitations",
    tag = "invitations",
    responses(
        (status = 200, description = "OK", body = Vec<Invitation>),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse),
    ),
    security(("jwt_token" = [])),
)]
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
            err.into()
        })
}

/// 招待の作成
#[utoipa::path(
    post,
    path = "/invitations",
    operation_id = "postInvitation",
    tag = "invitations",
    request_body(content = CreateInvitation),
    responses(
        (status = 201, description = "Created", body = CreatedInvitation),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 422, description = "Unprocessable Entity", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse),
    ),
    security(("jwt_token" = [])),
)]
pub async fn handle_post(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(raw_invitation): Json<CreateInvitation>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = ctx.user_id();
    let invitation = (raw_invitation, user_id).to_create_invitation_dto();
    let res = modules
        .invitation_use_case()
        .find_or_create(&ctx, invitation)
        .await;
    res.map(|id| (StatusCode::CREATED, Json(CreatedInvitation { id })))
        .map_err(|err| {
            tracing::error!("Failed to create invitation: {err:?}");
            err.into()
        })
}

/// 特定のIDの招待の取得
#[utoipa::path(
    get,
    path = "/invitations/{invitation_id}",
    operation_id = "getInvitationById",
    tag = "invitations",
    params(("invitation_id" = String, Path, format="uuid")),
    responses(
        (status = 200, description = "OK", body = Invitation),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse),
    ),
    security(("jwt_token" = [])),
)]
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
            Err(err.into())
        }
    }
}

/// 特定のIDの招待の受諾
#[utoipa::path(
    post,
    path = "/invitations/{invitation_id}",
    operation_id = "postInvitationById",
    tag = "invitations",
    params(("invitation_id" = String, Path, format="uuid")),
    responses(
        (status = 200, description = "OK"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse),
    ),
    security(("jwt_token" = [])),
)]
pub async fn handle_post_id(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.invitation_use_case().receive(&ctx, id).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to receive invitation: {err:?}");
        err.into()
    })
}

/// 特定のIDの招待の削除
#[utoipa::path(
    delete,
    path = "/invitations/{invitation_id}",
    operation_id = "deleteInvitationById",
    tag = "invitations",
    params(("invitation_id" = String, Path, format="uuid")),
    responses(
        (status = 200, description = "OK"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse),
    ),
    security(("jwt_token" = [])),
)]
pub async fn handle_delete_id(
    Path(id): Path<String>,
    Extension(ctx): Extension<Context>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.invitation_use_case().delete_by_id(&ctx, id).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to delete invitation: {err:?}");
        err.into()
    })
}
