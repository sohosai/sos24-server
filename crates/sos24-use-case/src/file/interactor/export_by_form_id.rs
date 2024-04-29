use std::sync::Arc;

use tokio::io::AsyncRead;

use sos24_domain::entity::file_data::FileName;
use sos24_domain::entity::file_object::ArchiveEntry;
use sos24_domain::entity::form::FormId;
use sos24_domain::entity::permission::Permissions;
use sos24_domain::repository::file_object::FileObjectRepository;
use sos24_domain::repository::form::FormRepository;
use sos24_domain::repository::form_answer::FormAnswerRepository;
use sos24_domain::repository::project::ProjectRepository;
use sos24_domain::repository::Repositories;
use sos24_domain::{ensure, repository::file_data::FileDataRepository};

use crate::file::dto::ArchiveToBeExportedDto;
use crate::file::{FileUseCase, FileUseCaseError};
use crate::shared::context::ContextProvider;

impl<R: Repositories> FileUseCase<R> {
    pub async fn export_by_form_id(
        &self,
        ctx: &impl ContextProvider,
        bucket: String,
        form_id: String,
    ) -> Result<ArchiveToBeExportedDto<impl AsyncRead>, FileUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
        ensure!(actor.has_permission(Permissions::READ_FILE_ALL));

        let form_id = FormId::try_from(form_id)?;
        let form = self
            .repositories
            .form_repository()
            .find_by_id(form_id.clone())
            .await?
            .ok_or(FileUseCaseError::FormNotFound(form_id.clone()))?;

        let form_answer_list = self
            .repositories
            .form_answer_repository()
            .find_by_form_id(form_id.clone())
            .await?;

        let mut file_list = Vec::new();
        for form_answer in form_answer_list {
            let project_id = form_answer.project_id().clone();
            let project = self
                .repositories
                .project_repository()
                .find_by_id(project_id.clone())
                .await?
                .ok_or(FileUseCaseError::ProjectNotFound(project_id))?;
            let project = project.destruct();

            let file_items = form_answer.list_file_items();
            for (item_id, files) in file_items {
                let Some(form_item) = form.find_item(&item_id) else {
                    return Err(FileUseCaseError::FormItemNotFound(item_id));
                };

                for (index, file_id) in files.into_iter().enumerate() {
                    let file = self
                        .repositories
                        .file_data_repository()
                        .find_by_id(file_id.clone())
                        .await?
                        .ok_or(FileUseCaseError::NotFound(file_id))?;
                    let file = file.destruct();

                    let filename = format!(
                        "{}/{}_{}_{}_{}_{}",
                        form_item.name().clone().value(),
                        project.index.clone().value(),
                        project.title.clone().value(),
                        project.group_name.clone().value(),
                        index + 1,
                        file.name.clone().value(),
                    );
                    file_list.push(ArchiveEntry::new(
                        file.url,
                        FileName::sanitized(filename),
                        file.updated_at,
                    ));
                }
            }
        }

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

        let form = form.destruct();
        Ok(ArchiveToBeExportedDto {
            filename: format!("{}_ファイル一覧.zip", form.title.value()),
            body: reader,
        })
    }
}

#[cfg(test)]
mod tests {}
