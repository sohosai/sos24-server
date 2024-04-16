use std::sync::Arc;

use axum::extract::{Multipart, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};

use serde::{Deserialize, Serialize};
use sos24_use_case::context::Context;
use sos24_use_case::file::use_case::create::CreateFileCommand;

use crate::{error::AppError, module::Modules};

#[derive(Debug, Deserialize)]
pub struct CreateFileQuery {
    pub visibility: Visibility,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Private,
    Public,
}

#[derive(Debug, Serialize)]
pub struct CreatedFile {
    pub ids: Vec<String>,
}

pub async fn handle(
    Query(query): Query<CreateFileQuery>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let mut filelist: Vec<CreateFileCommand> = vec![];

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
                .project
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
                filelist.push(CreateFileCommand {
                    filename,
                    file: filebytes.into(),
                    owner: owner.clone(),
                });
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
