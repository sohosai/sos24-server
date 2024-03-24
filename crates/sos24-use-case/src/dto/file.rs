use sos24_domain::entity::{common::date::WithDate, file_data::FileData};

use super::FromEntity;

#[derive(Debug)]
pub struct CreateFileDto {
    pub filename: String,
    pub file: String,
}

impl CreateFileDto {
    pub fn new(filename: String, file: String) -> Self {
        Self { filename, file }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FileDto {
    pub id: String,
    pub name: String,
    pub url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl FromEntity for FileDto {
    // ToDo: 一つの型にまとめたい
    type Entity = (WithDate<FileData>, String);
    fn from_entity((data, url): Self::Entity) -> Self {
        let file_data = data.value.destruct();
        Self {
            id: file_data.id.value().to_string(),
            name: file_data.name.value().to_string(),
            url,
            created_at: data.created_at,
            updated_at: data.updated_at,
            deleted_at: data.deleted_at,
        }
    }
}
