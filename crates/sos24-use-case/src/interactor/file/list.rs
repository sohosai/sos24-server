use sos24_domain::repository::file_data::FileDataRepository;
use sos24_domain::repository::Repositories;

use crate::dto::{file::FileInfoDto, FromEntity};

use super::{FileUseCase, FileUseCaseError};

impl<R: Repositories> FileUseCase<R> {
    pub async fn list(&self) -> Result<Vec<FileInfoDto>, FileUseCaseError> {
        // ToDo: 権限
        let raw_file_data_list = self.repositories.file_data_repository().list().await?;
        Ok(raw_file_data_list
            .into_iter()
            .map(FileInfoDto::from_entity)
            .collect())
    }
}
