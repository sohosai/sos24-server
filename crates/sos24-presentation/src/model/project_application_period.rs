use serde::Serialize;

use sos24_use_case::dto::project_application_period::ProjectApplicationPeriodDto;

#[derive(Debug, Serialize)]
pub struct ProjectApplicationPeriod {
    start_at: String,
    end_at: String,
}

impl From<ProjectApplicationPeriodDto> for ProjectApplicationPeriod {
    fn from(dto: ProjectApplicationPeriodDto) -> Self {
        Self {
            start_at: dto.start_at,
            end_at: dto.end_at,
        }
    }
}
