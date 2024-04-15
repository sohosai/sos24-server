use std::sync::Arc;

use tokio::io::AsyncRead;

use sos24_domain::entity::permission::Permissions;
use sos24_domain::entity::project::ProjectId;
use sos24_domain::repository::file_object::FileObjectRepository;
use sos24_domain::repository::project::ProjectRepository;
use sos24_domain::repository::Repositories;
use sos24_domain::{ensure, repository::file_data::FileDataRepository};

use crate::context::Context;
use crate::dto::file::ArchiveToBeExportedDto;

use super::{FileUseCase, FileUseCaseError};

impl<R: Repositories> FileUseCase<R> {
    pub async fn export_by_owner_project(
        &self,
        ctx: &Context,
        bucket: String,
        owner_project: String,
    ) -> Result<ArchiveToBeExportedDto<impl AsyncRead>, FileUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_FILE_ALL));

        let owner_project = ProjectId::try_from(owner_project)?;
        let raw_project = self
            .repositories
            .project_repository()
            .find_by_id(owner_project.clone())
            .await?
            .ok_or(FileUseCaseError::ProjectNotFound(owner_project.clone()))?;
        ensure!(raw_project.value.is_visible_to(&actor));

        let file_list = self
            .repositories
            .file_data_repository()
            .find_by_owner_project(owner_project)
            .await?;

        let archive = self
            .repositories
            .file_object_repository()
            .create_archive(bucket, file_list)
            .await?;

        let project = raw_project.value.destruct();
        Ok(ArchiveToBeExportedDto {
            filename: format!("{}_ファイル一覧.zip", project.title.value()),
            body: archive,
        })
    }
}

#[cfg(test)]
mod tests {}
