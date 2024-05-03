use sos24_domain::ensure;
use sos24_domain::entity::permission::Permissions;
use sos24_domain::entity::project::ProjectId;
use sos24_domain::repository::file_data::FileDataRepository;
use sos24_domain::repository::form_answer::FormAnswerRepository;
use sos24_domain::repository::invitation::InvitationRepository;
use sos24_domain::repository::project::ProjectRepository;
use sos24_domain::repository::Repositories;

use crate::project::{ProjectUseCase, ProjectUseCaseError};
use crate::shared::adapter::Adapters;
use crate::shared::context::ContextProvider;

impl<R: Repositories, A: Adapters> ProjectUseCase<R, A> {
    pub async fn delete_by_id(
        &self,
        ctx: &impl ContextProvider,
        id: String,
    ) -> Result<(), ProjectUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
        ensure!(actor.has_permission(Permissions::DELETE_PROJECT_ALL));

        let id = ProjectId::try_from(id)?;
        self.repositories
            .project_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(ProjectUseCaseError::NotFound(id.clone()))?;

        self.repositories
            .project_repository()
            .delete_by_id(id.clone())
            .await?;

        self.repositories
            .form_answer_repository()
            .delete_by_project_id(id.clone())
            .await?;

        self.repositories
            .invitation_repository()
            .delete_by_project_id(id.clone())
            .await?;

        self.repositories
            .file_data_repository()
            .delete_by_owner_project(id)
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

    use crate::project::{ProjectUseCase, ProjectUseCaseError};
    use crate::shared::adapter::MockAdapters;
    use crate::shared::context::TestContext;

    #[tokio::test]
    async fn 実委人は企画を削除できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user1(UserRole::Committee),
                )))
            });
        let adapters = MockAdapters::default();
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
            Arc::new(adapters),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .delete_by_id(&ctx, fixture::project::id1().value().to_string())
            .await;
        assert!(matches!(
            res,
            Err(ProjectUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn 実委人管理者は企画を削除できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user2(UserRole::General),
                )))
            });
        repositories
            .project_repository_mut()
            .expect_delete_by_id()
            .returning(|_| Ok(()));
        repositories
            .form_answer_repository_mut()
            .expect_delete_by_project_id()
            .returning(|_| Ok(()));
        repositories
            .invitation_repository_mut()
            .expect_delete_by_project_id()
            .returning(|_| Ok(()));
        repositories
            .file_data_repository_mut()
            .expect_delete_by_owner_project()
            .returning(|_| Ok(()));
        let adapters = MockAdapters::default();
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
            Arc::new(adapters),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .delete_by_id(&ctx, fixture::project::id1().value().to_string())
            .await;
        assert!(matches!(res, Ok(())));
    }
}
