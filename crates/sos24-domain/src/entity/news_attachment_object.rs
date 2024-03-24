use getset::{Getters, Setters};
use thiserror::Error;

use crate::impl_value_object;

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct NewsAttachmentObject {
    #[getset(get = "pub", set = "pub")]
    data: String,
    #[getset(get = "pub", set = "pub")]
    key: NewsAttachmentObjectKey,
}

impl NewsAttachmentObject {
    pub fn new(data: String, key: NewsAttachmentObjectKey) -> Self {
        Self { data, key }
    }
    pub fn create(data: String, prefix: &str, filename: &str) -> Self {
        Self {
            data,
            key: NewsAttachmentObjectKey::generate(prefix, filename),
        }
    }
}

impl_value_object!(NewsAttachmentObjectKey(String));
impl_value_object!(NewsAttachmentSignedUrl(url::Url));

#[derive(Debug, Error)]
pub enum NewsAttachmentSignedUrlError {
    #[error("Invalid URL")]
    InvalidUrl,
}
impl TryFrom<&str> for NewsAttachmentSignedUrl {
    type Error = NewsAttachmentSignedUrlError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        url::Url::parse(value)
            .map_err(|_| NewsAttachmentSignedUrlError::InvalidUrl)
            .map(Self)
    }
}

#[derive(Debug, Error)]
pub enum NewsAttachmentObjectError {
    #[error("Invalid UUID")]
    InvalidUuid,
}

impl NewsAttachmentObjectKey {
    pub fn generate(prefix: &str, filename: &str) -> Self {
        Self(format!("{}/{}/{}", prefix, uuid::Uuid::new_v4(), filename))
    }
    pub fn copy(&self) -> Self {
        Self(self.0.clone())
    }
}
