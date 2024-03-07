use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sos24_use_case::{
    dto::user::CreateUserDto,
    error::{user::UserError, UseCaseError},
};

use crate::{
    model::user::{ConvertToUpdateUserDto, CreateUser, UpdateUser, User},
    module::Modules,
};

use super::ToStatusCode;

impl ToStatusCode for UseCaseError<UserError> {
    fn status_code(&self) -> StatusCode {
        match self {
            UseCaseError::UseCase(UserError::NotFound(_)) => StatusCode::NOT_FOUND,
            UseCaseError::UseCase(UserError::InvalidEmail(_)) => StatusCode::BAD_REQUEST,
            UseCaseError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub async fn handle_get(
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let raw_user_list = modules.user_use_case().list().await;
    raw_user_list
        .map(|raw_user_list| {
            let user_list: Vec<User> = raw_user_list.into_iter().map(User::from).collect();
            (StatusCode::OK, Json(user_list))
        })
        .map_err(|err| {
            tracing::error!("Failed to list user: {err}");
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
        tracing::error!("Failed to create user: {err}");
        err.status_code()
    })
}

pub async fn handle_get_id(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let raw_user = modules.user_use_case().find_by_id(id).await;
    match raw_user {
        Ok(raw_user) => Ok((StatusCode::OK, Json(User::from(raw_user)))),
        Err(err) => {
            tracing::error!("Failed to find user: {err}");
            Err(err.status_code())
        }
    }
}

pub async fn handle_delete_id(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.user_use_case().delete_by_id(id).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to delete user: {err}");
        err.status_code()
    })
}

pub async fn handle_put_id(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Json(raw_user): Json<UpdateUser>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = (id, raw_user).to_update_user_dto();
    let res = modules.user_use_case().update(user).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to update user: {err}");
        err.status_code()
    })
}
