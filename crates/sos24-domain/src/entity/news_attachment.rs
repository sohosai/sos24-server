use getset::{Getters, Setters};
use thiserror::Error;

use crate::impl_value_object;

use super::news::NewsId;

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct NewsAttachment {
    #[getset(get = "pub")]
    id: NewsAttachmentId,
    #[getset(get = "pub", set = "pub")]
    news_id: NewsId,
    #[getset(get = "pub", set = "pub")]
    url: NewsAttachmentUrl,
}

impl NewsAttachment {
    pub fn new(id: NewsAttachmentId, news_id: NewsId, url: NewsAttachmentUrl) -> Self {
        Self { id, news_id, url }
    }

    pub fn create(news_id: NewsId, url: NewsAttachmentUrl) -> Self {
        Self {
            id: NewsAttachmentId::new(uuid::Uuid::new_v4()),
            news_id,
            url,
        }
    }

    pub fn destruct(self) -> DestructedNewsAttachment {
        DestructedNewsAttachment {
            id: self.id,
            news_id: self.news_id,
            url: self.url,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestructedNewsAttachment {
    pub id: NewsAttachmentId,
    pub news_id: NewsId,
    pub url: NewsAttachmentUrl,
}

impl_value_object!(NewsAttachmentId(uuid::Uuid));
impl_value_object!(NewsAttachmentUrl(url::Url));

#[derive(Debug, Error)]
pub enum NewsAttachmentIdError {
    #[error("Invalid UUID")]
    InvalidUuid,
}

impl TryFrom<String> for NewsAttachmentId {
    type Error = NewsAttachmentIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let uuid = uuid::Uuid::parse_str(&value).map_err(|_| NewsAttachmentIdError::InvalidUuid)?;
        Ok(Self(uuid))
    }
}
