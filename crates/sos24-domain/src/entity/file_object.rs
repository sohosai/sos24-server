use getset::{Getters, Setters};
use percent_encoding::NON_ALPHANUMERIC;
use thiserror::Error;

use crate::impl_value_object;

use super::file_data::FileName;

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct FileObject {
    #[getset(get = "pub", set = "pub")]
    data: String,
    #[getset(get = "pub", set = "pub")]
    key: FileObjectKey,
}

pub struct DestructedFileObject {
    pub data: String,
    pub key: FileObjectKey,
}

impl FileObject {
    pub fn new(data: String, key: FileObjectKey) -> Self {
        Self { data, key }
    }
    pub fn create(data: String, prefix: &str) -> Self {
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
        return match prefix.len() {
            // /から始まるkeyは無効
            0 => Self(format!("{}", uuid::Uuid::new_v4())),
            _ => Self(format!("{}/{}", prefix, uuid::Uuid::new_v4())),
        };
    }
    pub fn copy(&self) -> Self {
        Self(self.0.clone())
    }
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
