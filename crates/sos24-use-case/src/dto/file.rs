use sos24_domain::entity::{
    common::date::WithDate, file_data::FileData, file_object::FileSignedUrl,
};

use super::FromEntity;

#[derive(Debug)]
pub struct CreateFileDto {
    pub filename: String,
    pub file: Vec<u8>,
}

impl CreateFileDto {
    pub fn new(filename: String, file: Vec<u8>) -> Self {
        Self { filename, file }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FileDto {
    pub id: String,
    pub filename: String,
    pub url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct FileEntity {
    url: FileSignedUrl,
    data: WithDate<FileData>,
}

impl FileEntity {
    pub fn new(url: FileSignedUrl, data: WithDate<FileData>) -> Self {
        Self { url, data }
    }
}

impl FromEntity for FileDto {
    type Entity = FileEntity;
    fn from_entity(entity: Self::Entity) -> Self {
        let file_data = entity.data.value.destruct();
        let url = entity.url;
        Self {
            id: file_data.id.value().to_string(),
            filename: file_data.name.value().to_string(),
            url: url.value().to_string(),
            created_at: entity.data.created_at,
            updated_at: entity.data.updated_at,
            deleted_at: entity.data.deleted_at,
        }
    }
}
