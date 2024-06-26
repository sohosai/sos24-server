use sos24_domain::{
    entity::{
        invitation::{InvitationId, InvitationPosition},
        user::UserId,
    },
    repository::{invitation::InvitationRepository, project::ProjectRepository, Repositories},
};

use crate::{
    invitation::{InvitationUseCase, InvitationUseCaseError},
    shared::context::ContextProvider,
};

impl<R: Repositories> InvitationUseCase<R> {
    pub async fn receive(
        &self,
        ctx: &impl ContextProvider,
        id: String,
    ) -> Result<(), InvitationUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;

        if ctx.project(&*self.repositories).await?.is_some() {
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

        let project_id = invitation.project_id().clone();
        let project_with_owners = self
            .repositories
            .project_repository()
            .find_by_id(project_id.clone())
            .await?
            .ok_or(InvitationUseCaseError::ProjectNotFound(project_id))?;

        let mut new_project = project_with_owners.project;
        let user_id = UserId::new(ctx.user_id().clone());
        match invitation.position() {
            InvitationPosition::Owner => new_project.set_owner_id(user_id)?,
            InvitationPosition::SubOwner => new_project.set_sub_owner_id(user_id)?,
        }
        self.repositories
            .project_repository()
            .update(new_project)
            .await?;

        let mut new_invitation = invitation;
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
    use std::sync::Arc;

    use sos24_domain::{
        entity::{invitation::InvitationPosition, user::UserRole},
        test::{fixture, repository::MockRepositories},
    };

    use crate::{invitation::InvitationUseCase, shared::context::TestContext};

    #[tokio::test]
    async fn 一般ユーザーは招待を受けられる() {
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
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user2(UserRole::General),
                )))
            });
        repositories
            .project_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        repositories
            .invitation_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let use_case = InvitationUseCase::new(
            Arc::new(repositories),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .receive(&ctx, fixture::invitation::id().value().to_string())
            .await;
        println!("{res:?}");
        assert!(matches!(res, Ok(())));
    }

    // TODO: 一般ユーザーは自分の企画への招待を受けられない
}
