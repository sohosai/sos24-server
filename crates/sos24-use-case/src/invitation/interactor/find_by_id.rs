use sos24_domain::{
    entity::invitation::InvitationId,
    repository::{
        invitation::InvitationRepository, project::ProjectRepository, user::UserRepository,
        Repositories,
    },
};

use crate::{
    invitation::{dto::InvitationDto, InvitationUseCase, InvitationUseCaseError},
    shared::context::ContextProvider,
};

impl<R: Repositories> InvitationUseCase<R> {
    pub async fn find_by_id(
        &self,
        _ctx: &impl ContextProvider,
        id: String,
    ) -> Result<InvitationDto, InvitationUseCaseError> {
        let id = InvitationId::try_from(id)?;
        let raw_invitation = self
            .repositories
            .invitation_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(InvitationUseCaseError::NotFound(id.clone()))?;

        let inviter_id = raw_invitation.inviter();
        let raw_inviter = self
            .repositories
            .user_repository()
            .find_by_id(inviter_id.clone())
            .await?
            .ok_or(InvitationUseCaseError::UserNotFound(inviter_id.clone()))?;

        let project_id = raw_invitation.project_id();
        let project_with_owners = self
            .repositories
            .project_repository()
            .find_by_id(project_id.clone())
            .await?
            .ok_or(InvitationUseCaseError::ProjectNotFound(project_id.clone()))?;

        Ok(InvitationDto::from((
            raw_invitation,
            raw_inviter,
            project_with_owners.project,
        )))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::{
        entity::{invitation::InvitationPosition, user::UserRole},
        test::{fixture, repository::MockRepositories},
    };

    use crate::{invitation::InvitationUseCase, shared::context::TestContext};

    #[tokio::test]
    async fn 一般ユーザーは自分の企画への招待を取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .invitation_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::invitation::invitation(
                    fixture::user::id1(),
                    fixture::project::id1(),
                    InvitationPosition::SubOwner,
                )))
            });
        repositories
            .user_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::user::user1(UserRole::General))));
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user1(UserRole::General),
                )))
            });
        let use_case = InvitationUseCase::new(
            Arc::new(repositories),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_id(&ctx, fixture::invitation::id().value().to_string())
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn 一般ユーザーは他人の企画への招待を取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .invitation_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::invitation::invitation(
                    fixture::user::id2(),
                    fixture::project::id1(),
                    InvitationPosition::SubOwner,
                )))
            });
        repositories
            .user_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::user::user2(UserRole::General))));
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user1(UserRole::General),
                )))
            });
        let use_case = InvitationUseCase::new(
            Arc::new(repositories),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_id(&ctx, fixture::invitation::id().value().to_string())
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn 実委人は他人の企画への招待を取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .invitation_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::invitation::invitation(
                    fixture::user::id2(),
                    fixture::project::id1(),
                    InvitationPosition::SubOwner,
                )))
            });
        repositories
            .user_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::user::user1(UserRole::General))));
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user1(UserRole::CommitteeViewer),
                )))
            });
        let use_case = InvitationUseCase::new(
            Arc::new(repositories),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeViewer));
        let res = use_case
            .find_by_id(&ctx, fixture::invitation::id().value().to_string())
            .await;
        assert!(res.is_ok());
    }
}
