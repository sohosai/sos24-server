use crate::entity::file_data::{FileData, FileId, FileName};
use crate::entity::project::ProjectId;
use crate::test::fixture::file_object::key;

use super::datetime;

pub fn id() -> FileId {
    FileId::new(uuid::Uuid::from_u128(1))
}

pub fn filename() -> FileName {
    FileName::sanitized("test.txt".to_string())
}

pub fn file_data(owner: Option<ProjectId>) -> FileData {
    FileData::new(
        id(),
        filename(),
        key(),
        owner,
        datetime::now(),
        datetime::now(),
    )
}
