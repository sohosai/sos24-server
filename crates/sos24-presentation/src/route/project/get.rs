use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

use serde::Serialize;
use sos24_use_case::context::Context;
use sos24_use_case::project::dto::ProjectWithOwnersDto;

use crate::error::AppError;
use crate::module::Modules;

use super::{ProjectAttributes, ProjectCategory};

#[derive(Serialize)]
pub struct ProjectSummary {
    id: String,
    index: i32,
    title: String,
    category: ProjectCategory,
    attributes: ProjectAttributes,
    owner_id: String,
    owner_name: String,
    owner_email: String,
}

impl From<ProjectWithOwnersDto> for ProjectSummary {
    fn from(dto: ProjectWithOwnersDto) -> Self {
        ProjectSummary {
            id: dto.project.id,
            index: dto.project.index,
            title: dto.project.title,
            category: ProjectCategory::from(dto.project.category),
            attributes: ProjectAttributes::from(dto.project.attributes),
            owner_id: dto.owner.id,
            owner_name: dto.owner.name,
            owner_email: dto.owner.email,
        }
    }
}

pub async fn handle(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.project_use_case().list(&ctx).await;
    match res {
        Ok(raw_project_list) => {
            let project_list: Vec<_> = raw_project_list
                .into_iter()
                .map(ProjectSummary::from)
                .collect();
            Ok((StatusCode::OK, Json(project_list)))
        }
        Err(err) => {
            tracing::error!("Failed to list project: {err:?}");
            Err(err.into())
        }
    }
}
