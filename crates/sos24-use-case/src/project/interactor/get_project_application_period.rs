use sos24_domain::repository::Repositories;

use crate::{
    project::{ProjectUseCase, ProjectUseCaseError},
    project_application_period::dto::ProjectApplicationPeriodDto,
    FromEntity,
};

impl<R: Repositories> ProjectUseCase<R> {
    pub async fn get_project_application_period(
        &self,
    ) -> Result<ProjectApplicationPeriodDto, ProjectUseCaseError> {
        Ok(ProjectApplicationPeriodDto::from_entity(
            self.project_application_period.clone(),
        ))
    }
}
