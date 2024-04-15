use std::sync::Arc;

use axum::extract::{Multipart, Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use axum_extra::body::AsyncReadBody;
use percent_encoding::NON_ALPHANUMERIC;

use sos24_use_case::{context::Context, dto::file::CreateFileDto};

use crate::model::file::{CreatedFile, ExportFileQuery};
use crate::{
    error::AppError,
    model::file::{CreateFileQuery, File, FileInfo, Visibility},
    module::Modules,
};

pub async fn handle_get(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let raw_file_list = modules.file_use_case().list(&ctx).await;
    raw_file_list
        .map(|raw_file_list| {
            let file_list: Vec<FileInfo> = raw_file_list.into_iter().map(FileInfo::from).collect();
            (StatusCode::OK, Json(file_list))
        })
        .map_err(|err| {
            tracing::error!("Failed to list files: {err}");
            err.into()
        })
}

pub async fn handle_post(
    Query(query): Query<CreateFileQuery>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let mut filelist: Vec<CreateFileDto> = vec![];

    let owner: Option<String> = match query.visibility {
        Visibility::Private => Some(
            modules
                .project_use_case()
                .find_owned(&ctx)
                .await?
                .ok_or(AppError::new(
                    StatusCode::NOT_FOUND,
                    "file/project-not-found".to_string(),
                    "Project not found".to_string(),
                ))?
                .id,
        ),
        Visibility::Public => None,
    };

    while let Some(file) = multipart.next_field().await.map_err(|_| {
        AppError::new(
            StatusCode::BAD_REQUEST,
            "file/not-found".to_string(),
            "No file was found".to_string(),
        )
    })? {
        match file.name().ok_or(AppError::new(
            StatusCode::BAD_REQUEST,
            "file/no-name".to_string(),
            "No name was found".to_string(),
        ))? {
            "file" => {
                let filename = match file.file_name() {
                    Some(name) => name.to_string(),
                    None => Err(AppError::new(
                        StatusCode::BAD_REQUEST,
                        "file/no-file-name".to_string(),
                        "File name was not provided".to_string(),
                    ))?,
                };
                let filebytes = match file.bytes().await {
                    Ok(v) => v,
                    Err(e) => Err(AppError::new(
                        StatusCode::BAD_REQUEST,
                        "file/bad-file-bytes".to_string(),
                        e.body_text(),
                    ))?,
                };
                filelist.push(CreateFileDto::new(
                    filename,
                    filebytes.into(),
                    owner.clone(),
                ));
            }
            _ => {
                return Err(AppError::new(
                    StatusCode::BAD_REQUEST,
                    "file/invalid-name-field".to_string(),
                    "Invalid name was provided".to_string(),
                ));
            }
        }
    }

    if filelist.is_empty() {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "file/no-file-found".to_string(),
            "No content was provided".to_string(),
        ));
    }

    let mut created_file_ids = vec![];
    for file in filelist.into_iter() {
        let id = modules
            .file_use_case()
            .create(
                &ctx,
                modules.config().s3_bucket_name.clone(),
                "user-upload".to_string(),
                file,
            )
            .await?;
        created_file_ids.push(id);
    }

    Ok((
        StatusCode::CREATED,
        Json(CreatedFile {
            ids: created_file_ids,
        }),
    ))
}

pub async fn handle_export(
    State(modules): State<Arc<Modules>>,
    Query(query): Query<ExportFileQuery>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    fn archive_to_body(
        filename: String,
        body: AsyncReadBody,
    ) -> Result<impl IntoResponse, AppError> {
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
            archive_to_body(archive.filename, AsyncReadBody::new(archive.body))
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
            archive_to_body(archive.filename, AsyncReadBody::new(archive.body))
        }
        _ => Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "file/invalid-query".to_string(),
            "Invalid query".to_string(),
        )),
    }
}

pub async fn handle_get_id(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let raw_file = modules
        .file_use_case()
        .find_by_id(&ctx, modules.config().s3_bucket_name.clone(), id)
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
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.file_use_case().delete_by_id(&ctx, id).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to delete file: {err}");
        err.into()
    })
}
