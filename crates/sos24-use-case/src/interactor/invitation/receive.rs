use std::sync::Arc;

use sos24_domain::{
    entity::invitation::{InvitationId, InvitationPosition},
    repository::{invitation::InvitationRepository, project::ProjectRepository, Repositories},
};

use crate::context::Context;

use super::{InvitationUseCase, InvitationUseCaseError};

impl<R: Repositories> InvitationUseCase<R> {
    pub async fn receive(&self, ctx: &Context, id: String) -> Result<(), InvitationUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;

        if ctx.project(Arc::clone(&self.repositories)).await?.is_some() {
            return Err(InvitationUseCaseError::AlreadyOwnerOrSubOwner);
        }

        // TODO: トランザクションを貼るとより良い

        let id = InvitationId::try_from(id)?;
        let invitation = self
            .repositories
            .invitation_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(InvitationUseCaseError::NotFound(id.clone()))?;

        let project_id = invitation.value.project_id().clone();
        let project = self
            .repositories
            .project_repository()
            .find_by_id(project_id.clone())
            .await?
            .ok_or(InvitationUseCaseError::ProjectNotFound(project_id))?;

        let mut new_project = project.value;
        match invitation.value.position() {
            InvitationPosition::Owner => new_project.set_owner_id(ctx.user_id().clone())?,
            InvitationPosition::SubOwner => new_project.set_sub_owner_id(ctx.user_id().clone())?,
        }
        self.repositories
            .project_repository()
            .update(new_project)
            .await?;

        let mut new_invitation = invitation.value;
        new_invitation.receive(actor.user_id().clone())?;
        self.repositories
            .invitation_repository()
            .update(new_invitation)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use sos24_domain::{
        entity::{invitation::InvitationPosition, user::UserRole},
        test::{fixture, repository::MockRepositories},
    };

    use crate::{context::Context, interactor::invitation::InvitationUseCase};

    #[tokio::test]
    async fn 一般ユーザーは招待を受けられる() {
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
        repositories
            .project_repository_mut()
            .expect_find_by_owner_id()
            .returning(|_| Ok(None));
        repositories
            .project_repository_mut()
            .expect_find_by_sub_owner_id()
            .returning(|_| Ok(None));
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project1(
                    fixture::user::id2(),
                ))))
            });
        repositories
            .project_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        repositories
            .invitation_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let use_case = InvitationUseCase::new_for_test(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .receive(&ctx, fixture::invitation::id().value().to_string())
            .await;
        println!("{res:?}");
        assert!(matches!(res, Ok(())));
    }

    // TODO: 一般ユーザーは自分の企画への招待を受けられない
}
