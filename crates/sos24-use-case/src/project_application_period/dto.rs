use sos24_domain::entity::project_application_period::ProjectApplicationPeriod;

#[derive(Debug)]
pub struct ProjectApplicationPeriodDto {
    pub start_at: String,
    pub end_at: String,
}

impl From<ProjectApplicationPeriod> for ProjectApplicationPeriodDto {
    fn from(entity: ProjectApplicationPeriod) -> Self {
        Self {
            start_at: entity.start_at().to_rfc3339(),
            end_at: entity.end_at().to_rfc3339(),
        }
    }
}
