use sos24_domain::ensure;
use sos24_domain::repository::file_data::FileDataRepository;
use sos24_domain::repository::file_object::FileObjectRepository;
use sos24_domain::repository::project::ProjectRepository;
use sos24_domain::{
    entity::{file_data::FileId, file_object::ContentDisposition},
    repository::Repositories,
};

use crate::file::dto::{FileDto, FileEntity};
use crate::file::{FileUseCase, FileUseCaseError};
use crate::shared::context::ContextProvider;
use crate::FromEntity;

impl<R: Repositories> FileUseCase<R> {
    pub async fn find_by_id(
        &self,
        ctx: &impl ContextProvider,
        bucket: String,
        id: String,
    ) -> Result<FileDto, FileUseCaseError> {
        let id = FileId::try_from(id)?;
        let actor = ctx.actor(&*self.repositories).await?;
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
                .ok_or(FileUseCaseError::OwnerNotFound)?
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
    async fn 一般ユーザーは一般公開のファイルを取得できる() {
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
            .file_object_repository_mut()
            .expect_generate_url()
            .returning(|_, _, _| Ok(fixture::file_object::signed_url()));
        let use_case = FileUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_id(
                &ctx,
                String::new(),
                fixture::file_data::id().value().to_string(),
            )
            .await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn 一般ユーザーは自分の企画のファイルを取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .file_data_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::file_data::file_data(
                    Some(fixture::project::id1()),
                ))))
            });
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project1(
                    fixture::user::id1(),
                ))))
            });
        repositories
            .file_object_repository_mut()
            .expect_generate_url()
            .returning(|_, _, _| Ok(fixture::file_object::signed_url()));
        let use_case = FileUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_id(
                &ctx,
                String::new(),
                fixture::file_data::id().value().to_string(),
            )
            .await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn 一般ユーザーは他人の企画のファイルを取得できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .file_data_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::file_data::file_data(
                    Some(fixture::project::id2()),
                ))))
            });
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project1(
                    fixture::user::id2(),
                ))))
            });
        let use_case = FileUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_id(
                &ctx,
                String::new(),
                fixture::file_data::id().value().to_string(),
            )
            .await;

        assert!(matches!(
            res,
            Err(FileUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn 実委人は他人の企画のファイルを取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .file_data_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::file_data::file_data(
                    Some(fixture::project::id2()),
                ))))
            });
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project1(
                    fixture::user::id2(),
                ))))
            });
        repositories
            .file_object_repository_mut()
            .expect_generate_url()
            .returning(|_, _, _| Ok(fixture::file_object::signed_url()));
        let use_case = FileUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .find_by_id(
                &ctx,
                String::new(),
                fixture::file_data::id().value().to_string(),
            )
            .await;

        assert!(res.is_ok());
    }
}
