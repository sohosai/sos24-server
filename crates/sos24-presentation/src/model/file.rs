use serde::{Deserialize, Serialize};
use sos24_use_case::dto::file::{CreateFileDto, FileDto};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFile {
    filename: String,
    file: String,
}

impl From<CreateFile> for CreateFileDto {
    fn from(file: CreateFile) -> Self {
        CreateFileDto::new(file.filename, file.file)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub id: String,
    pub url: String,
    pub name: String,
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
            created_at: file.created_at.to_rfc3339(),
            updated_at: file.updated_at.to_rfc3339(),
            deleted_at: file.deleted_at.map(|it| it.to_rfc3339()),
        }
    }
}
