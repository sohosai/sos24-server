use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::permission::Permissions,
    repository::{
        invitation::InvitationRepository, project::ProjectRepository, user::UserRepository,
        Repositories,
    },
};

use crate::{
    context::Context,
    dto::{invitation::CreateInvitationDto, ToEntity},
};

use super::{InvitationUseCase, InvitationUseCaseError};

impl<R: Repositories> InvitationUseCase<R> {
    pub async fn create(
        &self,
        ctx: &Context,
        raw_invitation: CreateInvitationDto,
    ) -> Result<(), InvitationUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::CREATE_INVITATION));

        let invitation = raw_invitation.into_entity()?;

        ensure!(
            self.project_application_period.contains(ctx.requested_at())
                || actor.has_permission(Permissions::CREATE_INVITATION_ANYTIME)
        );

        self.repositories
            .user_repository()
            .find_by_id(invitation.inviter().clone())
            .await?
            .ok_or(InvitationUseCaseError::InviterNotFound(
                invitation.inviter().clone(),
            ))?;

        let project = self
            .repositories
            .project_repository()
            .find_by_id(invitation.project_id().clone())
            .await?
            .ok_or(InvitationUseCaseError::ProjectNotFound(
                invitation.project_id().clone(),
            ))?;

        ensure!(project.value.is_visible_to(&actor));

        self.repositories
            .invitation_repository()
            .create(invitation)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use sos24_domain::{
        entity::{permission::PermissionDeniedError, user::UserRole},
        test::{fixture, repository::MockRepositories},
    };

    use crate::{
        context::Context,
        dto::invitation::{CreateInvitationDto, InvitationPositionDto},
        interactor::invitation::{InvitationUseCase, InvitationUseCaseError},
    };

    #[tokio::test]
    async fn 一般ユーザーは自分の企画への招待を作成できる() {
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
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = InvitationUseCase::new_for_test(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .create(
                &ctx,
                CreateInvitationDto {
                    inviter: fixture::user::id1().value().to_string(),
                    project_id: fixture::project::id1().value().to_string(),
                    position: InvitationPositionDto::SubOwner,
                },
            )
            .await;
        assert!(matches!(res, Ok(())));
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
        let use_case = InvitationUseCase::new_for_test(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .create(
                &ctx,
                CreateInvitationDto {
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

    // TODO: 一般ユーザーは募集期間外に自分の企画への招待を作成できない
    // TODO: 実委人管理者は募集期間外に他人の企画への招待を作成できる
}
