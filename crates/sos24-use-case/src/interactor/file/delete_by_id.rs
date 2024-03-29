use std::sync::Arc;

use sos24_domain::entity::permission::Permissions;
use sos24_domain::repository::file_data::FileDataRepository;
use sos24_domain::{ensure, entity::file_data::FileId, repository::Repositories};

use crate::context::Context;

use super::{FileUseCase, FileUseCaseError};

impl<R: Repositories> FileUseCase<R> {
    pub async fn delete_by_id(&self, ctx: &Context, id: String) -> Result<(), FileUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::DELETE_FILE_ALL));

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
