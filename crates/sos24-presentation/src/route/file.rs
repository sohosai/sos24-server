use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use sos24_domain::entity::actor::Actor;
use sos24_use_case::dto::file::CreateFileDto;

use crate::error::AppError;
use crate::model::file::{CreateFile, File};
use crate::module::Modules;

pub async fn handle_get(
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, AppError> {
    let raw_file_list = modules
        .file_use_case()
        .list(modules.config().s3_bucket_name.clone())
        .await;
    raw_file_list
        .map(|raw_file_list| {
            let file_list: Vec<File> = raw_file_list.into_iter().map(File::from).collect();
            (StatusCode::OK, Json(file_list))
        })
        .map_err(|err| {
            tracing::error!("Failed to list files: {err}");
            err.into()
        })
}

pub async fn handle_post(
    State(modules): State<Arc<Modules>>,
    Json(raw_file): Json<CreateFile>,
) -> Result<impl IntoResponse, AppError> {
    let file = CreateFileDto::from(raw_file);
    let res = modules
        .file_use_case()
        .create(
            modules.config().s3_bucket_name.clone(),
            "user-upload".to_string(),
            file,
        )
        .await;
    res.map(|_| StatusCode::CREATED).map_err(|err| {
        tracing::error!("Failed to create file: {err}");
        err.into()
    })
}

pub async fn handle_get_id(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, AppError> {
    let raw_file = modules
        .file_use_case()
        .find_by_id(modules.config().s3_bucket_name.clone(), id)
        .await;
    match raw_file {
        Ok(raw_file) => Ok((StatusCode::OK, Json(File::from(raw_file)))),
        Err(err) => {
            tracing::error!("Failed to find file: {err}");
            Err(err.into())
        }
    }
}

pub async fn handle_delete_id(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(actor): Extension<Actor>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.file_use_case().delete_by_id(&actor, id).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to delete file: {err}");
        err.into()
    })
}