use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

use serde::{Deserialize, Serialize};
use sos24_use_case::{
    context::Context,
    form_answer::{dto::FormAnswerItemDto, use_case::create::CreateFormAnswerCommand},
};

use crate::{error::AppError, module::Modules};

use super::FormAnswerItem;

#[derive(Debug, Deserialize)]
pub struct CreateFormAnswer {
    pub form_id: String,
    pub items: Vec<FormAnswerItem>,
}

impl From<CreateFormAnswer> for CreateFormAnswerCommand {
    fn from(raw: CreateFormAnswer) -> Self {
        CreateFormAnswerCommand {
            form_id: raw.form_id,
            items: raw.items.into_iter().map(FormAnswerItemDto::from).collect(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreatedFormAnswer {
    pub id: String,
}

pub async fn handle(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(form_answer_data): Json<CreateFormAnswer>,
) -> Result<impl IntoResponse, AppError> {
    let form_answer = CreateFormAnswerCommand::from(form_answer_data);
    let res = modules
        .form_answer_use_case()
        .create(&ctx, form_answer)
        .await;
    match res {
        Ok(id) => Ok((StatusCode::CREATED, Json(CreatedFormAnswer { id }))),
        Err(err) => {
            tracing::error!("Failed to create form answer: {err:?}");
            Err(err.into())
        }
    }
}
