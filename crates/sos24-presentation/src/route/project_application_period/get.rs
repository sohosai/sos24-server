use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use sos24_use_case::project_application_period::dto::ProjectApplicationPeriodDto;

use crate::error::AppError;
use crate::module::Modules;

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

pub async fn handle(State(modules): State<Arc<Modules>>) -> Result<impl IntoResponse, AppError> {
    match modules.project_use_case().get_project_application_period() {
        Ok(raw_project_application_period) => {
            let project_application_period =
                ProjectApplicationPeriod::from(raw_project_application_period);
            Ok((StatusCode::OK, Json(project_application_period)))
        }
        Err(err) => {
            tracing::error!("Failed to get project application period: {err:?}");
            Err(err.into())
        }
    }
}
