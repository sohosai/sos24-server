use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::invitation::InvitationId,
    repository::{invitation::InvitationRepository, Repositories},
};

use crate::{
    context::Context,
    dto::{FromEntity, invitation::InvitationDto},
};

use super::{InvitationUseCase, InvitationUseCaseError};

impl<R: Repositories> InvitationUseCase<R> {
    pub async fn find_by_id(
        &self,
        ctx: &Context,
        id: String,
    ) -> Result<InvitationDto, InvitationUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;

        let id = InvitationId::try_from(id)?;
        let raw_invitation = self
            .repositories
            .invitation_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(InvitationUseCaseError::NotFound(id.clone()))?;

        ensure!(raw_invitation.value.is_visible_to(&actor));

        Ok(InvitationDto::from_entity(raw_invitation))
    }
}

#[cfg(test)]
mod tests {
    use sos24_domain::{
        entity::{
            invitation::InvitationPosition, permission::PermissionDeniedError, user::UserRole,
        },
        test::{fixture, repository::MockRepositories},
    };

    use crate::{
        context::Context,
        interactor::invitation::{InvitationUseCase, InvitationUseCaseError},
    };

    #[tokio::test]
    async fn 一般ユーザーは自分の企画への招待を取得できる() {
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
        let use_case = InvitationUseCase::new_for_test(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_id(&ctx, fixture::invitation::id().value().to_string())
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn 一般ユーザーは他人の企画への招待を取得できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .invitation_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::invitation::invitation(
                    fixture::user::id2(),
                    fixture::project::id1(),
                    InvitationPosition::SubOwner,
                ))))
            });
        let use_case = InvitationUseCase::new_for_test(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_id(&ctx, fixture::invitation::id().value().to_string())
            .await;
        assert!(matches!(
            res,
            Err(InvitationUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn 実委人は他人の企画への招待を取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .invitation_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::invitation::invitation(
                    fixture::user::id2(),
                    fixture::project::id1(),
                    InvitationPosition::SubOwner,
                ))))
            });
        let use_case = InvitationUseCase::new_for_test(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .find_by_id(&ctx, fixture::invitation::id().value().to_string())
            .await;
        assert!(res.is_ok());
    }
}
