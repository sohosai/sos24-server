use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};

use serde::{Deserialize, Serialize};
use sos24_use_case::context::Context;
use sos24_use_case::news::use_case::create::CreateNewsCommand;
use sos24_use_case::project::dto::{ProjectAttributesDto, ProjectCategoriesDto};

use crate::error::AppError;
use crate::module::Modules;
use crate::route::project::{ProjectAttributes, ProjectCategories};

#[derive(Debug, Deserialize)]
pub struct CreateNews {
    title: String,
    body: String,
    attachments: Vec<String>,
    categories: ProjectCategories,
    attributes: ProjectAttributes,
}

impl From<CreateNews> for CreateNewsCommand {
    fn from(news: CreateNews) -> Self {
        CreateNewsCommand {
            title: news.title,
            body: news.body,
            attachments: news.attachments,
            categories: ProjectCategoriesDto::from(news.categories),
            attributes: ProjectAttributesDto::from(news.attributes),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreatedNews {
    pub id: String,
}

pub async fn handle(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(news_data): Json<CreateNews>,
) -> Result<impl IntoResponse, AppError> {
    let news = CreateNewsCommand::from(news_data);
    let res = modules.news_use_case().create(&ctx, news).await;
    match res {
        Ok(id) => Ok((StatusCode::CREATED, Json(CreatedNews { id }))),
        Err(err) => {
            tracing::error!("Failed to create news: {err:?}");
            Err(err.into())
        }
    }
}
