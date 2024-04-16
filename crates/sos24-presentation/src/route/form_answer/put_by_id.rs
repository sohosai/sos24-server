use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};

use serde::{Deserialize, Serialize};
use sos24_use_case::{
    context::Context,
    form_answer::{dto::FormAnswerItemDto, use_case::update::UpdateFormAnswerCommand},
};

use crate::{error::AppError, module::Modules};

use super::FormAnswerItem;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFormAnswer {
    items: Vec<FormAnswerItem>,
}

impl From<UpdateFormAnswer> for UpdateFormAnswerCommand {
    fn from(raw: UpdateFormAnswer) -> Self {
        UpdateFormAnswerCommand {
            items: raw.items.into_iter().map(FormAnswerItemDto::from).collect(),
        }
    }
}

pub async fn handle(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(form_answer_data): Json<UpdateFormAnswer>,
) -> Result<impl IntoResponse, AppError> {
    let form_answer = UpdateFormAnswerCommand::from(form_answer_data);
    let res = modules
        .form_answer_use_case()
        .update(&ctx, id, form_answer)
        .await;
    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(err) => {
            tracing::error!("Failed to update form answer by id: {err:?}");
            Err(err.into())
        }
    }
}
