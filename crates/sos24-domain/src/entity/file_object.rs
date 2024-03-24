use getset::{Getters, Setters};
use thiserror::Error;

use crate::impl_value_object;

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct FileObject {
    #[getset(get = "pub", set = "pub")]
    data: String,
    #[getset(get = "pub", set = "pub")]
    key: FileObjectKey,
}

impl FileObject {
    pub fn new(data: String, key: FileObjectKey) -> Self {
        Self { data, key }
    }
    pub fn create(data: String, prefix: &str, filename: &str) -> Self {
        Self {
            data,
            key: FileObjectKey::generate(prefix, filename),
        }
    }
}

impl_value_object!(FileObjectKey(String));
impl_value_object!(FileSignedUrl(url::Url));

#[derive(Debug, Error)]
pub enum FileSignedUrlError {
    #[error("Invalid URL")]
    InvalidUrl,
}
impl TryFrom<&str> for FileSignedUrl {
    type Error = FileSignedUrlError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        url::Url::parse(value)
            .map_err(|_| FileSignedUrlError::InvalidUrl)
            .map(Self)
    }
}

#[derive(Debug, Error)]
pub enum FileObjectError {
    #[error("Invalid UUID")]
    InvalidUuid,
}

impl FileObjectKey {
    pub fn generate(prefix: &str, filename: &str) -> Self {
        Self(format!("{}/{}/{}", prefix, uuid::Uuid::new_v4(), filename))
    }
    pub fn copy(&self) -> Self {
        Self(self.0.clone())
    }
}
