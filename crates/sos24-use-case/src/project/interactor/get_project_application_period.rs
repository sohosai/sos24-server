use sos24_domain::repository::Repositories;

use crate::{
    project::{dto::ProjectApplicationPeriodDto, ProjectUseCase, ProjectUseCaseError},
    shared::adapter::Adapters,
};

impl<R: Repositories, A: Adapters> ProjectUseCase<R, A> {
    pub async fn get_project_application_period(
        &self,
    ) -> Result<ProjectApplicationPeriodDto, ProjectUseCaseError> {
        Ok(ProjectApplicationPeriodDto::from(
            self.project_application_period.clone(),
        ))
    }
}
