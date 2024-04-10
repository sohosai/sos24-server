use sos24_domain::repository::Repositories;

use crate::dto::project_application_period::ProjectApplicationPeriodDto;
use crate::dto::FromEntity;
use crate::interactor::project::{ProjectUseCase, ProjectUseCaseError};

impl<R: Repositories> ProjectUseCase<R> {
    pub async fn get_project_application_period(
        &self,
    ) -> Result<ProjectApplicationPeriodDto, ProjectUseCaseError> {
        Ok(ProjectApplicationPeriodDto::from_entity(
            self.project_application_period.clone(),
        ))
    }
}
