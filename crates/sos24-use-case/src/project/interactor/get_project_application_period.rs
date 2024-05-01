use sos24_domain::repository::Repositories;

use crate::project::{dto::ProjectApplicationPeriodDto, ProjectUseCase, ProjectUseCaseError};

impl<R: Repositories> ProjectUseCase<R> {
    pub async fn get_project_application_period(
        &self,
    ) -> Result<ProjectApplicationPeriodDto, ProjectUseCaseError> {
        Ok(ProjectApplicationPeriodDto::from(
            self.project_application_period.clone(),
        ))
    }
}
