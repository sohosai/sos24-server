use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde::Deserialize;
use sos24_use_case::{
    context::Context,
    user::{dto::UserRoleDto, use_case::update::UpdateUserCommand},
};

use crate::{error::AppError, module::Modules};

use super::UserRole;

#[derive(Deserialize)]
pub struct UpdateUser {
    pub name: String,
    pub kana_name: String,
    pub email: String,
    pub phone_number: String,
    pub role: UserRole,
}

impl From<UpdateUser> for UpdateUserCommand {
    fn from(value: UpdateUser) -> Self {
        UpdateUserCommand {
            name: value.name,
            kana_name: value.kana_name,
            email: value.email,
            phone_number: value.phone_number,
            role: UserRoleDto::from(value.role),
        }
    }
}

pub async fn handle(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(user_data): Json<UpdateUser>,
) -> Result<impl IntoResponse, AppError> {
    let user_data = UpdateUserCommand::from(user_data);
    let res = modules.user_use_case().update(&ctx, id, user_data).await;
    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(err) => {
            tracing::error!("Failed to update user: {err:?}");
            Err(err.into())
        }
    }
}
