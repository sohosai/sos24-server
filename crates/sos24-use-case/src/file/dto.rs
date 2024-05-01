use tokio::io::AsyncRead;

use sos24_domain::entity::{file_data::FileData, file_object::FileSignedUrl};

#[derive(Debug, PartialEq, Eq)]
pub struct FileDto {
    pub id: String,
    pub filename: String,
    pub url: String,
    pub owner: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct FileInfoDto {
    pub id: String,
    pub filename: String,
    pub owner: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<FileData> for FileInfoDto {
    fn from(entity: FileData) -> Self {
        let filedata = entity.destruct();
        Self {
            id: filedata.id.value().to_string(),
            filename: filedata.name.value(),
            owner: filedata.owner.map(|it| it.value().to_string()),
            created_at: filedata.created_at.value(),
            updated_at: filedata.updated_at.value(),
        }
    }
}

impl From<(FileSignedUrl, FileData)> for FileDto {
    fn from((url, file_data_entity): (FileSignedUrl, FileData)) -> Self {
        let file_data = file_data_entity.destruct();
        Self {
            id: file_data.id.value().to_string(),
            filename: file_data.name.value().to_string(),
            url: url.value().to_string(),
            owner: file_data.owner.map(|it| it.value().to_string()),
            created_at: file_data.created_at.value(),
            updated_at: file_data.updated_at.value(),
        }
    }
}

pub struct ArchiveToBeExportedDto<R: AsyncRead> {
    pub filename: String,
    pub body: R,
}
