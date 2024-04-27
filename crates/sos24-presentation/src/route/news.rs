use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};

use sos24_use_case::dto::news::CreateNewsDto;

use crate::context::Context;
use crate::error::AppError;
use crate::model::news::{
    ConvertToUpdateNewsDto, CreateNews, CreatedNews, News, NewsSummary, UpdateNews,
};
use crate::module::Modules;

/// お知らせ一覧の取得
#[utoipa::path(
    get,
    path = "/news",
    operation_id = "getNews",
    tag = "news",
    responses(
        (status = 200, description = "OK", body = Vec<NewsSummary>),
        (status = 401, description = "Unauthorized", body = Error),
        (status = 500, description = "Internal Server Error", body = Error),
    ),
    security(("jwt_token" = [])),
)]
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

/// お知らせの作成
#[utoipa::path(
    post,
    path = "/news",
    operation_id = "postNews",
    tag = "news",
    request_body(content = CreateNews),
    responses(
        (status = 201, description = "Created", body = CreatedNews),
        (status = 400, description = "Bad Request", body = Error),
        (status = 401, description = "Unauthorized", body = Error),
        (status = 403, description = "Forbidden", body = Error),
        (status = 422, description = "Unprocessable Entity", body = Error),
        (status = 500, description = "Internal Server Error", body = Error),
    ),
    security(("jwt_token" = [])),
)]
pub async fn handle_post(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(raw_news): Json<CreateNews>,
) -> Result<impl IntoResponse, AppError> {
    let news = CreateNewsDto::from(raw_news);
    let res = modules.news_use_case().create(&ctx, news).await;
    res.map(|id| (StatusCode::CREATED, Json(CreatedNews { id })))
        .map_err(|err| {
            tracing::error!("Failed to create news: {err:?}");
            err.into()
        })
}

/// 特定のIDのお知らせの取得
#[utoipa::path(
    get,
    path = "/news/{news_id}",
    operation_id = "getNewsById",
    tag = "news",
    params(("news_id" = String, Path, format="uuid")),
    responses(
        (status = 200, description = "OK", body = News),
        (status = 401, description = "Unauthorized", body = Error),
        (status = 404, description = "Not Found", body = Error),
        (status = 500, description = "Internal Server Error", body = Error),
    ),
    security(("jwt_token" = [])),
)]
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

/// 特定のIDのお知らせの削除
#[utoipa::path(
    delete,
    path = "/news/{news_id}",
    operation_id = "deleteNewsById",
    tag = "news",
    params(("news_id" = String, Path, format="uuid")),
    responses(
        (status = 200, description = "OK"),
        (status = 401, description = "Unauthorized", body = Error),
        (status = 403, description = "Forbidden", body = Error),
        (status = 404, description = "Not Found", body = Error),
        (status = 500, description = "Internal Server Error", body = Error),
    ),
    security(("jwt_token" = [])),
)]
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

/// 特定のIDのお知らせを更新
#[utoipa::path(
    put,
    path = "/news/{news_id}",
    operation_id = "putNewsById",
    tag = "news",
    params(("news_id" = String, Path, format="uuid")),
    request_body(content = UpdateNews),
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = Error),
        (status = 401, description = "Unauthorized", body = Error),
        (status = 403, description = "Forbidden", body = Error),
        (status = 404, description = "Not Found", body = Error),
        (status = 422, description = "Unprocessable Entity", body = Error),
        (status = 500, description = "Internal Server Error", body = Error),
    ),
    security(("jwt_token" = [])),
)]
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
