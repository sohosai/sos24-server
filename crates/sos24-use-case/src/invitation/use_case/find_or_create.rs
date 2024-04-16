use std::sync::Arc;

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

use crate::{context::Context, invitation::dto::InvitationPositionDto};

use super::{InvitationUseCase, InvitationUseCaseError};

pub struct CreateInvitationCommand {
    pub project_id: String,
    pub position: InvitationPositionDto,
}

impl<R: Repositories> InvitationUseCase<R> {
    pub async fn find_or_create(
        &self,
        ctx: &Context,
        invitation_data: CreateInvitationCommand,
    ) -> Result<String, InvitationUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::CREATE_INVITATION));

        let inviter = actor.user_id().clone().value();
        let new_invitation = Invitation::create(
            UserId::new(inviter),
            ProjectId::try_from(invitation_data.project_id)?,
            InvitationPosition::from(invitation_data.position),
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

        let project = self
            .repositories
            .project_repository()
            .find_by_id(new_invitation.project_id().clone())
            .await?
            .ok_or(InvitationUseCaseError::ProjectNotFound(
                new_invitation.project_id().clone(),
            ))?;

        ensure!(project.value.is_visible_to(&actor));

        let invitation_list = self
            .repositories
            .invitation_repository()
            .find_by_inviter(new_invitation.inviter().clone())
            .await?;
        for invitation in invitation_list {
            let invitation = invitation.value;
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
        context::Context,
        invitation::{
            dto::InvitationPositionDto,
            use_case::{
                find_or_create::CreateInvitationCommand, InvitationUseCase, InvitationUseCaseError,
            },
        },
    };

    #[tokio::test]
    async fn 一般ユーザーは企画募集期間内に自分の企画への招待を作成できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .user_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::user::user1(
                    UserRole::General,
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

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_or_create(
                &ctx,
                CreateInvitationCommand {
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

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_or_create(
                &ctx,
                CreateInvitationCommand {
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
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::user::user1(
                    UserRole::General,
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
            .invitation_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = InvitationUseCase::new(
            Arc::new(repositories),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_or_create(
                &ctx,
                CreateInvitationCommand {
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
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::user::user1(
                    UserRole::CommitteeOperator,
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

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .find_or_create(
                &ctx,
                CreateInvitationCommand {
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
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::user::user1(
                    UserRole::CommitteeOperator,
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

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .find_or_create(
                &ctx,
                CreateInvitationCommand {
                    project_id: fixture::project::id1().value().to_string(),
                    position: InvitationPositionDto::SubOwner,
                },
            )
            .await;
        assert!(res.is_ok());
    }
}
