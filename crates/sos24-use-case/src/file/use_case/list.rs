use std::sync::Arc;

use sos24_domain::entity::common::date::WithDate;
use sos24_domain::entity::file_data::FileData;
use sos24_domain::entity::permission::Permissions;
use sos24_domain::repository::Repositories;
use sos24_domain::{ensure, repository::file_data::FileDataRepository};

use crate::context::Context;

use super::{FileUseCase, FileUseCaseError};

pub struct FileSummaryDto {
    pub id: String,
    pub filename: String,
    pub owner: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<WithDate<FileData>> for FileSummaryDto {
    fn from(entity: WithDate<FileData>) -> Self {
        let file = entity.value.destruct();
        Self {
            id: file.id.value().to_string(),
            filename: file.name.value(),
            owner: file.owner.map(|it| it.value().to_string()),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
        }
    }
}

impl<R: Repositories> FileUseCase<R> {
    pub async fn list(&self, ctx: &Context) -> Result<Vec<FileSummaryDto>, FileUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_FILE_ALL));
        let raw_file_data_list = self.repositories.file_data_repository().list().await?;
        Ok(raw_file_data_list
            .into_iter()
            .map(FileSummaryDto::from)
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

    use crate::context::Context;
    use crate::file::use_case::{FileUseCase, FileUseCaseError};

    #[tokio::test]
    async fn 一般ユーザーはファイル一覧を取得できない() {
        let repositories = MockRepositories::default();
        let use_case = FileUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
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

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case.list(&ctx).await;

        assert!(res.is_ok());
    }
}
