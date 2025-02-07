use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::error::{AppError, ErrorResponse};
use crate::model::project_application_period::ProjectApplicationPeriod;
use crate::module::Modules;

/// 企画募集期間を取得する
#[utoipa::path(
    get,
    path = "/project-application-period",
    operation_id = "getProjectApplicationPeriod",
    tag = "projects",
    responses(
        (status = 200, description = "OK", body = ProjectApplicationPeriod),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse),
    ),
    security(()),
)]
pub async fn handle_get(
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, AppError> {
    match modules
        .project_use_case()
        .get_project_application_period()
        .await
    {
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
