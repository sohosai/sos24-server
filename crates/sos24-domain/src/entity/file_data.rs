use std::ffi::OsStr;
use std::path::Path;

use getset::{Getters, Setters};
use thiserror::Error;

use crate::impl_value_object;

use super::{common::datetime::DateTime, file_object::FileObjectKey, project::ProjectId};

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct FileData {
    #[getset(get = "pub")]
    id: FileId,
    #[getset(get = "pub", set = "pub")]
    filename: FileName,
    #[getset(get = "pub", set = "pub")]
    url: FileObjectKey,
    #[getset(get = "pub", set = "pub")]
    owner: Option<ProjectId>,
    #[getset(get = "pub")]
    created_at: DateTime,
    #[getset(get = "pub")]
    updated_at: DateTime,
}

impl FileData {
    pub fn new(
        id: FileId,
        name: FileName,
        url: FileObjectKey,
        owner: Option<ProjectId>,
        created_at: DateTime,
        updated_at: DateTime,
    ) -> Self {
        Self {
            id,
            filename: name,
            url,
            owner,
            created_at,
            updated_at,
        }
    }

    pub fn create(filename: FileName, url: FileObjectKey, owner: Option<ProjectId>) -> Self {
        let now = DateTime::now();
        Self {
            id: FileId::new(uuid::Uuid::new_v4()),
            filename,
            url,
            owner,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn destruct(self) -> DestructedFileData {
        DestructedFileData {
            id: self.id,
            name: self.filename,
            url: self.url,
            owner: self.owner,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestructedFileData {
    pub id: FileId,
    pub name: FileName,
    pub url: FileObjectKey,
    pub owner: Option<ProjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl_value_object!(FileId(uuid::Uuid));

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileName(String);

impl FileName {
    pub fn sanitized(name: String) -> Self {
        // ref: https://github.com/rwf2/Rocket/blob/60f3cd57b06243beaee87fd5b7545e3bf0fa6f60/core/lib/src/fs/file_name.rs#L140-L146
        static BAD_CHARS: &[char] = &[
            // These have special meaning in a file name.
            '.', '/', '\\', // These are treated specially by shells.
            '<', '>', '|', ':', '(', ')', '&', ';', '#', '?', '*',
        ];

        let file_name = Path::new(&name)
            .file_name()
            .and_then(OsStr::to_str)
            .and_then(|n| n.split(BAD_CHARS).find(|s| !s.is_empty()))
            .unwrap_or("");

        let ext = Path::new(&name)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("");

        Self(format!("{}.{}", file_name, ext))
    }

    pub fn value(self) -> String {
        self.0
    }
}

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

#[cfg(test)]
mod tests {
    use crate::entity::file_data::FileName;

    #[test]
    fn filename_sanitized() {
        const TEST_CASES: [(&str, &str); 6] = [
            ("foo.txt", "foo.txt"),
            ("foo.exe.txt", "foo.txt"),
            ("../../foo.txt", "foo.txt"),
            ("./foo.txt", "foo.txt"),
            ("/bar/foo.txt", "foo.txt"),
            ("/bar/.foo.txt", "foo.txt"),
        ];

        for (input, expected) in TEST_CASES {
            let actual = FileName::sanitized(String::from(input));
            assert_eq!(actual.value().as_str(), expected);
        }
    }
}
