use getset::{Getters, Setters};
use percent_encoding::NON_ALPHANUMERIC;
use thiserror::Error;

use crate::impl_value_object;

use super::{common::datetime::DateTime, file_data::FileName};

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct FileObject {
    #[getset(get = "pub", set = "pub")]
    data: Vec<u8>,
    #[getset(get = "pub", set = "pub")]
    key: FileObjectKey,
}

pub struct DestructedFileObject {
    pub data: Vec<u8>,
    pub key: FileObjectKey,
}

impl FileObject {
    pub fn new(data: Vec<u8>, key: FileObjectKey) -> Self {
        Self { data, key }
    }
    pub fn create(data: Vec<u8>, prefix: &str) -> Self {
        Self {
            data,
            key: FileObjectKey::generate(prefix),
        }
    }
    pub fn destruct(self) -> DestructedFileObject {
        DestructedFileObject {
            data: self.data,
            key: self.key,
        }
    }
}

impl_value_object!(FileObjectKey(String));
impl_value_object!(FileSignedUrl(url::Url));
impl_value_object!(ContentDisposition(String));

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

impl From<FileName> for ContentDisposition {
    fn from(value: FileName) -> Self {
        Self::new(generate_content_disposition(value.value().as_bytes()))
    }
}

fn generate_content_disposition(value: &[u8]) -> String {
    format!(
        "attachment; filename*=UTF8''{}",
        percent_encoding::percent_encode(value, NON_ALPHANUMERIC)
    )
}

#[derive(Debug, Error)]
pub enum FileObjectError {
    #[error("Invalid UUID")]
    InvalidUuid,
}

impl FileObjectKey {
    pub fn generate(prefix: &str) -> Self {
        match prefix.len() {
            // /から始まるkeyは無効
            0 => Self(format!("{}", uuid::Uuid::new_v4())),
            _ => Self(format!("{}/{}", prefix, uuid::Uuid::new_v4())),
        }
    }
    pub fn copy(&self) -> Self {
        Self(self.0.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveEntry {
    key: FileObjectKey,
    filename: FileName,
    updated_at: DateTime,
}

impl ArchiveEntry {
    pub fn new(key: FileObjectKey, filename: FileName, updated_at: DateTime) -> Self {
        Self {
            key,
            filename,
            updated_at,
        }
    }

    pub fn destruct(self) -> DestructedArchiveEntry {
        DestructedArchiveEntry {
            key: self.key,
            filename: self.filename,
            updated_at: self.updated_at,
        }
    }
}

pub struct DestructedArchiveEntry {
    pub key: FileObjectKey,
    pub filename: FileName,
    pub updated_at: DateTime,
}

#[cfg(test)]
mod test {
    use crate::entity::file_object::generate_content_disposition;

    #[test]
    fn encode_non_ascii_file_name() {
        assert_eq!(
            "attachment; filename*=UTF8''%E3%83%86%E3%82%B9%E3%83%88%2Etxt",
            generate_content_disposition("テスト.txt".as_bytes())
        );
        assert_eq!(
            "attachment; filename*=UTF8''%E3%83%86%20%E3%82%B9%E3%83%88%2Etxt",
            generate_content_disposition("テ スト.txt".as_bytes())
        )
    }

    #[test]
    fn encode_injecting_file_name() {
        assert_eq!(
            "attachment; filename*=UTF8''example%22%3B%27%3B%2Etxt",
            generate_content_disposition("example\";';.txt".as_bytes())
        );
    }
}
