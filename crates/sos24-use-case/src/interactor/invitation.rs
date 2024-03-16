use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::{
        common::email::EmailError,
        invitation::{InvitationError, InvitationId, InvitationIdError, InvitationPosition},
        permission::{PermissionDeniedError, Permissions},
        project::{ProjectError, ProjectId, ProjectIdError},
        project_application_period::ProjectApplicationPeriod,
        user::UserId,
    },
    repository::{
        invitation::{InvitationRepository, InvitationRepositoryError},
        project::{ProjectRepository, ProjectRepositoryError},
        user::{UserRepository, UserRepositoryError},
        Repositories,
    },
};
use thiserror::Error;

use crate::{
    context::{Context, ContextError},
    dto::{
        invitation::{CreateInvitationDto, InvitationDto},
        FromEntity, ToEntity,
    },
};

#[derive(Debug, Error)]
pub enum InvitationUseCaseError {
    #[error("Invitation not found: {0:?}")]
    NotFound(InvitationId),
    #[error("Inviter not found: {0:?}")]
    InviterNotFound(UserId),
    #[error("Project not found: {0:?}")]
    ProjectNotFound(ProjectId),
    #[error("Already owner or sub-owner")]
    AlreadyOwnerOrSubOwner,

    #[error(transparent)]
    ProjectError(#[from] ProjectError),
    #[error(transparent)]
    InvitationError(#[from] InvitationError),
    #[error(transparent)]
    InvitationIdError(#[from] InvitationIdError),
    #[error(transparent)]
    ProjectIdError(#[from] ProjectIdError),
    #[error(transparent)]
    EmailError(#[from] EmailError),
    #[error(transparent)]
    InvitationRepositoryError(#[from] InvitationRepositoryError),
    #[error(transparent)]
    ProjectRepositoryError(#[from] ProjectRepositoryError),
    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
    #[error(transparent)]
    ContextError(#[from] ContextError),
    #[error(transparent)]
    PermissionDeniedError(#[from] PermissionDeniedError),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

pub struct InvitationUseCase<R: Repositories> {
    repositories: Arc<R>,
    project_application_period: ProjectApplicationPeriod, // TODO
}

impl<R: Repositories> InvitationUseCase<R> {
    pub fn new(repositories: Arc<R>, project_application_period: ProjectApplicationPeriod) -> Self {
        Self {
            repositories,
            project_application_period,
        }
    }

    pub async fn list(&self, ctx: &Context) -> Result<Vec<InvitationDto>, InvitationUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_INVITATION_ALL));

        let raw_invitation_list = self.repositories.invitation_repository().list().await?;
        let invitation_list = raw_invitation_list
            .into_iter()
            .map(|invitation| InvitationDto::from_entity(invitation));
        Ok(invitation_list.collect())
    }

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

    pub async fn delete_by_id(
        &self,
        ctx: &Context,
        id: String,
    ) -> Result<(), InvitationUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
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
            invitation::InvitationPosition,
            permission::PermissionDeniedError,
            project::{ProjectAttributes, ProjectCategory},
            project_application_period::ProjectApplicationPeriod,
            user::UserRole,
        },
        repository::Repositories,
        test::{fixture, repository::MockRepositories},
    };

    use crate::{
        context::Context,
        dto::invitation::{CreateInvitationDto, InvitationPositionDto},
        interactor::invitation::InvitationUseCaseError,
    };

    use super::InvitationUseCase;

    fn new_invitation_use_case<R: Repositories>(repositories: R) -> InvitationUseCase<R> {
        let application_period = ProjectApplicationPeriod::new(
            chrono::Utc::now()
                .checked_sub_days(chrono::Days::new(1))
                .unwrap()
                .to_rfc3339(),
            chrono::Utc::now()
                .checked_add_days(chrono::Days::new(1))
                .unwrap()
                .to_rfc3339(),
        );
        InvitationUseCase::new(Arc::new(repositories), application_period)
    }

    #[tokio::test]
    async fn list_general_fail() {
        let repositories = MockRepositories::default();
        let use_case = new_invitation_use_case(repositories);

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
    async fn list_committee_success() {
        let mut repositories = MockRepositories::default();
        repositories
            .invitation_repository_mut()
            .expect_list()
            .returning(|| Ok(vec![]));
        let use_case = new_invitation_use_case(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case.list(&ctx).await;
        assert!(matches!(res, Ok(list) if list.is_empty()));
    }

    #[tokio::test]
    async fn create_general_success() {
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
                    ProjectCategory::General,
                    ProjectAttributes::new(0),
                    fixture::user::id1(),
                ))))
            });
        repositories
            .invitation_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = new_invitation_use_case(repositories);

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
    async fn create_general_fail() {
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
                    ProjectCategory::General,
                    ProjectAttributes::new(0),
                    fixture::user::id2(),
                ))))
            });
        repositories
            .invitation_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = new_invitation_use_case(repositories);

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

    #[tokio::test]
    async fn receive_general_sucess() {
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
                    ProjectCategory::General,
                    ProjectAttributes::new(0),
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
        let use_case = new_invitation_use_case(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .receive(&ctx, fixture::invitation::id().value().to_string())
            .await;
        println!("{res:?}");
        assert!(matches!(res, Ok(())));
    }

    #[tokio::test]
    async fn find_by_id_general_success() {
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
        let use_case = new_invitation_use_case(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_id(&ctx, fixture::invitation::id().value().to_string())
            .await;
        assert!(matches!(res, Ok(_)));
    }

    #[tokio::test]
    async fn find_by_id_general_fail() {
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
        let use_case = new_invitation_use_case(repositories);

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
    async fn find_by_id_committee_success() {
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
        let use_case = new_invitation_use_case(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .find_by_id(&ctx, fixture::invitation::id().value().to_string())
            .await;
        assert!(matches!(res, Ok(_)));
    }

    #[tokio::test]
    async fn delete_by_id_committee_fail() {
        let repositories = MockRepositories::default();
        let use_case = new_invitation_use_case(repositories);

        let ctx = Context::with_actor(fixture::actor::actor2(UserRole::Committee));
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
    async fn delete_by_id_operator_success() {
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
        let use_case = new_invitation_use_case(repositories);

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .delete_by_id(&ctx, fixture::invitation::id().value().to_string())
            .await;
        assert!(matches!(res, Ok(())));
    }
}
