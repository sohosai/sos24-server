use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};

use serde::Serialize;
use sos24_use_case::context::Context;
use sos24_use_case::news::dto::NewsDto;

use crate::error::AppError;
use crate::module::Modules;
use crate::route::project::{ProjectAttributes, ProjectCategories};

#[derive(Debug, Serialize)]
pub struct NewsSummary {
    id: String,
    title: String,
    categories: ProjectCategories,
    attributes: ProjectAttributes,
    updated_at: String,
}

impl From<NewsDto> for NewsSummary {
    fn from(news: NewsDto) -> Self {
        NewsSummary {
            id: news.id,
            title: news.title,
            categories: ProjectCategories::from(news.categories),
            attributes: ProjectAttributes::from(news.attributes),
            updated_at: news.updated_at.to_rfc3339(),
        }
    }
}

pub async fn handle(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.news_use_case().list(&ctx).await;
    match res {
        Ok(raw_news_list) => {
            let news_list: Vec<_> = raw_news_list.into_iter().map(NewsSummary::from).collect();
            Ok((StatusCode::OK, Json(news_list)))
        }
        Err(err) => {
            tracing::error!("Failed to list news: {err:?}");
            Err(err.into())
        }
    }
}
