use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Serialize;
use sos24_use_case::{context::Context, user::dto::UserDto};

use crate::{error::AppError, module::Modules};

use super::UserRole;

#[derive(Debug, Serialize)]
pub struct UserSummary {
    id: String,
    name: String,
    email: String,
    role: UserRole,
}

impl From<UserDto> for UserSummary {
    fn from(value: UserDto) -> Self {
        UserSummary {
            id: value.id,
            name: value.name,
            email: value.email,
            role: value.role.into(),
        }
    }
}

pub async fn handle(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.user_use_case().list(&ctx).await;
    match res {
        Ok(raw_user_list) => {
            let user_list: Vec<_> = raw_user_list.into_iter().map(UserSummary::from).collect();
            Ok((StatusCode::OK, Json(user_list)))
        }
        Err(err) => {
            tracing::error!("Failed to list user: {err:?}");
            Err(err.into())
        }
    }
}
