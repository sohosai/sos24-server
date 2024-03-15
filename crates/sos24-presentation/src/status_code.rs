use axum::http::StatusCode;
use sos24_domain::{
    entity::{
        common::email::EmailError, news::NewsIdError, permission::PermissionDeniedError,
        project::ProjectIdError,
    },
    repository::{
        firebase_user::FirebaseUserRepositoryError, news::NewsRepositoryError,
        project::ProjectRepositoryError, user::UserRepositoryError,
    },
};
use sos24_use_case::{
    context::ContextError,
    interactor::{news::NewsUseCaseError, project::ProjectUseCaseError, user::UserUseCaseError},
};

pub trait ToStatusCode {
    fn status_code(&self) -> StatusCode;
}

impl ToStatusCode for NewsUseCaseError {
    fn status_code(&self) -> StatusCode {
        match self {
            NewsUseCaseError::NotFound(_) => StatusCode::NOT_FOUND,
            NewsUseCaseError::ContextError(e) => e.status_code(),
            NewsUseCaseError::NewsRepositoryError(e) => e.status_code(),
            NewsUseCaseError::NewsIdError(e) => e.status_code(),
            NewsUseCaseError::PermissionDeniedError(e) => e.status_code(),
            NewsUseCaseError::InternalError(e) => e.status_code(),
        }
    }
}

impl ToStatusCode for ProjectUseCaseError {
    fn status_code(&self) -> StatusCode {
        match self {
            ProjectUseCaseError::NotFound(_) => StatusCode::NOT_FOUND,
            ProjectUseCaseError::ApplicationsNotAccepted => StatusCode::BAD_REQUEST,
            ProjectUseCaseError::AlreadyOwnedProject(_) => StatusCode::CONFLICT,
            ProjectUseCaseError::ContextError(e) => e.status_code(),
            ProjectUseCaseError::ProjectRepositoryError(e) => e.status_code(),
            ProjectUseCaseError::ProjectIdError(e) => e.status_code(),
            ProjectUseCaseError::PermissionDeniedError(e) => e.status_code(),
            ProjectUseCaseError::InternalError(e) => e.status_code(),
        }
    }
}

impl ToStatusCode for UserUseCaseError {
    fn status_code(&self) -> StatusCode {
        match self {
            UserUseCaseError::NotFound(_) => StatusCode::NOT_FOUND,
            UserUseCaseError::ContextError(e) => e.status_code(),
            UserUseCaseError::UserRepositoryError(e) => e.status_code(),
            UserUseCaseError::FirebaseUserRepositoryError(e) => e.status_code(),
            UserUseCaseError::EmailError(e) => e.status_code(),
            UserUseCaseError::PermissionDenied(e) => e.status_code(),
            UserUseCaseError::InternalError(e) => e.status_code(),
        }
    }
}

impl ToStatusCode for ContextError {
    fn status_code(&self) -> StatusCode {
        match self {
            ContextError::UserNotFound(_) => StatusCode::NOT_FOUND,
            ContextError::UserRepositoryError(e) => e.status_code(),
            ContextError::ProjectRepositoryError(e) => e.status_code(),
        }
    }
}

impl ToStatusCode for NewsRepositoryError {
    fn status_code(&self) -> StatusCode {
        match self {
            NewsRepositoryError::InternalError(e) => e.status_code(),
        }
    }
}

impl ToStatusCode for ProjectRepositoryError {
    fn status_code(&self) -> StatusCode {
        match self {
            ProjectRepositoryError::InternalError(e) => e.status_code(),
        }
    }
}

impl ToStatusCode for UserRepositoryError {
    fn status_code(&self) -> StatusCode {
        match self {
            UserRepositoryError::EmailAlreadyUsed(_) => StatusCode::CONFLICT,
            UserRepositoryError::PhoneNumberAlreadyUsed(_) => StatusCode::CONFLICT,
            UserRepositoryError::InternalError(e) => e.status_code(),
        }
    }
}

impl ToStatusCode for FirebaseUserRepositoryError {
    fn status_code(&self) -> StatusCode {
        match self {
            FirebaseUserRepositoryError::EmailExists(_) => StatusCode::CONFLICT,
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

impl ToStatusCode for ProjectIdError {
    fn status_code(&self) -> StatusCode {
        match self {
            ProjectIdError::InvalidUuid => StatusCode::BAD_REQUEST,
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
