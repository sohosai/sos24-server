use std::sync::Arc;

use axum::response::Response;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use sos24_use_case::{context::Context, dto::user::CreateUserDto};

use crate::error::AppError;
use crate::{
    model::user::{
        ConvertToUpdateUserDto, CreateUser, UpdateUser, User, UserSummary, UserTobeExported,
    },
    module::Modules,
};

pub async fn handle_get(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let raw_user_list = modules.user_use_case().list(&ctx).await;
    raw_user_list
        .map(|raw_user_list| {
            let user_list: Vec<UserSummary> =
                raw_user_list.into_iter().map(UserSummary::from).collect();
            (StatusCode::OK, Json(user_list))
        })
        .map_err(|err| {
            tracing::error!("Failed to list user: {err:?}");
            err.into()
        })
}

pub async fn handle_export(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let raw_user_list = modules.user_use_case().list(&ctx).await;
    let user_list = match raw_user_list {
        Ok(user_list) => user_list
            .into_iter()
            .map(UserTobeExported::from)
            .collect::<Vec<UserTobeExported>>(),
        Err(err) => {
            tracing::error!("Failed to list user: {err:?}");
            return Err(err.into());
        }
    };

    let mut wrt = csv::Writer::from_writer(vec![]);
    for user in user_list {
        match wrt.serialize(user) {
            Ok(result) => result,
            Err(err) => {
                tracing::error!("Failed to serialize user: {err:?}");
                return Err(AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "csv/failed-to-serialize".to_string(),
                    format!("{err:?}"),
                ));
            }
        }
    }

    let csv = match wrt.into_inner() {
        Ok(csv) => csv,
        Err(err) => {
            tracing::error!("Failed to write csv: {err:?}");
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "csv/failed-to-write".to_string(),
                format!("{err:?}"),
            ));
        }
    };

    let data = match String::from_utf8(csv) {
        Ok(data) => data,
        Err(err) => {
            tracing::error!("Failed to convert csv to string: {err:?}");
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "csv/failed-to-convert".to_string(),
                format!("{err:?}"),
            ));
        }
    };

    Response::builder()
        .header("Content-Type", "text/csv")
        .header("Content-Disposition", "attachment; filename=users.csv")
        .body(data)
        .map(|response| response)
        .map_err(|err| {
            tracing::error!("Failed to create response: {err:?}");
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "csv/failed-to-convert".to_string(),
                format!("{err:?}"),
            )
        })
}

pub async fn handle_post(
    State(modules): State<Arc<Modules>>,
    Json(raw_user): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = CreateUserDto::from(raw_user);
    let res = modules.user_use_case().create(user).await;
    res.map(|_| StatusCode::CREATED).map_err(|err| {
        tracing::error!("Failed to create user: {err:?}");
        err.into()
    })
}

pub async fn handle_get_id(
    Path(id): Path<String>,
    Extension(ctx): Extension<Context>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, AppError> {
    let raw_user = modules.user_use_case().find_by_id(&ctx, id).await;
    match raw_user {
        Ok(raw_user) => Ok((StatusCode::OK, Json(User::from(raw_user)))),
        Err(err) => {
            tracing::error!("Failed to find user: {err:?}");
            Err(err.into())
        }
    }
}

pub async fn handle_get_me(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let user = modules
        .user_use_case()
        .find_by_id(&ctx, ctx.user_id().clone().value())
        .await;
    match user {
        Ok(user) => Ok((StatusCode::OK, Json(User::from(user)))),
        Err(err) => {
            tracing::error!("Failed to find me: {err}");
            Err(err.into())
        }
    }
}

pub async fn handle_delete_id(
    Path(id): Path<String>,
    Extension(ctx): Extension<Context>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.user_use_case().delete_by_id(&ctx, id).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to delete user: {err:?}");
        err.into()
    })
}

pub async fn handle_put_id(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(raw_user): Json<UpdateUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = (id, raw_user).to_update_user_dto();
    let res = modules.user_use_case().update(&ctx, user).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to update user: {err:?}");
        err.into()
    })
}
