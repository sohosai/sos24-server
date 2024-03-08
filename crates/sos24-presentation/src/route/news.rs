use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use sos24_domain::entity::actor::Actor;
use sos24_use_case::dto::news::CreateNewsDto;
use sos24_use_case::error::news::NewsError;
use sos24_use_case::error::UseCaseError;

use crate::model::news::{ConvertToUpdateNewsDto, CreateNews, News, UpdateNews};
use crate::module::Modules;

use super::ToStatusCode;

impl ToStatusCode for UseCaseError<NewsError> {
    fn status_code(&self) -> StatusCode {
        match self {
            UseCaseError::UseCase(NewsError::NotFound(_)) => StatusCode::NOT_FOUND,
            UseCaseError::UseCase(NewsError::InvalidNewsId(_)) => StatusCode::BAD_REQUEST,
            UseCaseError::UseCase(NewsError::PermissionDenied(_)) => StatusCode::NOT_FOUND,
            UseCaseError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub async fn handle_get(
    State(modules): State<Arc<Modules>>,
    Extension(actor): Extension<Actor>,
) -> Result<impl IntoResponse, StatusCode> {
    let raw_news_list = modules.news_use_case().list(&actor).await;
    raw_news_list
        .map(|raw_news_list| {
            let news_list: Vec<News> = raw_news_list.into_iter().map(News::from).collect();
            (StatusCode::OK, Json(news_list))
        })
        .map_err(|err| {
            tracing::error!("Failed to list news: {err}");
            err.status_code()
        })
}

pub async fn handle_post(
    State(modules): State<Arc<Modules>>,
    Extension(actor): Extension<Actor>,
    Json(raw_news): Json<CreateNews>,
) -> Result<impl IntoResponse, StatusCode> {
    let news = CreateNewsDto::from(raw_news);
    let res = modules.news_use_case().create(&actor, news).await;
    res.map(|_| StatusCode::CREATED).map_err(|err| {
        tracing::error!("Failed to create news: {err}");
        err.status_code()
    })
}

pub async fn handle_get_id(
    Path(id): Path<String>,
    Extension(actor): Extension<Actor>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let raw_news = modules.news_use_case().find_by_id(&actor, id).await;
    match raw_news {
        Ok(raw_news) => Ok((StatusCode::OK, Json(News::from(raw_news)))),
        Err(err) => {
            tracing::error!("Failed to find news: {err}");
            Err(err.status_code())
        }
    }
}

pub async fn handle_delete_id(
    Path(id): Path<String>,
    Extension(actor): Extension<Actor>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.news_use_case().delete_by_id(&actor, id).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to delete news: {err}");
        err.status_code()
    })
}

pub async fn handle_put_id(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(actor): Extension<Actor>,
    Json(raw_news): Json<UpdateNews>,
) -> Result<impl IntoResponse, StatusCode> {
    let news = (id, raw_news).to_update_news_dto();
    let res = modules.news_use_case().update(&actor, news).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to update news: {err}");
        err.status_code()
    })
}
