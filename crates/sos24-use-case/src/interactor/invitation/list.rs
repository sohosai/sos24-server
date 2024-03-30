use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::permission::Permissions,
    repository::{invitation::InvitationRepository, Repositories},
};

use crate::{
    context::Context,
    dto::{invitation::InvitationDto, FromEntity},
};

use super::{InvitationUseCase, InvitationUseCaseError};

impl<R: Repositories> InvitationUseCase<R> {
    pub async fn list(&self, ctx: &Context) -> Result<Vec<InvitationDto>, InvitationUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_INVITATION_ALL));

        let raw_invitation_list = self.repositories.invitation_repository().list().await?;
        let invitation_list = raw_invitation_list
            .into_iter()
            .map(InvitationDto::from_entity);
        Ok(invitation_list.collect())
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
        interactor::invitation::{InvitationUseCase, InvitationUseCaseError},
    };

    #[tokio::test]
    async fn 一般ユーザーは招待一覧を取得できない() {
        let repositories = MockRepositories::default();
        let use_case = InvitationUseCase::new_for_test(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case.list(&ctx).await;
        assert!(matches!(
            res,
            Err(InvitationUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn 実委人は招待一覧を取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .invitation_repository_mut()
            .expect_list()
            .returning(|| Ok(vec![]));
        let use_case = InvitationUseCase::new_for_test(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case.list(&ctx).await;
        assert!(matches!(res, Ok(list) if list.is_empty()));
    }
}
