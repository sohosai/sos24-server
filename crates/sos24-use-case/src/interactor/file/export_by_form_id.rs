use std::sync::Arc;

use tokio::io::AsyncRead;

use sos24_domain::entity::form::FormId;
use sos24_domain::entity::permission::Permissions;
use sos24_domain::repository::file_object::FileObjectRepository;
use sos24_domain::repository::form::FormRepository;
use sos24_domain::repository::form_answer::FormAnswerRepository;
use sos24_domain::repository::Repositories;
use sos24_domain::{ensure, repository::file_data::FileDataRepository};

use crate::context::Context;
use crate::dto::file::ArchiveToBeExportedDto;

use super::{FileUseCase, FileUseCaseError};

impl<R: Repositories> FileUseCase<R> {
    pub async fn export_by_form_id(
        &self,
        ctx: &Context,
        bucket: String,
        form_id: String,
    ) -> Result<ArchiveToBeExportedDto<impl AsyncRead>, FileUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
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
            let file_ids = form_answer.value.list_files();
            for file_id in file_ids {
                let file = self
                    .repositories
                    .file_data_repository()
                    .find_by_id(file_id.clone())
                    .await?
                    .ok_or(FileUseCaseError::NotFound(file_id))?;
                file_list.push(file);
            }
        }

        let archive = self
            .repositories
            .file_object_repository()
            .create_archive(bucket, file_list)
            .await?;

        let form = form.value.destruct();
        Ok(ArchiveToBeExportedDto {
            filename: format!("{}_ファイル一覧.zip", form.title.value()),
            body: archive,
        })
    }
}

#[cfg(test)]
mod tests {}
