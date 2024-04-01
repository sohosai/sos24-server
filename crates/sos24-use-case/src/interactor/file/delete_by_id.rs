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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::entity::permission::PermissionDeniedError;
    use sos24_domain::entity::user::UserRole;
    use sos24_domain::test::fixture;
    use sos24_domain::test::repository::MockRepositories;

    use crate::context::Context;
    use crate::interactor::file::{FileUseCase, FileUseCaseError};

    #[tokio::test]
    async fn 実委人はファイルを削除できない() {
        let repositories = MockRepositories::default();
        let use_case = FileUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .delete_by_id(&ctx, fixture::file_data::id().value().to_string())
            .await;

        assert!(matches!(
            res,
            Err(FileUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn 実委人管理者はファイルを削除できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .file_data_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::file_data::file_data(
                    None,
                ))))
            });
        repositories
            .file_data_repository_mut()
            .expect_delete_by_id()
            .returning(|_| Ok(()));
        let use_case = FileUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .delete_by_id(&ctx, fixture::file_data::id().value().to_string())
            .await;

        assert!(res.is_ok());
    }
}
