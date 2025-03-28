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

use crate::{
    file::{FileUseCase, FileUseCaseError},
    shared::context::ContextProvider,
};

#[derive(Debug)]
pub struct CreateFileCommand {
    pub filename: String,
    pub file: Vec<u8>,
    pub owner: Option<String>,
}

impl<R: Repositories> FileUseCase<R> {
    pub async fn create(
        &self,
        ctx: &impl ContextProvider,
        bucket: String,
        key_prefix: String,
        raw_file: CreateFileCommand,
    ) -> Result<String, FileUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
        let key = FileObjectKey::generate(key_prefix.as_str());
        let filename = FileName::sanitized(raw_file.filename);
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::entity::permission::PermissionDeniedError;
    use sos24_domain::entity::user::UserRole;
    use sos24_domain::test::fixture;
    use sos24_domain::test::repository::MockRepositories;

    use crate::file::interactor::create::CreateFileCommand;
    use crate::file::{FileUseCase, FileUseCaseError};
    use crate::shared::context::TestContext;

    #[tokio::test]
    async fn 実委人は自分の企画向けのファイルを作成できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .file_object_repository_mut()
            .expect_create()
            .returning(|_, _| Ok(()));
        repositories
            .file_data_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = FileUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeViewer));
        let res = use_case
            .create(
                &ctx,
                String::new(),
                String::new(),
                CreateFileCommand {
                    filename: fixture::file_data::filename().value(),
                    file: fixture::file_object::data(),
                    owner: Some(fixture::project::id1().value().to_string()),
                },
            )
            .await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn 実委人は他人の企画向けのファイルを作成できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .file_object_repository_mut()
            .expect_create()
            .returning(|_, _| Ok(()));
        repositories
            .file_data_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = FileUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeViewer));
        let res = use_case
            .create(
                &ctx,
                String::new(),
                String::new(),
                CreateFileCommand {
                    filename: fixture::file_data::filename().value(),
                    file: fixture::file_object::data(),
                    owner: Some(fixture::project::id2().value().to_string()),
                },
            )
            .await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn 実委人は一般公開のファイルを作成できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .file_object_repository_mut()
            .expect_create()
            .returning(|_, _| Ok(()));
        repositories
            .file_data_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = FileUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeViewer));
        let res = use_case
            .create(
                &ctx,
                String::new(),
                String::new(),
                CreateFileCommand {
                    filename: fixture::file_data::filename().value(),
                    file: fixture::file_object::data(),
                    owner: None,
                },
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
    async fn 実委人管理者は一般公開のファイルを作成できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .file_object_repository_mut()
            .expect_create()
            .returning(|_, _| Ok(()));
        repositories
            .file_data_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = FileUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .create(
                &ctx,
                String::new(),
                String::new(),
                CreateFileCommand {
                    filename: fixture::file_data::filename().value(),
                    file: fixture::file_object::data(),
                    owner: None,
                },
            )
            .await;

        assert!(res.is_ok());
    }
}
