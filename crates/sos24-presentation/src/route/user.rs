use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sos24_use_case::error::{user::UserError, UseCaseError};

use crate::{
    model::user::{ConvertToCreateUserDto, CreateUser},
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

pub async fn handle_post(
    State(modules): State<Arc<Modules>>,
    Json(raw_user): Json<CreateUser>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = "test_id".to_string(); // TODO

    let user = (id, raw_user).to_create_user_dto();
    let res = modules.user_use_case().create(user).await;
    res.map(|_| StatusCode::CREATED).map_err(|err| {
        tracing::error!("Failed to create user: {:?}", err);
        err.status_code()
    })
}
