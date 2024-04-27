use std::sync::Arc;

use tokio::io::AsyncRead;

use sos24_domain::entity::file_object::ArchiveEntry;
use sos24_domain::entity::permission::Permissions;
use sos24_domain::entity::project::ProjectId;
use sos24_domain::repository::file_object::FileObjectRepository;
use sos24_domain::repository::project::ProjectRepository;
use sos24_domain::repository::Repositories;
use sos24_domain::{ensure, repository::file_data::FileDataRepository};

use crate::file::dto::ArchiveToBeExportedDto;
use crate::file::{FileUseCase, FileUseCaseError};
use crate::shared::context::ContextProvider;

impl<R: Repositories> FileUseCase<R> {
    pub async fn export_by_owner_project(
        &self,
        ctx: &impl ContextProvider,
        bucket: String,
        owner_project: String,
    ) -> Result<ArchiveToBeExportedDto<impl AsyncRead>, FileUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
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
            .await?
            .into_iter()
            .map(|file| {
                let file_data = file.value.destruct();
                ArchiveEntry::new(file_data.url, file_data.name, file.updated_at)
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65535);
        let repositories = Arc::clone(&self.repositories);
        tokio::spawn(async move {
            if let Err(err) = repositories
                .file_object_repository()
                .create_archive(bucket, file_list, writer)
                .await
            {
                tracing::error!("Failed to create archive: {err:?}");
            }
        });

        let project = raw_project.value.destruct();
        Ok(ArchiveToBeExportedDto {
            filename: format!("{}_ファイル一覧.zip", project.title.value()),
            body: reader,
        })
    }
}

#[cfg(test)]
mod tests {}
