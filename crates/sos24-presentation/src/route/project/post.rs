use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

use serde::{Deserialize, Serialize};
use sos24_use_case::project::dto::ProjectAttributesDto;
use sos24_use_case::project::use_case::create::CreateProjectCommand;
use sos24_use_case::{context::Context, project::dto::ProjectCategoryDto};

use crate::error::AppError;
use crate::module::Modules;

use super::{ProjectAttributes, ProjectCategory};

#[derive(Debug, Deserialize)]
pub struct CreateProject {
    title: String,
    kana_title: String,
    group_name: String,
    kana_group_name: String,
    category: ProjectCategory,
    attributes: ProjectAttributes,
}

impl From<CreateProject> for CreateProjectCommand {
    fn from(data: CreateProject) -> Self {
        CreateProjectCommand {
            title: data.title,
            kana_title: data.kana_title,
            group_name: data.group_name,
            kana_group_name: data.kana_group_name,
            category: ProjectCategoryDto::from(data.category),
            attributes: ProjectAttributesDto::from(data.attributes),
        }
    }
}

#[derive(Serialize)]
pub struct CreatedProject {
    pub id: String,
}

pub async fn handle(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(project_data): Json<CreateProject>,
) -> Result<impl IntoResponse, AppError> {
    let project = CreateProjectCommand::from(project_data);
    let res = modules.project_use_case().create(&ctx, project).await;
    match res {
        Ok(id) => Ok((StatusCode::CREATED, Json(CreatedProject { id }))),
        Err(err) => {
            tracing::error!("Failed to create project: {err:?}");
            Err(err.into())
        }
    }
}
