use mockall::automock;
use thiserror::Error;

use crate::entity::{
    common::date::WithDate,
    invitation::{Invitation, InvitationId},
};

#[derive(Debug, Error)]
pub enum InvitationRepositoryError {
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

#[automock]
#[allow(async_fn_in_trait)]
pub trait InvitationRepository: Send + Sync + 'static {
    async fn create(&self, invitation: Invitation) -> Result<(), InvitationRepositoryError>;
    async fn find_by_id(
        &self,
        id: InvitationId,
    ) -> Result<Option<WithDate<Invitation>>, InvitationRepositoryError>;
    async fn update(&self, invitation: Invitation) -> Result<(), InvitationRepositoryError>;
}
