use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

use serde::{Deserialize, Serialize};
use sos24_use_case::context::Context;
use sos24_use_case::form::dto::NewFormItemDto;
use sos24_use_case::form::use_case::create::CreateFormCommand;
use sos24_use_case::project::dto::{ProjectAttributesDto, ProjectCategoriesDto};

use crate::error::AppError;
use crate::module::Modules;
use crate::route::project::{ProjectAttributes, ProjectCategories};

use super::NewFormItem;

#[derive(Debug, Deserialize)]
pub struct CreateForm {
    title: String,
    description: String,
    starts_at: String,
    ends_at: String,
    categories: ProjectCategories,
    attributes: ProjectAttributes,
    items: Vec<NewFormItem>,
    attachments: Vec<String>,
}

impl From<CreateForm> for CreateFormCommand {
    fn from(form: CreateForm) -> Self {
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

#[derive(Debug, Serialize)]
pub struct CreatedForm {
    pub id: String,
}

pub async fn handle(
    State(module): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(form_data): Json<CreateForm>,
) -> Result<impl IntoResponse, AppError> {
    let form = CreateFormCommand::from(form_data);
    let res = module.form_use_case().create(&ctx, form).await;
    match res {
        Ok(id) => Ok((StatusCode::CREATED, Json(CreatedForm { id }))),
        Err(err) => {
            tracing::error!("Failed to create form: {err:?}");
            Err(err.into())
        }
    }
}
