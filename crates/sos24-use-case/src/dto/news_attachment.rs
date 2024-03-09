use std::str::FromStr;

use sos24_domain::entity::{
    common::date::WithDate,
    news::{NewsId, NewsIdError},
    news_attachment::{NewsAttachment, NewsAttachmentUrl},
};

use crate::interactor::news_attachment::NewsAttachmentUseCaseError;

use super::{FromEntity, ToEntity};

#[derive(Debug)]
pub struct CreateNewsAttachmentDto {
    pub news_id: String,
    pub url: String,
}

impl CreateNewsAttachmentDto {
    pub fn new(news_id: String, url: String) -> Self {
        Self { news_id, url }
    }
}

impl ToEntity for CreateNewsAttachmentDto {
    type Entity = NewsAttachment;
    type Error = NewsAttachmentUseCaseError;

    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        let news_id = uuid::Uuid::from_str(&self.news_id).map_err(|_| {
            NewsAttachmentUseCaseError::NewsAttachmentNewsIdError(NewsIdError::InvalidUuid)
        })?;
        let url = url::Url::parse(&self.url)
            .map_err(NewsAttachmentUseCaseError::NewsAttachmentUrlError)?;

        Ok(NewsAttachment::create(
            NewsId::new(news_id),
            NewsAttachmentUrl::new(url),
        ))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NewsAttachmentDto {
    pub id: String,
    pub news_id: String,
    pub url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl FromEntity for NewsAttachmentDto {
    type Entity = WithDate<NewsAttachment>;
    fn from_entity(entity: Self::Entity) -> Self {
        let news_attachment = entity.value.destruct();
        Self {
            id: news_attachment.id.value().to_string(),
            news_id: news_attachment.news_id.value().to_string(),
            url: news_attachment.url.value().to_string(),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
        }
    }
}
