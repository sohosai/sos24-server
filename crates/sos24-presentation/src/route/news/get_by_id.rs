use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};

use sos24_use_case::context::Context;

use crate::error::AppError;
use crate::module::Modules;

use super::News;

pub async fn handle(
    Path(id): Path<String>,
    Extension(ctx): Extension<Context>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.news_use_case().find_by_id(&ctx, id).await;
    match res {
        Ok(raw_news) => Ok((StatusCode::OK, Json(News::from(raw_news)))),
        Err(err) => {
            tracing::error!("Failed to find news: {err:?}");
            Err(err.into())
        }
    }
}
