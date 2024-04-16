use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};

use serde::Serialize;
use sos24_use_case::context::Context;
use sos24_use_case::file::use_case::list::FileSummaryDto;

use crate::{error::AppError, module::Modules};

#[derive(Debug, Serialize)]
pub struct FileSummary {
    pub id: String,
    pub filename: String,
    pub owner: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl From<FileSummaryDto> for FileSummary {
    fn from(file: FileSummaryDto) -> Self {
        FileSummary {
            id: file.id,
            filename: file.filename,
            owner: file.owner,
            created_at: file.created_at.to_rfc3339(),
            updated_at: file.updated_at.to_rfc3339(),
            deleted_at: file.deleted_at.map(|it| it.to_rfc3339()),
        }
    }
}

pub async fn handle(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.file_use_case().list(&ctx).await;
    match res {
        Ok(raw_file_list) => {
            let file_list: Vec<_> = raw_file_list.into_iter().map(FileSummary::from).collect();
            Ok((StatusCode::OK, Json(file_list)))
        }
        Err(err) => {
            tracing::error!("Failed to list files: {err}");
            Err(err.into())
        }
    }
}
