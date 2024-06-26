use sos24_domain::{
    ensure,
    entity::{
        invitation::{Invitation, InvitationPosition},
        permission::Permissions,
        project::ProjectId,
        user::UserId,
    },
    repository::{
        invitation::InvitationRepository, project::ProjectRepository, user::UserRepository,
        Repositories,
    },
};

use crate::{
    invitation::{dto::InvitationPositionDto, InvitationUseCase, InvitationUseCaseError},
    shared::context::ContextProvider,
};

#[derive(Debug)]
pub struct CreateInvitationCommand {
    pub inviter: String,
    pub project_id: String,
    pub position: InvitationPositionDto,
}

impl<R: Repositories> InvitationUseCase<R> {
    pub async fn find_or_create(
        &self,
        ctx: &impl ContextProvider,
        raw_invitation: CreateInvitationCommand,
    ) -> Result<String, InvitationUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
        ensure!(actor.has_permission(Permissions::CREATE_INVITATION));

        let new_invitation = Invitation::create(
            UserId::new(raw_invitation.inviter),
            ProjectId::try_from(raw_invitation.project_id)?,
            InvitationPosition::from(raw_invitation.position),
        );

        ensure!(
            self.project_application_period
                .can_create_project(&actor, ctx.requested_at())
                || actor.has_permission(Permissions::CREATE_INVITATION_ANYTIME)
        );

        self.repositories
            .user_repository()
            .find_by_id(new_invitation.inviter().clone())
            .await?
            .ok_or(InvitationUseCaseError::InviterNotFound(
                new_invitation.inviter().clone(),
            ))?;

        let project_with_owners = self
            .repositories
            .project_repository()
            .find_by_id(new_invitation.project_id().clone())
            .await?
            .ok_or(InvitationUseCaseError::ProjectNotFound(
                new_invitation.project_id().clone(),
            ))?;

        ensure!(project_with_owners.project.is_visible_to(&actor));

        let invitation_list = self
            .repositories
            .invitation_repository()
            .find_by_inviter(new_invitation.inviter().clone())
            .await?;
        for invitation in invitation_list {
            if !invitation.is_used()
                && invitation.project_id() == new_invitation.project_id()
                && invitation.position() == new_invitation.position()
            {
                let invitation_id = invitation.id().clone();
                return Ok(invitation_id.value().to_string());
            }
        }

        let invitation_id = new_invitation.id().clone();
        self.repositories
            .invitation_repository()
            .create(new_invitation)
            .await?;

        Ok(invitation_id.value().to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::{
        entity::{permission::PermissionDeniedError, user::UserRole},
        test::{fixture, repository::MockRepositories},
    };

    use crate::{
        invitation::{
            dto::InvitationPositionDto, interactor::find_or_create::CreateInvitationCommand,
            InvitationUseCase, InvitationUseCaseError,
        },
        shared::context::TestContext,
    };

    #[tokio::test]
    async fn 一般ユーザーは企画募集期間内に自分の企画への招待を作成できる() {
        let mut repositories = MockRepositories::default();
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
        repositories
            .invitation_repository_mut()
            .expect_find_by_inviter()
            .returning(|_| Ok(vec![]));
        repositories
            .invitation_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = InvitationUseCase::new(
            Arc::new(repositories),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_or_create(
                &ctx,
                CreateInvitationCommand {
                    inviter: fixture::user::id1().value().to_string(),
                    project_id: fixture::project::id1().value().to_string(),
                    position: InvitationPositionDto::SubOwner,
                },
            )
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn 一般ユーザーは企画募集期間外に自分の企画への招待を作成できない() {
        let repositories = MockRepositories::default();
        let use_case = InvitationUseCase::new(
            Arc::new(repositories),
            fixture::project_application_period::not_applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_or_create(
                &ctx,
                CreateInvitationCommand {
                    inviter: fixture::user::id1().value().to_string(),
                    project_id: fixture::project::id1().value().to_string(),
                    position: InvitationPositionDto::SubOwner,
                },
            )
            .await;
        assert!(matches!(
            res,
            Err(InvitationUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn 一般ユーザーは他人の企画への招待を作成できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .user_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::user::user1(UserRole::General))));
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user2(UserRole::General),
                )))
            });
        repositories
            .invitation_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = InvitationUseCase::new(
            Arc::new(repositories),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_or_create(
                &ctx,
                CreateInvitationCommand {
                    inviter: fixture::user::id1().value().to_string(),
                    project_id: fixture::project::id1().value().to_string(),
                    position: InvitationPositionDto::SubOwner,
                },
            )
            .await;
        assert!(matches!(
            res,
            Err(InvitationUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn 実委人管理者は他人の企画への招待を作成できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .user_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::user::user1(UserRole::CommitteeOperator))));
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user2(UserRole::General),
                )))
            });
        repositories
            .invitation_repository_mut()
            .expect_find_by_inviter()
            .returning(|_| Ok(vec![]));
        repositories
            .invitation_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = InvitationUseCase::new(
            Arc::new(repositories),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .find_or_create(
                &ctx,
                CreateInvitationCommand {
                    inviter: fixture::user::id1().value().to_string(),
                    project_id: fixture::project::id1().value().to_string(),
                    position: InvitationPositionDto::SubOwner,
                },
            )
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn 実委人管理者は企画募集期間外に他人の企画への招待を作成できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .user_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::user::user1(UserRole::CommitteeOperator))));
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user2(UserRole::General),
                )))
            });
        repositories
            .invitation_repository_mut()
            .expect_find_by_inviter()
            .returning(|_| Ok(vec![]));
        repositories
            .invitation_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = InvitationUseCase::new(
            Arc::new(repositories),
            fixture::project_application_period::not_applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .find_or_create(
                &ctx,
                CreateInvitationCommand {
                    inviter: fixture::user::id1().value().to_string(),
                    project_id: fixture::project::id1().value().to_string(),
                    position: InvitationPositionDto::SubOwner,
                },
            )
            .await;
        assert!(res.is_ok());
    }
}
