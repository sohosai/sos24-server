use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use sos24_use_case::{context::Context, dto::user::CreateUserDto};

use crate::{
    model::user::{ConvertToUpdateUserDto, CreateUser, UpdateUser, User},
    module::Modules,
    status_code::ToStatusCode,
};

pub async fn handle_get(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, StatusCode> {
    let raw_user_list = modules.user_use_case().list(&ctx).await;
    raw_user_list
        .map(|raw_user_list| {
            let user_list: Vec<User> = raw_user_list.into_iter().map(User::from).collect();
            (StatusCode::OK, Json(user_list))
        })
        .map_err(|err| {
            tracing::error!("Failed to list user: {err:?}");
            err.status_code()
        })
}

pub async fn handle_post(
    State(modules): State<Arc<Modules>>,
    Json(raw_user): Json<CreateUser>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = CreateUserDto::from(raw_user);
    let res = modules.user_use_case().create(user).await;
    res.map(|_| StatusCode::CREATED).map_err(|err| {
        tracing::error!("Failed to create user: {err:?}");
        err.status_code()
    })
}

pub async fn handle_get_id(
    Path(id): Path<String>,
    Extension(ctx): Extension<Context>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let raw_user = modules.user_use_case().find_by_id(&ctx, id).await;
    match raw_user {
        Ok(raw_user) => Ok((StatusCode::OK, Json(User::from(raw_user)))),
        Err(err) => {
            tracing::error!("Failed to find user: {err:?}");
            Err(err.status_code())
        }
    }
}

pub async fn handle_get_me(
    State(modules): State<Arc<Modules>>,
    Extension(actor): Extension<Actor>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = modules.user_use_case().find_by_id(&actor, actor.user_id().clone().value()).await;
    match user {
        Ok(user) => Ok((StatusCode::OK, Json(User::from(user)))),
        Err(err) => {
            tracing::error!("Failed to find me: {err}");
            Err(err.status_code())
        }
    }
}

pub async fn handle_delete_id(
    Path(id): Path<String>,
    Extension(ctx): Extension<Context>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.user_use_case().delete_by_id(&ctx, id).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to delete user: {err:?}");
        err.status_code()
    })
}

pub async fn handle_put_id(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(raw_user): Json<UpdateUser>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = (id, raw_user).to_update_user_dto();
    let res = modules.user_use_case().update(&ctx, user).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to update user: {err:?}");
        err.status_code()
    })
}
