use sos24_domain::entity::permission::Permissions;
use sos24_domain::repository::file_data::FileDataRepository;
use sos24_domain::{
    ensure,
    entity::{actor::Actor, file_data::FileId},
    repository::Repositories,
};

use super::{FileUseCase, FileUseCaseError};

impl<R: Repositories> FileUseCase<R> {
    pub async fn delete_by_id(&self, actor: &Actor, id: String) -> Result<(), FileUseCaseError> {
        ensure!(actor.has_permission(Permissions::DELETE_NEWS_ALL));

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
