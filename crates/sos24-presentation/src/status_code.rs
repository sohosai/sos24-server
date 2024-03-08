use axum::http::StatusCode;
use sos24_domain::{
    entity::{common::email::EmailError, news::NewsIdError, permission::PermissionDeniedError},
    repository::{
        firebase_user::FirebaseUserRepositoryError, news::NewsRepositoryError,
        user::UserRepositoryError,
    },
};
use sos24_use_case::interactor::{news::NewsUseCaseError, user::UserUseCaseError};

pub trait ToStatusCode {
    fn status_code(&self) -> StatusCode;
}

impl ToStatusCode for NewsUseCaseError {
    fn status_code(&self) -> StatusCode {
        match self {
            NewsUseCaseError::NotFound(_) => StatusCode::NOT_FOUND,
            NewsUseCaseError::NewsRepositoryError(e) => e.status_code(),
            NewsUseCaseError::NewsIdError(e) => e.status_code(),
            NewsUseCaseError::PermissionDeniedError(e) => e.status_code(),
            NewsUseCaseError::InternalError(e) => e.status_code(),
        }
    }
}

impl ToStatusCode for UserUseCaseError {
    fn status_code(&self) -> StatusCode {
        match self {
            UserUseCaseError::NotFound(_) => StatusCode::NOT_FOUND,
            UserUseCaseError::UserRepositoryError(e) => e.status_code(),
            UserUseCaseError::FirebaseUserRepositoryError(e) => e.status_code(),
            UserUseCaseError::EmailError(e) => e.status_code(),
            UserUseCaseError::PermissionDenied(e) => e.status_code(),
            UserUseCaseError::InternalError(e) => e.status_code(),
        }
    }
}

impl ToStatusCode for NewsRepositoryError {
    fn status_code(&self) -> StatusCode {
        match self {
            NewsRepositoryError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl ToStatusCode for UserRepositoryError {
    fn status_code(&self) -> StatusCode {
        match self {
            UserRepositoryError::InternalError(e) => e.status_code(),
        }
    }
}

impl ToStatusCode for FirebaseUserRepositoryError {
    fn status_code(&self) -> StatusCode {
        match self {
            FirebaseUserRepositoryError::EmailExists => StatusCode::CONFLICT,
            FirebaseUserRepositoryError::InternalError(e) => e.status_code(),
        }
    }
}

impl ToStatusCode for NewsIdError {
    fn status_code(&self) -> StatusCode {
        match self {
            NewsIdError::InvalidUuid => StatusCode::BAD_REQUEST,
        }
    }
}

impl ToStatusCode for EmailError {
    fn status_code(&self) -> StatusCode {
        match self {
            EmailError::InvalidFormat => StatusCode::BAD_REQUEST,
            EmailError::InvalidDomain => StatusCode::BAD_GATEWAY,
        }
    }
}

impl ToStatusCode for PermissionDeniedError {
    fn status_code(&self) -> StatusCode {
        StatusCode::FORBIDDEN
    }
}

impl ToStatusCode for anyhow::Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
