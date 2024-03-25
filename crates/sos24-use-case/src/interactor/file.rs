use std::sync::Arc;

use sos24_domain::entity::file_data::{FileData, FileId, FileIdError, FileName};
use sos24_domain::entity::file_object::{ContentDisposition, FileObject, FileObjectKey};
use sos24_domain::repository::file_data::{FileDataRepository, FileDataRepositoryError};
use sos24_domain::repository::file_object::{FileObjectRepository, FileObjectRepositoryError};
use sos24_domain::{
    ensure,
    entity::{
        actor::Actor,
        permission::{PermissionDeniedError, Permissions},
    },
    repository::Repositories,
};
use thiserror::Error;

use crate::dto::{
    file::{CreateFileDto, FileDto, FileEntity},
    FromEntity,
};

#[derive(Debug, Error)]
pub enum FileUseCaseError {
    #[error("File not found: {0:?}")]
    NotFound(FileId),
    #[error(transparent)]
    FileDataRepositoryError(#[from] FileDataRepositoryError),
    #[error(transparent)]
    FileObjectRepositoryError(#[from] FileObjectRepositoryError),
    #[error(transparent)]
    FileIdError(#[from] FileIdError),
    #[error(transparent)]
    PermissionDeniedError(#[from] PermissionDeniedError),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

pub struct FileUseCase<R: Repositories> {
    repositories: Arc<R>,
}

impl<R: Repositories> FileUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }

    pub async fn list(&self, bucket: String) -> Result<Vec<FileDto>, FileUseCaseError> {
        // ToDo: 権限
        let raw_file_data_list = self.repositories.file_data_repository().list().await?;
        let mut file_list: Vec<FileDto> = vec![];
        for file_data in raw_file_data_list {
            let url = self
                .repositories
                .file_object_repository()
                .generate_url(
                    bucket.clone(),
                    file_data.value.url().copy(),
                    Some(ContentDisposition::from(file_data.value.filename().clone())),
                )
                .await?;
            file_list.push(FileDto::from_entity(FileEntity::new(url, file_data)));
        }
        Ok(file_list)
    }

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

    pub async fn find_by_id(
        &self,
        backet: String,
        id: String,
    ) -> Result<FileDto, FileUseCaseError> {
        let id = FileId::try_from(id)?;
        let raw_file_data = self
            .repositories
            .file_data_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(FileUseCaseError::NotFound(id))?;
        let signed_url = self
            .repositories
            .file_object_repository()
            .generate_url(
                backet,
                raw_file_data.value.url().copy(),
                Some(ContentDisposition::from(
                    raw_file_data.value.filename().clone(),
                )),
            )
            .await?;
        Ok(FileDto::from_entity(FileEntity::new(
            signed_url,
            raw_file_data,
        )))
    }

    pub async fn delete_by_id(&self, actor: &Actor, id: String) -> Result<(), FileUseCaseError> {
        ensure!(actor.has_permission(Permissions::DELETE_NEWS_ALL));

        // ソフトデリートで実装している（オブジェクトストレージからは削除されない）
        let id = FileId::try_from(id)?;
        self.repositories
            .file_data_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(FileUseCaseError::NotFound(id.clone()))?;

        self.repositories
            .file_data_repository()
            .delete_by_id(id)
            .await?;
        Ok(())
    }
}
