use std::sync::Arc;

use sos24_domain::entity::permission::Permissions;
use sos24_domain::repository::Repositories;
use sos24_domain::{ensure, repository::file_data::FileDataRepository};

use crate::{
    context::Context,
    dto::{file::FileInfoDto, FromEntity},
};

use super::{FileUseCase, FileUseCaseError};

impl<R: Repositories> FileUseCase<R> {
    pub async fn list(&self, ctx: &Context) -> Result<Vec<FileInfoDto>, FileUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_FILE_ALL));
        let raw_file_data_list = self.repositories.file_data_repository().list().await?;
        Ok(raw_file_data_list
            .into_iter()
            .map(FileInfoDto::from_entity)
            .collect())
    }
}
