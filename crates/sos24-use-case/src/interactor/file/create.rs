use sos24_domain::repository::file_data::FileDataRepository;
use sos24_domain::repository::file_object::FileObjectRepository;
use sos24_domain::{
    entity::{
        file_data::{FileData, FileName},
        file_object::{FileObject, FileObjectKey},
    },
    repository::Repositories,
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
        // ToDo: 権限・所有者設定
        let key = FileObjectKey::generate(key_prefix.as_str());
        let filename = FileName::new(raw_file.filename);

        let object = FileObject::new(raw_file.file, key.clone());
        self.repositories
            .file_object_repository()
            .create(bucket, object)
            .await?;

        let data = FileData::create(filename, key);
        self.repositories
            .file_data_repository()
            .create(data)
            .await?;
        Ok(())
    }
}
