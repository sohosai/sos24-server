use std::sync::Arc;

use axum::extract::Path;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

use serde::Deserialize;
use sos24_use_case::context::Context;
use sos24_use_case::form::dto::NewFormItemDto;
use sos24_use_case::form::use_case::update::UpdateFormCommand;
use sos24_use_case::project::dto::{ProjectAttributesDto, ProjectCategoriesDto};

use crate::error::AppError;
use crate::module::Modules;
use crate::route::project::{ProjectAttributes, ProjectCategories};

use super::NewFormItem;

#[derive(Debug, Deserialize)]
pub struct UpdateForm {
    pub title: String,
    pub description: String,
    pub starts_at: String,
    pub ends_at: String,
    pub categories: ProjectCategories,
    pub attributes: ProjectAttributes,
    pub items: Vec<NewFormItem>,
    pub attachments: Vec<String>,
}

impl From<UpdateForm> for UpdateFormCommand {
    fn from(form: UpdateForm) -> Self {
        Self {
            title: form.title,
            description: form.description,
            starts_at: form.starts_at,
            ends_at: form.ends_at,
            categories: ProjectCategoriesDto::from(form.categories),
            attributes: ProjectAttributesDto::from(form.attributes),
            items: form.items.into_iter().map(NewFormItemDto::from).collect(),
            attachments: form.attachments,
        }
    }
}

pub async fn handle(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(form_data): Json<UpdateForm>,
) -> Result<impl IntoResponse, AppError> {
    let form = UpdateFormCommand::from(form_data);
    let res = modules.form_use_case().update(&ctx, id, form).await;
    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(err) => {
            tracing::error!("Failed to update form: {err:?}");
            Err(err.into())
        }
    }
}
