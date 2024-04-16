use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};

use serde::Serialize;
use sos24_use_case::context::Context;
use sos24_use_case::file::use_case::find_by_id::FileDto;

use crate::{error::AppError, module::Modules};

#[derive(Debug, Serialize)]
pub struct File {
    pub id: String,
    pub url: String,
    pub name: String,
    pub owner: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl From<FileDto> for File {
    fn from(file: FileDto) -> Self {
        File {
            id: file.id,
            url: file.url,
            name: file.filename,
            owner: file.owner,
            created_at: file.created_at.to_rfc3339(),
            updated_at: file.updated_at.to_rfc3339(),
            deleted_at: file.deleted_at.map(|it| it.to_rfc3339()),
        }
    }
}

pub async fn handle(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules
        .file_use_case()
        .find_by_id(&ctx, modules.config().s3_bucket_name.clone(), id)
        .await;
    match res {
        Ok(raw_file) => Ok((StatusCode::OK, Json(File::from(raw_file)))),
        Err(err) => {
            tracing::error!("Failed to find file: {err}");
            Err(err.into())
        }
    }
}
