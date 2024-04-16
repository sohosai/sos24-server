use std::sync::Arc;

use axum::extract::Path;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

use serde::Deserialize;
use sos24_use_case::context::Context;
use sos24_use_case::project::dto::{ProjectAttributesDto, ProjectCategoryDto};
use sos24_use_case::project::use_case::update::UpdateProjectCommand;

use crate::error::AppError;
use crate::module::Modules;

use super::{ProjectAttributes, ProjectCategory};

#[derive(Debug, Deserialize)]
pub struct UpdateProject {
    title: String,
    kana_title: String,
    group_name: String,
    kana_group_name: String,
    category: ProjectCategory,
    attributes: ProjectAttributes,
    remarks: Option<String>,
}

impl From<UpdateProject> for UpdateProjectCommand {
    fn from(project: UpdateProject) -> Self {
        UpdateProjectCommand {
            title: project.title,
            kana_title: project.kana_title,
            group_name: project.group_name,
            kana_group_name: project.kana_group_name,
            category: ProjectCategoryDto::from(project.category),
            attributes: ProjectAttributesDto::from(project.attributes),
            remarks: project.remarks,
        }
    }
}

pub async fn handle(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(project_data): Json<UpdateProject>,
) -> Result<impl IntoResponse, AppError> {
    let project = UpdateProjectCommand::from(project_data);
    let res = modules.project_use_case().update(&ctx, id, project).await;
    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(err) => {
            tracing::error!("Failed to update project: {err:?}");
            Err(err.into())
        }
    }
}
