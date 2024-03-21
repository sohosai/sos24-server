use axum::http::StatusCode;

use sos24_domain::entity::common::datetime::DateTimeError;
use sos24_domain::entity::form::FormIdError;
use sos24_domain::repository::form::FormRepositoryError;
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
use sos24_use_case::interactor::form::FormUseCaseError;
use sos24_use_case::{
    context::ContextError,
    interactor::{
        invitation::InvitationUseCaseError, news::NewsUseCaseError, project::ProjectUseCaseError,
        user::UserUseCaseError,
    },
};

use super::AppError;

impl From<FormUseCaseError> for AppError {
    fn from(error: FormUseCaseError) -> Self {
        let message = error.to_string();
        match error {
            FormUseCaseError::NotFound(_) => {
                AppError::new(StatusCode::NOT_FOUND, "form/not-found".to_string(), message)
            }
            FormUseCaseError::ProjectUseCaseError(e) => e.into(),
            FormUseCaseError::DateTimeError(e) => e.into(),
            FormUseCaseError::FormRepositoryError(e) => e.into(),
            FormUseCaseError::ContextError(e) => e.into(),
            FormUseCaseError::PermissionDeniedError(e) => e.into(),
            FormUseCaseError::InternalError(e) => e.into(),
            FormUseCaseError::FormIdError(e) => e.into(),
        }
    }
}

impl From<InvitationUseCaseError> for AppError {
    fn from(error: InvitationUseCaseError) -> AppError {
        let message = error.to_string();
        match error {
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
            InvitationUseCaseError::ProjectError(e) => e.into(),
            InvitationUseCaseError::InvitationError(e) => e.into(),
            InvitationUseCaseError::InvitationIdError(e) => e.into(),
            InvitationUseCaseError::ProjectIdError(e) => e.into(),
            InvitationUseCaseError::EmailError(e) => e.into(),
            InvitationUseCaseError::InvitationRepositoryError(e) => e.into(),
            InvitationUseCaseError::ProjectRepositoryError(e) => e.into(),
            InvitationUseCaseError::UserRepositoryError(e) => e.into(),
            InvitationUseCaseError::ContextError(e) => e.into(),
            InvitationUseCaseError::PermissionDeniedError(e) => e.into(),
            InvitationUseCaseError::InternalError(e) => e.into(),
        }
    }
}

impl From<NewsUseCaseError> for AppError {
    fn from(error: NewsUseCaseError) -> AppError {
        let message = error.to_string();
        match error {
            NewsUseCaseError::NotFound(_) => {
                AppError::new(StatusCode::NOT_FOUND, "news/not-found".to_string(), message)
            }
            NewsUseCaseError::ProjectUseCaseError(e) => e.into(),
            NewsUseCaseError::ContextError(e) => e.into(),
            NewsUseCaseError::NewsRepositoryError(e) => e.into(),
            NewsUseCaseError::NewsIdError(e) => e.into(),
            NewsUseCaseError::PermissionDeniedError(e) => e.into(),
            NewsUseCaseError::InternalError(e) => e.into(),
        }
    }
}

impl From<ProjectUseCaseError> for AppError {
    fn from(error: ProjectUseCaseError) -> AppError {
        let message = error.to_string();
        match error {
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
            ProjectUseCaseError::ContextError(e) => e.into(),
            ProjectUseCaseError::ProjectRepositoryError(e) => e.into(),
            ProjectUseCaseError::ProjectIdError(e) => e.into(),
            ProjectUseCaseError::PermissionDeniedError(e) => e.into(),
            ProjectUseCaseError::InternalError(e) => e.into(),
        }
    }
}

impl From<UserUseCaseError> for AppError {
    fn from(error: UserUseCaseError) -> AppError {
        match error {
            UserUseCaseError::NotFound(_) => AppError::new(
                StatusCode::NOT_FOUND,
                "user/not-found".to_string(),
                error.to_string(),
            ),
            UserUseCaseError::ContextError(e) => e.into(),
            UserUseCaseError::UserRepositoryError(e) => e.into(),
            UserUseCaseError::FirebaseUserRepositoryError(e) => e.into(),
            UserUseCaseError::EmailError(e) => e.into(),
            UserUseCaseError::PermissionDenied(e) => e.into(),
            UserUseCaseError::InternalError(e) => e.into(),
        }
    }
}

impl From<ContextError> for AppError {
    fn from(error: ContextError) -> AppError {
        match error {
            ContextError::UserNotFound(_) => AppError::new(
                StatusCode::NOT_FOUND,
                "user/not-found".to_string(),
                error.to_string(),
            ),
            ContextError::UserRepositoryError(e) => e.into(),
            ContextError::ProjectRepositoryError(e) => e.into(),
        }
    }
}

impl From<FormRepositoryError> for AppError {
    fn from(error: FormRepositoryError) -> Self {
        match error {
            FormRepositoryError::InternalError(e) => e.into(),
        }
    }
}

impl From<InvitationRepositoryError> for AppError {
    fn from(error: InvitationRepositoryError) -> AppError {
        match error {
            InvitationRepositoryError::InternalError(e) => e.into(),
        }
    }
}

impl From<NewsRepositoryError> for AppError {
    fn from(error: NewsRepositoryError) -> AppError {
        match error {
            NewsRepositoryError::InternalError(e) => e.into(),
        }
    }
}

impl From<ProjectRepositoryError> for AppError {
    fn from(error: ProjectRepositoryError) -> AppError {
        match error {
            ProjectRepositoryError::InternalError(e) => e.into(),
        }
    }
}

impl From<UserRepositoryError> for AppError {
    fn from(error: UserRepositoryError) -> AppError {
        match error {
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
            UserRepositoryError::InternalError(e) => e.into(),
        }
    }
}

impl From<FirebaseUserRepositoryError> for AppError {
    fn from(error: FirebaseUserRepositoryError) -> AppError {
        match error {
            FirebaseUserRepositoryError::EmailExists(_) => AppError::new(
                StatusCode::BAD_REQUEST,
                // メールアドレスが既に使われていることを外に出さない
                "user/bad-credential".to_string(),
                "Bad credential".to_string(),
            ),
            FirebaseUserRepositoryError::InternalError(e) => e.into(),
        }
    }
}

impl From<ProjectError> for AppError {
    fn from(error: ProjectError) -> AppError {
        match error {
            ProjectError::AlreadyOwnerOrSubOwner => AppError::new(
                StatusCode::CONFLICT,
                "project/already-owner-or-sub-owner".to_string(),
                error.to_string(),
            ),
        }
    }
}

impl From<InvitationError> for AppError {
    fn from(error: InvitationError) -> AppError {
        match error {
            InvitationError::AlreadyUsed => AppError::new(
                StatusCode::BAD_REQUEST,
                "invitation/already-used".to_string(),
                error.to_string(),
            ),
            InvitationError::InviterAndReceiverAreSame => AppError::new(
                StatusCode::BAD_REQUEST,
                "invitation/inviter-and-receiver-are-same".to_string(),
                error.to_string(),
            ),
        }
    }
}

impl From<InvitationIdError> for AppError {
    fn from(error: InvitationIdError) -> AppError {
        match error {
            InvitationIdError::InvalidUuid => AppError::new(
                StatusCode::BAD_REQUEST,
                "invitation/invalid-uuid".to_string(),
                error.to_string(),
            ),
        }
    }
}

impl From<FormIdError> for AppError {
    fn from(error: FormIdError) -> Self {
        match error {
            FormIdError::InvalidUuid => AppError::new(
                StatusCode::BAD_REQUEST,
                "form/invalid-uuid".to_string(),
                error.to_string(),
            ),
        }
    }
}

impl From<NewsIdError> for AppError {
    fn from(error: NewsIdError) -> AppError {
        match error {
            NewsIdError::InvalidUuid => AppError::new(
                StatusCode::BAD_REQUEST,
                "news/invalid-uuid".to_string(),
                error.to_string(),
            ),
        }
    }
}

impl From<ProjectIdError> for AppError {
    fn from(error: ProjectIdError) -> AppError {
        match error {
            ProjectIdError::InvalidUuid => AppError::new(
                StatusCode::BAD_REQUEST,
                "project/invalid-uuid".to_string(),
                error.to_string(),
            ),
        }
    }
}

impl From<EmailError> for AppError {
    fn from(error: EmailError) -> AppError {
        match error {
            EmailError::InvalidFormat => AppError::new(
                StatusCode::BAD_REQUEST,
                "email/invalid-format".to_string(),
                error.to_string(),
            ),
            EmailError::InvalidDomain => AppError::new(
                StatusCode::BAD_GATEWAY,
                "email/invalid-domain".to_string(),
                error.to_string(),
            ),
        }
    }
}

impl From<DateTimeError> for AppError {
    fn from(error: DateTimeError) -> Self {
        match error {
            DateTimeError::InvalidFormat => AppError::new(
                StatusCode::BAD_REQUEST,
                "datetime/invalid-format".to_string(),
                error.to_string(),
            ),
        }
    }
}

impl From<PermissionDeniedError> for AppError {
    fn from(error: PermissionDeniedError) -> AppError {
        AppError::new(
            StatusCode::FORBIDDEN,
            "permission-denied".to_string(),
            error.to_string(),
        )
    }
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> AppError {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal-error".to_string(),
            error.to_string(),
        )
    }
}
