use sos24_domain::entity::permission::Permissions;
use sos24_domain::repository::Repositories;
use sos24_domain::{ensure, repository::file_data::FileDataRepository};

use crate::file::dto::FileInfoDto;
use crate::file::{FileUseCase, FileUseCaseError};
use crate::shared::context::ContextProvider;
use crate::FromEntity;

impl<R: Repositories> FileUseCase<R> {
    pub async fn list(
        &self,
        ctx: &impl ContextProvider,
    ) -> Result<Vec<FileInfoDto>, FileUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
        ensure!(actor.has_permission(Permissions::READ_FILE_ALL));
        let raw_file_data_list = self.repositories.file_data_repository().list().await?;
        Ok(raw_file_data_list
            .into_iter()
            .map(FileInfoDto::from_entity)
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::entity::permission::PermissionDeniedError;
    use sos24_domain::entity::user::UserRole;
    use sos24_domain::test::fixture;
    use sos24_domain::test::repository::MockRepositories;

    use crate::file::{FileUseCase, FileUseCaseError};
    use crate::shared::context::TestContext;

    #[tokio::test]
    async fn 一般ユーザーはファイル一覧を取得できない() {
        let repositories = MockRepositories::default();
        let use_case = FileUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case.list(&ctx).await;

        assert!(matches!(
            res,
            Err(FileUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn 実委人はファイル一覧を取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .file_data_repository_mut()
            .expect_list()
            .returning(|| Ok(vec![]));
        let use_case = FileUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::Committee));
        let res = use_case.list(&ctx).await;

        assert!(res.is_ok());
    }
}
