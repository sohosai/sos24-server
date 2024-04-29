use mockall::automock;
use thiserror::Error;

use crate::entity::invitation::{Invitation, InvitationId};
use crate::entity::project::ProjectId;
use crate::entity::user::UserId;

#[derive(Debug, Error)]
pub enum InvitationRepositoryError {
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

#[automock]
#[allow(async_fn_in_trait)]
pub trait InvitationRepository: Send + Sync + 'static {
    async fn list(&self) -> Result<Vec<Invitation>, InvitationRepositoryError>;
    async fn create(&self, invitation: Invitation) -> Result<(), InvitationRepositoryError>;
    async fn find_by_id(
        &self,
        id: InvitationId,
    ) -> Result<Option<Invitation>, InvitationRepositoryError>;

    async fn find_by_inviter(
        &self,
        inviter: UserId,
    ) -> Result<Vec<Invitation>, InvitationRepositoryError>;

    async fn update(&self, invitation: Invitation) -> Result<(), InvitationRepositoryError>;
    async fn delete_by_id(&self, id: InvitationId) -> Result<(), InvitationRepositoryError>;
    async fn delete_by_project_id(&self, id: ProjectId) -> Result<(), InvitationRepositoryError>;
}
