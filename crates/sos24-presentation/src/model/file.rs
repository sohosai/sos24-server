use serde::{Deserialize, Serialize};

use sos24_use_case::dto::file::{FileDto, FileInfoDto};

#[derive(Debug, Serialize, Deserialize)]
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
pub struct FileInfo {
    pub id: String,
    pub filename: String,
    pub owner: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl From<FileInfoDto> for FileInfo {
    fn from(file: FileInfoDto) -> Self {
        FileInfo {
            id: file.id,
            filename: file.filename,
            owner: file.owner,
            created_at: file.created_at.to_rfc3339(),
            updated_at: file.updated_at.to_rfc3339(),
            deleted_at: file.deleted_at.map(|it| it.to_rfc3339()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreatedFile {
    pub ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExportFileQuery {
    pub project_id: Option<String>,
    pub form_id: Option<String>,
}
