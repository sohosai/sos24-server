use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Extension;

use percent_encoding::NON_ALPHANUMERIC;
use serde::Deserialize;
use sos24_use_case::context::Context;
use tokio_util::io::ReaderStream;

use crate::{error::AppError, module::Modules};

#[derive(Debug, Deserialize)]
pub struct ExportFileQuery {
    pub project_id: Option<String>,
    pub form_id: Option<String>,
}

pub async fn handle(
    State(modules): State<Arc<Modules>>,
    Query(query): Query<ExportFileQuery>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    fn archive_to_body(filename: String, body: Body) -> Result<impl IntoResponse, AppError> {
        let encoded_filename =
            percent_encoding::percent_encode(filename.as_bytes(), NON_ALPHANUMERIC);
        Response::builder()
            .header("Content-Type", "application/zip")
            .header(
                "Content-Disposition",
                format!(
                    "attachment; filename=\"{}\" filename*=UTF-8''{}",
                    filename, encoded_filename
                ),
            )
            .body(body)
            .map_err(|err| {
                tracing::error!("Failed to create response: {err:?}");
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "file/failed-to-create-response".to_string(),
                    format!("{err:?}"),
                )
            })
    }

    match (query.project_id, query.form_id) {
        (Some(project_id), None) => {
            let archive = modules
                .file_use_case()
                .export_by_owner_project(&ctx, modules.config().s3_bucket_name.clone(), project_id)
                .await
                .map_err(|err| {
                    tracing::error!("Failed to export file: {err:?}");
                    AppError::from(err)
                })?;
            archive_to_body(
                archive.filename,
                Body::from_stream(ReaderStream::new(archive.body)),
            )
        }
        (None, Some(form_id)) => {
            let archive = modules
                .file_use_case()
                .export_by_form_id(&ctx, modules.config().s3_bucket_name.clone(), form_id)
                .await
                .map_err(|err| {
                    tracing::error!("Failed to export file: {err:?}");
                    AppError::from(err)
                })?;
            archive_to_body(
                archive.filename,
                Body::from_stream(ReaderStream::new(archive.body)),
            )
        }
        _ => Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "file/invalid-query".to_string(),
            "Invalid query".to_string(),
        )),
    }
}
