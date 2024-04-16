use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};

use serde::{Deserialize, Serialize};
use sos24_use_case::{context::Context, form_answer::dto::FormAnswerDto};

use crate::{error::AppError, module::Modules};

use super::FormAnswer;

#[derive(Debug, Deserialize)]
pub struct FormAnswerQuery {
    pub project_id: Option<String>,
    pub form_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FormAnswerSummary {
    id: String,
    project_id: String,
    project_title: String,
    form_id: String,
    form_title: String,
    updated_at: String,
}

impl From<FormAnswerDto> for FormAnswerSummary {
    fn from(form_answer_dto: FormAnswerDto) -> Self {
        FormAnswerSummary {
            id: form_answer_dto.id,
            project_id: form_answer_dto.project_id,
            project_title: form_answer_dto.project_title,
            form_id: form_answer_dto.form_id,
            form_title: form_answer_dto.form_title,
            updated_at: form_answer_dto.updated_at.to_rfc3339(),
        }
    }
}

pub async fn handle(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let raw_form = modules.form_answer_use_case().find_by_id(&ctx, id).await;
    match raw_form {
        Ok(raw_form) => Ok((StatusCode::OK, Json(FormAnswer::from(raw_form)))),
        Err(err) => {
            tracing::error!("Failed to find form answer by id: {err:?}");
            Err(err.into())
        }
    }
}
