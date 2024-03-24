use getset::{Getters, Setters};
use thiserror::Error;

use crate::impl_value_object;

use super::news_attachment_object::NewsAttachmentObjectKey;

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct NewsAttachmentData {
    #[getset(get = "pub")]
    id: NewsAttachmentId,
    #[getset(get = "pub", set = "pub")]
    filename: NewsAttachmentFilename,
    #[getset(get = "pub", set = "pub")]
    url: NewsAttachmentObjectKey,
}

impl NewsAttachmentData {
    pub fn new(
        id: NewsAttachmentId,
        name: NewsAttachmentFilename,
        url: NewsAttachmentObjectKey,
    ) -> Self {
        Self {
            id,
            filename: name,
            url,
        }
    }

    pub fn create(filename: NewsAttachmentFilename, url: NewsAttachmentObjectKey) -> Self {
        Self {
            id: NewsAttachmentId::new(uuid::Uuid::new_v4()),
            filename,
            url,
        }
    }

    pub fn destruct(self) -> DestructedNewsAttachment {
        DestructedNewsAttachment {
            id: self.id,
            name: self.filename,
            url: self.url,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestructedNewsAttachment {
    pub id: NewsAttachmentId,
    pub name: NewsAttachmentFilename,
    pub url: NewsAttachmentObjectKey,
}

impl_value_object!(NewsAttachmentId(uuid::Uuid));
impl_value_object!(NewsAttachmentFilename(String));

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
