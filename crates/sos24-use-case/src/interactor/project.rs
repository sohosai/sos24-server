use std::sync::Arc;

use thiserror::Error;

use sos24_domain::entity::user::UserId;
use sos24_domain::repository::user::UserRepositoryError;
use sos24_domain::{
    entity::{
        permission::PermissionDeniedError,
        project::{BoundedStringError, ProjectId, ProjectIdError},
        project_application_period::ProjectApplicationPeriod,
    },
    repository::{project::ProjectRepositoryError, Repositories},
};

use crate::context::ContextError;

pub mod create;
pub mod delete_by_id;
pub mod find_by_id;
pub mod find_owned;
pub mod get_project_application_period;
pub mod list;
pub mod update;

#[derive(Debug, Error)]
pub enum ProjectUseCaseError {
    #[error("Project not found: {0:?}")]
    NotFound(ProjectId),
    #[error("User already owned project: {0:?}")]
    AlreadyOwnedProject(ProjectId),
    #[error("Project applications are not being accepted")]
    ApplicationsNotAccepted,
    #[error("User not found: {0:?}")]
    UserNotFound(UserId),

    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
    #[error(transparent)]
    BoundedStringError(#[from] BoundedStringError),
    #[error(transparent)]
    ContextError(#[from] ContextError),
    #[error(transparent)]
    ProjectRepositoryError(#[from] ProjectRepositoryError),
    #[error(transparent)]
    ProjectIdError(#[from] ProjectIdError),
    #[error(transparent)]
    PermissionDeniedError(#[from] PermissionDeniedError),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

pub struct ProjectUseCase<R: Repositories> {
    repositories: Arc<R>,
    project_application_period: ProjectApplicationPeriod, // TODO
}

impl<R: Repositories> ProjectUseCase<R> {
    pub fn new(repositories: Arc<R>, project_application_period: ProjectApplicationPeriod) -> Self {
        Self {
            repositories,
            project_application_period,
        }
    }

    // for test purpose only
    #[cfg(test)]
    pub(crate) fn new_for_test(repositories: R) -> Self {
        let application_period = ProjectApplicationPeriod::new(
            chrono::Utc::now()
                .checked_sub_days(chrono::Days::new(1))
                .unwrap()
                .to_rfc3339(),
            chrono::Utc::now()
                .checked_add_days(chrono::Days::new(1))
                .unwrap()
                .to_rfc3339(),
        );
        ProjectUseCase::new(Arc::new(repositories), application_period)
    }
}
