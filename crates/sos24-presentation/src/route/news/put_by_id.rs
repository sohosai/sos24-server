use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};

use serde::Deserialize;
use sos24_use_case::context::Context;
use sos24_use_case::news::use_case::update::UpdateNewsCommand;

use crate::error::AppError;
use crate::module::Modules;
use crate::route::project::{ProjectAttributes, ProjectCategories};

#[derive(Debug, Deserialize)]
pub struct UpdateNews {
    title: String,
    body: String,
    attachments: Vec<String>,
    categories: ProjectCategories,
    attributes: ProjectAttributes,
}

impl From<UpdateNews> for UpdateNewsCommand {
    fn from(news: UpdateNews) -> Self {
        UpdateNewsCommand {
            title: news.title,
            body: news.body,
            attachments: news.attachments,
            categories: news.categories.into(),
            attributes: news.attributes.into(),
        }
    }
}

pub async fn handle(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(news_data): Json<UpdateNews>,
) -> Result<impl IntoResponse, AppError> {
    let news = UpdateNewsCommand::from(news_data);
    let res = modules.news_use_case().update(&ctx, id, news).await;
    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(err) => {
            tracing::error!("Failed to update news: {err:?}");
            Err(err.into())
        }
    }
}
