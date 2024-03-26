use crate::dto::FromEntity;
use sos24_domain::repository::file_data::FileDataRepository;
use sos24_domain::repository::file_object::FileObjectRepository;
use sos24_domain::{
    entity::{file_data::FileId, file_object::ContentDisposition},
    repository::Repositories,
};

use crate::dto::file::{FileDto, FileEntity};

use super::{FileUseCase, FileUseCaseError};

impl<R: Repositories> FileUseCase<R> {
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
}
