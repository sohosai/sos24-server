use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};

use serde::{Deserialize, Serialize};
use sos24_use_case::{context::Context, form_answer::dto::FormAnswerDto};

use crate::{error::AppError, module::Modules};

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
    State(modules): State<Arc<Modules>>,
    Query(query): Query<FormAnswerQuery>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let res = match (query.project_id, query.form_id) {
        (None, None) => modules.form_answer_use_case().list(&ctx).await,
        (Some(project_id), None) => {
            modules
                .form_answer_use_case()
                .find_by_project_id(&ctx, project_id)
                .await
        }
        (None, Some(form_id)) => {
            modules
                .form_answer_use_case()
                .find_by_form_id(&ctx, form_id)
                .await
        }
        _ => {
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                "form-answer/invalid-query".to_string(),
                "Invalid query".to_string(),
            ));
        }
    };

    match res {
        Ok(raw_form_answer_list) => {
            let form_answer_list: Vec<_> = raw_form_answer_list
                .into_iter()
                .map(FormAnswerSummary::from)
                .collect();
            Ok((StatusCode::OK, Json(form_answer_list)))
        }
        Err(err) => {
            tracing::error!("Failed to list form answer: {err:?}");
            Err(err.into())
        }
    }
}
