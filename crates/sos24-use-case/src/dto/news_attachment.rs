use sos24_domain::entity::{common::date::WithDate, news_attachment_data::NewsAttachmentData};

use super::FromEntity;

#[derive(Debug)]
pub struct CreateNewsAttachmentDto {
    pub filename: String,
    pub file: String,
}

impl CreateNewsAttachmentDto {
    pub fn new(filename: String, file: String) -> Self {
        Self { filename, file }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NewsAttachmentDto {
    pub id: String,
    pub name: String,
    pub url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl FromEntity for NewsAttachmentDto {
    // ToDo: 一つの型にまとめたい
    type Entity = (WithDate<NewsAttachmentData>, String);
    fn from_entity((data, url): Self::Entity) -> Self {
        let attachment_data = data.value.destruct();
        Self {
            id: attachment_data.id.value().to_string(),
            name: attachment_data.name.value().to_string(),
            url,
            created_at: data.created_at,
            updated_at: data.updated_at,
            deleted_at: data.deleted_at,
        }
    }
}
