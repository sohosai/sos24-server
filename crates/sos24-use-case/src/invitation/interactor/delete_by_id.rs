use sos24_domain::{
    ensure,
    entity::{invitation::InvitationId, permission::Permissions},
    repository::{invitation::InvitationRepository, Repositories},
};

use crate::{
    invitation::{InvitationUseCase, InvitationUseCaseError},
    shared::context::ContextProvider,
};

impl<R: Repositories> InvitationUseCase<R> {
    pub async fn delete_by_id(
        &self,
        ctx: &impl ContextProvider,
        id: String,
    ) -> Result<(), InvitationUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
        ensure!(actor.has_permission(Permissions::DELETE_INVITATION_ALL));

        let id = InvitationId::try_from(id)?;
        self.repositories
            .invitation_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(InvitationUseCaseError::NotFound(id.clone()))?;

        self.repositories
            .invitation_repository()
            .delete_by_id(id)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::{
        entity::{
            invitation::InvitationPosition, permission::PermissionDeniedError, user::UserRole,
        },
        test::{fixture, repository::MockRepositories},
    };

    use crate::{
        invitation::{InvitationUseCase, InvitationUseCaseError},
        shared::context::TestContext,
    };

    #[tokio::test]
    async fn 実委人は招待を削除できない() {
        let repositories = MockRepositories::default();
        let use_case = InvitationUseCase::new(
            Arc::new(repositories),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor2(UserRole::Committee));
        let res = use_case
            .delete_by_id(&ctx, fixture::invitation::id().value().to_string())
            .await;
        assert!(matches!(
            res,
            Err(InvitationUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn 実委人管理者は招待を削除できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .invitation_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::invitation::invitation(
                    fixture::user::id1(),
                    fixture::project::id1(),
                    InvitationPosition::SubOwner,
                ))))
            });
        repositories
            .invitation_repository_mut()
            .expect_delete_by_id()
            .returning(|_| Ok(()));
        let use_case = InvitationUseCase::new(
            Arc::new(repositories),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .delete_by_id(&ctx, fixture::invitation::id().value().to_string())
            .await;
        assert!(matches!(res, Ok(())));
    }
}
