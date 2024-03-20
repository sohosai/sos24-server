use axum::http::StatusCode;
use sos24_domain::{
    entity::{
        common::email::EmailError,
        invitation::{InvitationError, InvitationIdError},
        news::NewsIdError,
        permission::PermissionDeniedError,
        project::{ProjectError, ProjectIdError},
    },
    repository::{
        firebase_user::FirebaseUserRepositoryError, invitation::InvitationRepositoryError,
        news::NewsRepositoryError, project::ProjectRepositoryError, user::UserRepositoryError,
    },
};
use sos24_use_case::{
    context::ContextError,
    interactor::{
        invitation::InvitationUseCaseError, news::NewsUseCaseError, project::ProjectUseCaseError,
        user::UserUseCaseError,
    },
};

use crate::error::AppError;

pub trait ToAppError {
    fn to_app_error(&self) -> AppError;
}

impl ToAppError for InvitationUseCaseError {
    fn to_app_error(&self) -> AppError {
        let message = self.to_string();
        match self {
            InvitationUseCaseError::NotFound(_) => AppError::new(
                StatusCode::NOT_FOUND,
                "invitation/not-found".to_string(),
                message,
            ),
            InvitationUseCaseError::InviterNotFound(_) => AppError::new(
                StatusCode::NOT_FOUND,
                "invitation/inviter-not-found".to_string(),
                message,
            ),
            InvitationUseCaseError::ProjectNotFound(_) => AppError::new(
                StatusCode::NOT_FOUND,
                "invitation/project-not-found".to_string(),
                message,
            ),
            InvitationUseCaseError::AlreadyOwnerOrSubOwner => AppError::new(
                StatusCode::CONFLICT,
                "invitation/already-owner-or-subowner".to_string(),
                message,
            ),
            InvitationUseCaseError::ProjectError(e) => e.to_app_error(),
            InvitationUseCaseError::InvitationError(e) => e.to_app_error(),
            InvitationUseCaseError::InvitationIdError(e) => e.to_app_error(),
            InvitationUseCaseError::ProjectIdError(e) => e.to_app_error(),
            InvitationUseCaseError::EmailError(e) => e.to_app_error(),
            InvitationUseCaseError::InvitationRepositoryError(e) => e.to_app_error(),
            InvitationUseCaseError::ProjectRepositoryError(e) => e.to_app_error(),
            InvitationUseCaseError::UserRepositoryError(e) => e.to_app_error(),
            InvitationUseCaseError::ContextError(e) => e.to_app_error(),
            InvitationUseCaseError::PermissionDeniedError(e) => e.to_app_error(),
            InvitationUseCaseError::InternalError(e) => e.to_app_error(),
        }
    }
}

impl ToAppError for NewsUseCaseError {
    fn to_app_error(&self) -> AppError {
        let message = self.to_string();
        match self {
            NewsUseCaseError::NotFound(_) => {
                AppError::new(StatusCode::NOT_FOUND, "news/not-found".to_string(), message)
            }
            NewsUseCaseError::ContextError(e) => e.to_app_error(),
            NewsUseCaseError::NewsRepositoryError(e) => e.to_app_error(),
            NewsUseCaseError::NewsIdError(e) => e.to_app_error(),
            NewsUseCaseError::PermissionDeniedError(e) => e.to_app_error(),
            NewsUseCaseError::InternalError(e) => e.to_app_error(),
        }
    }
}

impl ToAppError for ProjectUseCaseError {
    fn to_app_error(&self) -> AppError {
        let message = self.to_string();
        match self {
            ProjectUseCaseError::NotFound(_) => AppError::new(
                StatusCode::NOT_FOUND,
                "project/not-found".to_string(),
                message,
            ),
            ProjectUseCaseError::AlreadyOwnedProject(_) => AppError::new(
                StatusCode::CONFLICT,
                "project/alread-owned-project".to_string(),
                message,
            ),
            ProjectUseCaseError::ApplicationsNotAccepted => AppError::new(
                StatusCode::BAD_REQUEST,
                "project/applications-not-accepted".to_string(),
                message,
            ),
            ProjectUseCaseError::ContextError(e) => e.to_app_error(),
            ProjectUseCaseError::ProjectRepositoryError(e) => e.to_app_error(),
            ProjectUseCaseError::ProjectIdError(e) => e.to_app_error(),
            ProjectUseCaseError::PermissionDeniedError(e) => e.to_app_error(),
            ProjectUseCaseError::InternalError(e) => e.to_app_error(),
        }
    }
}

impl ToAppError for UserUseCaseError {
    fn to_app_error(&self) -> AppError {
        match self {
            UserUseCaseError::NotFound(_) => AppError::new(
                StatusCode::NOT_FOUND,
                "user/not-found".to_string(),
                self.to_string(),
            ),
            UserUseCaseError::ContextError(e) => e.to_app_error(),
            UserUseCaseError::UserRepositoryError(e) => e.to_app_error(),
            UserUseCaseError::FirebaseUserRepositoryError(e) => e.to_app_error(),
            UserUseCaseError::EmailError(e) => e.to_app_error(),
            UserUseCaseError::PermissionDenied(e) => e.to_app_error(),
            UserUseCaseError::InternalError(e) => e.to_app_error(),
        }
    }
}

impl ToAppError for ContextError {
    fn to_app_error(&self) -> AppError {
        match self {
            ContextError::UserNotFound(_) => AppError::new(
                StatusCode::NOT_FOUND,
                "user/not-found".to_string(),
                self.to_string(),
            ),
            ContextError::UserRepositoryError(e) => e.to_app_error(),
            ContextError::ProjectRepositoryError(e) => e.to_app_error(),
        }
    }
}

impl ToAppError for InvitationRepositoryError {
    fn to_app_error(&self) -> AppError {
        match self {
            InvitationRepositoryError::InternalError(e) => e.to_app_error(),
        }
    }
}

impl ToAppError for NewsRepositoryError {
    fn to_app_error(&self) -> AppError {
        match self {
            NewsRepositoryError::InternalError(e) => e.to_app_error(),
        }
    }
}

impl ToAppError for ProjectRepositoryError {
    fn to_app_error(&self) -> AppError {
        match self {
            ProjectRepositoryError::InternalError(e) => e.to_app_error(),
        }
    }
}

impl ToAppError for UserRepositoryError {
    fn to_app_error(&self) -> AppError {
        match self {
            UserRepositoryError::EmailAlreadyUsed(_) => AppError::new(
                StatusCode::BAD_REQUEST,
                // メールアドレスが既に使われていることを外に出さない
                "user/bad-credential".to_string(),
                "Bad credential".to_string(),
            ),
            UserRepositoryError::PhoneNumberAlreadyUsed(_) => AppError::new(
                StatusCode::BAD_REQUEST,
                // 電話番号が既に使われていることを外に出さない
                "user/bad-credential".to_string(),
                "Bad credential".to_string(),
            ),
            UserRepositoryError::InternalError(e) => e.to_app_error(),
        }
    }
}

impl ToAppError for FirebaseUserRepositoryError {
    fn to_app_error(&self) -> AppError {
        match self {
            FirebaseUserRepositoryError::EmailExists(_) => AppError::new(
                StatusCode::BAD_REQUEST,
                // メールアドレスが既に使われていることを外に出さない
                "user/bad-credential".to_string(),
                "Bad credential".to_string(),
            ),
            FirebaseUserRepositoryError::InternalError(e) => e.to_app_error(),
        }
    }
}

impl ToAppError for ProjectError {
    fn to_app_error(&self) -> AppError {
        match self {
            ProjectError::AlreadyOwnerOrSubOwner => AppError::new(
                StatusCode::CONFLICT,
                "project/already-owner-or-sub-owner".to_string(),
                self.to_string(),
            ),
        }
    }
}

impl ToAppError for InvitationError {
    fn to_app_error(&self) -> AppError {
        match self {
            InvitationError::AlreadyUsed => AppError::new(
                StatusCode::BAD_REQUEST,
                "invitation/already-used".to_string(),
                self.to_string(),
            ),
            InvitationError::InviterAndReceiverAreSame => AppError::new(
                StatusCode::BAD_REQUEST,
                "invitation/inviter-and-receiver-are-same".to_string(),
                self.to_string(),
            ),
        }
    }
}

impl ToAppError for InvitationIdError {
    fn to_app_error(&self) -> AppError {
        match self {
            InvitationIdError::InvalidUuid => AppError::new(
                StatusCode::BAD_REQUEST,
                "invitation/invalid-uuid".to_string(),
                self.to_string(),
            ),
        }
    }
}

impl ToAppError for NewsIdError {
    fn to_app_error(&self) -> AppError {
        match self {
            NewsIdError::InvalidUuid => AppError::new(
                StatusCode::BAD_REQUEST,
                "news/invalid-uuid".to_string(),
                self.to_string(),
            ),
        }
    }
}

impl ToAppError for ProjectIdError {
    fn to_app_error(&self) -> AppError {
        match self {
            ProjectIdError::InvalidUuid => AppError::new(
                StatusCode::BAD_REQUEST,
                "project/invalid-uuid".to_string(),
                self.to_string(),
            ),
        }
    }
}

impl ToAppError for EmailError {
    fn to_app_error(&self) -> AppError {
        match self {
            EmailError::InvalidFormat => AppError::new(
                StatusCode::BAD_REQUEST,
                "email/invalid-format".to_string(),
                self.to_string(),
            ),
            EmailError::InvalidDomain => AppError::new(
                StatusCode::BAD_GATEWAY,
                "email/invalid-domain".to_string(),
                self.to_string(),
            ),
        }
    }
}

impl ToAppError for PermissionDeniedError {
    fn to_app_error(&self) -> AppError {
        AppError::new(
            StatusCode::FORBIDDEN,
            "permission-denied".to_string(),
            self.to_string(),
        )
    }
}

impl ToAppError for anyhow::Error {
    fn to_app_error(&self) -> AppError {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal-error".to_string(),
            self.to_string(),
        )
    }
}
