use std::sync::Arc;

use sos24_domain::{
    entity::{file_data::FileId, file_object::ContentDisposition},
    repository::Repositories,
};
use sos24_domain::ensure;
use sos24_domain::repository::file_data::FileDataRepository;
use sos24_domain::repository::file_object::FileObjectRepository;
use sos24_domain::repository::project::ProjectRepository;

use crate::context::Context;
use crate::dto::file::{FileDto, FileEntity};
use crate::dto::FromEntity;

use super::{FileUseCase, FileUseCaseError};

impl<R: Repositories> FileUseCase<R> {
    pub async fn find_by_id(
        &self,
        ctx: &Context,
        bucket: String,
        id: String,
    ) -> Result<FileDto, FileUseCaseError> {
        let id = FileId::try_from(id)?;
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        let raw_file_data = self
            .repositories
            .file_data_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(FileUseCaseError::NotFound(id))?;
        if let Some(project_id) = raw_file_data.value.owner().clone() {
            let project = self
                .repositories
                .project_repository()
                .find_by_id(project_id)
                .await?
                .ok_or(FileUseCaseError::OwnerNotFound())?
                .value;
            ensure!(project.is_visible_to(&actor));
        }
        let signed_url = self
            .repositories
            .file_object_repository()
            .generate_url(
                bucket,
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
