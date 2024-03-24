use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use sos24_domain::entity::actor::Actor;
use sos24_use_case::context::Context;
use sos24_use_case::dto::news_attachment::CreateNewsAttachmentDto;

use crate::error::AppError;
use crate::model::news_attachment::{CreateNewsAttachment, NewsAttachment};
use crate::module::Modules;

pub async fn handle_get(
    State(modules): State<Arc<Modules>>,
    Extension(actor): Extension<Actor>,
) -> Result<impl IntoResponse, AppError> {
    let raw_news_list = modules
        .news_attachment_use_case()
        .list(modules.config().s3_bucket_name.clone(), &actor)
        .await;
    raw_news_list
        .map(|raw_news_list| {
            let news_list: Vec<NewsAttachment> = raw_news_list
                .into_iter()
                .map(NewsAttachment::from)
                .collect();
            (StatusCode::OK, Json(news_list))
        })
        .map_err(|err| {
            tracing::error!("Failed to list news attachment: {err}");
            err.into()
        })
}

pub async fn handle_post(
    State(modules): State<Arc<Modules>>,
    Json(raw_news_attachment): Json<CreateNewsAttachment>,
) -> Result<impl IntoResponse, AppError> {
    let news_attachment = CreateNewsAttachmentDto::from(raw_news_attachment);
    let res = modules
        .news_attachment_use_case()
        .create(
            modules.config().s3_bucket_name.clone(),
            // ToDo: そもそもkey_prefixは必要なのか？
            "test".to_string(),
            news_attachment,
        )
        .await;
    res.map(|_| StatusCode::CREATED).map_err(|err| {
        tracing::error!("Failed to create news attachment: {err}");
        err.into()
    })
}

pub async fn handle_get_id(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let raw_news_attachment = modules
        .news_attachment_use_case()
        .find_by_id(&ctx, modules.config().s3_bucket_name.clone(), id)
        .await;
    match raw_news_attachment {
        Ok(raw_news_attachment) => Ok((
            StatusCode::OK,
            Json(NewsAttachment::from(raw_news_attachment)),
        )),
        Err(err) => {
            tracing::error!("Failed to find news attachment: {err}");
            Err(err.into())
        }
    }
}

pub async fn handle_delete_id(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(actor): Extension<Actor>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules
        .news_attachment_use_case()
        .delete_by_id(&actor, id)
        .await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to delete news attachment: {err}");
        err.into()
    })
}
