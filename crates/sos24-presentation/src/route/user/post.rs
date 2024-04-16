use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sos24_use_case::user::use_case::create::CreateUserCommand;

use crate::{error::AppError, module::Modules};

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub kana_name: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
}

impl From<CreateUser> for CreateUserCommand {
    fn from(value: CreateUser) -> Self {
        CreateUserCommand {
            name: value.name,
            kana_name: value.kana_name,
            email: value.email,
            password: value.password,
            phone_number: value.phone_number,
        }
    }
}

#[derive(Serialize)]
pub struct CreatedUser {
    pub id: String,
}

pub async fn handle(
    State(modules): State<Arc<Modules>>,
    Json(user_data): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = CreateUserCommand::from(user_data);
    let res = modules.user_use_case().create(user).await;
    match res {
        Ok(id) => Ok((StatusCode::CREATED, Json(CreatedUser { id }))),
        Err(err) => {
            tracing::error!("Failed to create user: {err:?}");
            Err(err.into())
        }
    }
}
