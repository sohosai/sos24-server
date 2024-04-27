use serde::{Deserialize, Serialize};

use sos24_use_case::file::dto::{FileDto, FileInfoDto};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct File {
    #[schema(format = "uuid")]
    pub id: String,
    #[schema(format = "uri")]
    pub url: String,
    pub name: String,
    #[schema(format = "uuid")]
    pub owner: Option<String>,
    #[schema(format = "date-time")]
    pub created_at: String,
    #[schema(format = "date-time")]
    pub updated_at: String,
    #[schema(format = "date-time")]
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

#[derive(Debug, Deserialize, IntoParams)]
pub struct CreateFileQuery {
    #[param(inline)]
    pub visibility: Visibility,
}

#[derive(ToSchema)]
pub struct CreateFile {
    #[schema(format = "binary")]
    pub file: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Private,
    Public,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FileInfo {
    #[schema(format = "uuid")]
    pub id: String,
    pub filename: String,
    #[schema(format = "uuid")]
    pub owner: Option<String>,
    #[schema(format = "date-time")]
    pub created_at: String,
    #[schema(format = "date-time")]
    pub updated_at: String,
    #[schema(format = "date-time")]
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

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedFile {
    #[schema(format = "uuid")]
    pub ids: Vec<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ExportFileQuery {
    #[param(format = "uuid")]
    pub project_id: Option<String>,
    #[param(format = "uuid")]
    pub form_id: Option<String>,
}
