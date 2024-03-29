use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::{
        file_data::{FileData, FileName},
        file_object::{FileObject, FileObjectKey},
        permission::Permissions,
        project::ProjectId,
    },
    repository::{file_data::FileDataRepository, file_object::FileObjectRepository, Repositories},
};

use crate::{context::Context, dto::file::CreateFileDto};

use super::{FileUseCase, FileUseCaseError};

impl<R: Repositories> FileUseCase<R> {
    pub async fn create(
        &self,
        ctx: &Context,
        bucket: String,
        key_prefix: String,
        raw_file: CreateFileDto,
    ) -> Result<String, FileUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        let key = FileObjectKey::generate(key_prefix.as_str());
        let filename = FileName::new(raw_file.filename);
        let owner = match raw_file.owner {
            Some(it) => {
                ensure!(actor.has_permission(Permissions::CREATE_FILE_PRIVATE));
                Some(ProjectId::try_from(it)?)
            }
            None => {
                // Publicなファイルは権限を持っていないと作れない
                ensure!(actor.has_permission(Permissions::CREATE_FILE_PUBLIC));
                None
            }
        };

        let object = FileObject::new(raw_file.file, key.clone());
        self.repositories
            .file_object_repository()
            .create(bucket, object)
            .await?;

        let data = FileData::create(filename, key, owner);
        let id = data.id().clone();
        self.repositories
            .file_data_repository()
            .create(data)
            .await?;
        
        Ok(id.value().to_string())
    }
}
