use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::permission::Permissions,
    repository::{invitation::InvitationRepository, Repositories},
};
use sos24_domain::repository::project::ProjectRepository;
use sos24_domain::repository::user::UserRepository;

use crate::{
    context::Context,
    dto::{FromEntity, invitation::InvitationDto},
};

use super::{InvitationUseCase, InvitationUseCaseError};

impl<R: Repositories> InvitationUseCase<R> {
    pub async fn list(&self, ctx: &Context) -> Result<Vec<InvitationDto>, InvitationUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_INVITATION_ALL));

        let raw_invitation_list = self.repositories.invitation_repository().list().await?;

        let mut invitation_list = Vec::new();
        for raw_invitation in raw_invitation_list {
            let inviter_id = raw_invitation.value.inviter();
            let raw_inviter = self
                .repositories
                .user_repository()
                .find_by_id(inviter_id.clone())
                .await?
                .ok_or(InvitationUseCaseError::UserNotFound(inviter_id.clone()))?;

            let project_id = raw_invitation.value.project_id();
            let raw_project = self
                .repositories
                .project_repository()
                .find_by_id(project_id.clone())
                .await?
                .ok_or(InvitationUseCaseError::ProjectNotFound(project_id.clone()))?;

            invitation_list.push(InvitationDto::from_entity((
                raw_invitation,
                raw_inviter,
                raw_project,
            )));
        }

        Ok(invitation_list)
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
        interactor::invitation::{InvitationUseCase, InvitationUseCaseError},
    };

    #[tokio::test]
    async fn 一般ユーザーは招待一覧を取得できない() {
        let repositories = MockRepositories::default();
        let use_case = InvitationUseCase::new(Arc::new(repositories), fixture::project_application_period::applicable_period());

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
        let use_case = InvitationUseCase::new(Arc::new(repositories), fixture::project_application_period::applicable_period());

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case.list(&ctx).await;
        assert!(matches!(res, Ok(list) if list.is_empty()));
    }
}
