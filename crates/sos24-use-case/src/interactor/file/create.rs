
use sos24_domain::{
    entity::{
        file_data::{FileData, FileName},
        file_object::{FileObject, FileObjectKey},
        project::ProjectId,
    },
    repository::{
        file_data::FileDataRepository, file_object::FileObjectRepository,
        Repositories,
    },
};

use crate::dto::file::CreateFileDto;

use super::{FileUseCase, FileUseCaseError};

impl<R: Repositories> FileUseCase<R> {
    pub async fn create(
        &self,
        bucket: String,
        key_prefix: String,
        raw_file: CreateFileDto,
    ) -> Result<(), FileUseCaseError> {
        let key = FileObjectKey::generate(key_prefix.as_str());
        let filename = FileName::new(raw_file.filename);
        let owner = match raw_file.owner {
            Some(it) =>  Some(ProjectId::try_from(it)?),
            None => None,
        };

        let object = FileObject::new(raw_file.file, key.clone());
        self.repositories
            .file_object_repository()
            .create(bucket, object)
            .await?;

        let data = FileData::create(filename, key, owner);
        self.repositories
            .file_data_repository()
            .create(data)
            .await?;
        Ok(())
    }
}
