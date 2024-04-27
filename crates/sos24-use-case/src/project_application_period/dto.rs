use sos24_domain::entity::project_application_period::ProjectApplicationPeriod;

use crate::FromEntity;

#[derive(Debug)]
pub struct ProjectApplicationPeriodDto {
    pub start_at: String,
    pub end_at: String,
}

impl FromEntity for ProjectApplicationPeriodDto {
    type Entity = ProjectApplicationPeriod;
    fn from_entity(entity: Self::Entity) -> Self {
        Self {
            start_at: entity.start_at().to_rfc3339(),
            end_at: entity.end_at().to_rfc3339(),
        }
    }
}
