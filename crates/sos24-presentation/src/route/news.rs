use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use sos24_use_case::context::Context;
use sos24_use_case::dto::news::CreateNewsDto;

use crate::error::AppError;
use crate::model::news::{ConvertToUpdateNewsDto, CreateNews, News, NewsSummary, UpdateNews};
use crate::module::Modules;

pub async fn handle_get(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let raw_news_list = modules.news_use_case().list(&ctx).await;
    raw_news_list
        .map(|raw_news_list| {
            let news_list: Vec<NewsSummary> =
                raw_news_list.into_iter().map(NewsSummary::from).collect();
            (StatusCode::OK, Json(news_list))
        })
        .map_err(|err| {
            tracing::error!("Failed to list news: {err:?}");
            err.into()
        })
}

pub async fn handle_post(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(raw_news): Json<CreateNews>,
) -> Result<impl IntoResponse, AppError> {
    let news = CreateNewsDto::from(raw_news);
    let res = modules.news_use_case().create(&ctx, news).await;
    res.map(|_| StatusCode::CREATED).map_err(|err| {
        tracing::error!("Failed to create news: {err:?}");
        err.into()
    })
}

pub async fn handle_get_id(
    Path(id): Path<String>,
    Extension(ctx): Extension<Context>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, AppError> {
    let raw_news = modules.news_use_case().find_by_id(&ctx, id).await;
    match raw_news {
        Ok(raw_news) => Ok((StatusCode::OK, Json(News::from(raw_news)))),
        Err(err) => {
            tracing::error!("Failed to find news: {err:?}");
            Err(err.into())
        }
    }
}

pub async fn handle_delete_id(
    Path(id): Path<String>,
    Extension(ctx): Extension<Context>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.news_use_case().delete_by_id(&ctx, id).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to delete news: {err:?}");
        err.into()
    })
}

pub async fn handle_put_id(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(raw_news): Json<UpdateNews>,
) -> Result<impl IntoResponse, AppError> {
    let news = (id, raw_news).to_update_news_dto();
    let res = modules.news_use_case().update(&ctx, news).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to update news: {err:?}");
        err.into()
    })
}
