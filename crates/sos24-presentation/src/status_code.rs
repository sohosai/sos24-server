use axum::http::StatusCode;
use sos24_domain::{
    entity::{
        common::email::EmailError, news::NewsIdError, news_attachment::NewsAttachmentIdError,
        permission::PermissionDeniedError,
    },
    repository::{
        firebase_user::FirebaseUserRepositoryError, news::NewsRepositoryError,
        news_attachment::NewsAttachmentRepositoryError, user::UserRepositoryError,
    },
};
use sos24_use_case::interactor::{
    news::NewsUseCaseError, news_attachment::NewsAttachmentUseCaseError, user::UserUseCaseError,
};

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

impl ToStatusCode for NewsAttachmentUseCaseError {
    fn status_code(&self) -> StatusCode {
        match self {
            NewsAttachmentUseCaseError::NotFound(_) => StatusCode::NOT_FOUND,
            NewsAttachmentUseCaseError::NewsAttachmentRepositoryError(e) => e.status_code(),
            NewsAttachmentUseCaseError::NewsAttachmentIdError(e) => e.status_code(),
            NewsAttachmentUseCaseError::NewsAttachmentNewsIdError(e) => e.status_code(),
            NewsAttachmentUseCaseError::NewsAttachmentUrlError(e) => e.status_code(),
            NewsAttachmentUseCaseError::PermissionDeniedError(e) => e.status_code(),
            NewsAttachmentUseCaseError::InternalError(e) => e.status_code(),
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

impl ToStatusCode for NewsAttachmentRepositoryError {
    fn status_code(&self) -> StatusCode {
        match self {
            NewsAttachmentRepositoryError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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

impl ToStatusCode for NewsAttachmentIdError {
    fn status_code(&self) -> StatusCode {
        match self {
            NewsAttachmentIdError::InvalidUuid => StatusCode::BAD_REQUEST,
        }
    }
}

impl ToStatusCode for url::ParseError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
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
