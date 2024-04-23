use serde::Serialize;

use sos24_use_case::dto::project_application_period::ProjectApplicationPeriodDto;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ProjectApplicationPeriod {
    #[schema(format = "date-time")]
    start_at: String,
    #[schema(format = "date-time")]
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
