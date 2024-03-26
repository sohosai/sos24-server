use getset::{Getters, Setters};
use thiserror::Error;

use crate::impl_value_object;

use super::file_object::FileObjectKey;

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct FileData {
    #[getset(get = "pub")]
    id: FileId,
    #[getset(get = "pub", set = "pub")]
    filename: FileName,
    #[getset(get = "pub", set = "pub")]
    url: FileObjectKey,
}

impl FileData {
    pub fn new(id: FileId, name: FileName, url: FileObjectKey) -> Self {
        Self {
            id,
            filename: name,
            url,
        }
    }

    pub fn create(filename: FileName, url: FileObjectKey) -> Self {
        Self {
            id: FileId::new(uuid::Uuid::new_v4()),
            filename,
            url,
        }
    }

    pub fn destruct(self) -> DestructedFileData {
        DestructedFileData {
            id: self.id,
            name: self.filename,
            url: self.url,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestructedFileData {
    pub id: FileId,
    pub name: FileName,
    pub url: FileObjectKey,
}

impl_value_object!(FileId(uuid::Uuid));
impl_value_object!(FileName(String));

#[derive(Debug, Error)]
pub enum FileIdError {
    #[error("Invalid UUID")]
    InvalidUuid,
}

impl TryFrom<String> for FileId {
    type Error = FileIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let uuid = uuid::Uuid::parse_str(&value).map_err(|_| FileIdError::InvalidUuid)?;
        Ok(Self(uuid))
    }
}
