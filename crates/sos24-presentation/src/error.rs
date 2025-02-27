use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

pub mod convert_error;

pub struct AppError {
    status_code: StatusCode,
    code: String,
    message: String,
}

impl AppError {
    pub fn new(status_code: StatusCode, code: String, message: String) -> Self {
        Self {
            status_code,
            code,
            message,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    code: String,
    message: String,
}

impl From<AppError> for ErrorResponse {
    fn from(value: AppError) -> Self {
        ErrorResponse {
            code: value.code,
            message: value.message,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status_code, Json(ErrorResponse::from(self))).into_response()
    }
}
