use mockall::automock;
use thiserror::Error;

use crate::entity::invitation::Invitation;

#[derive(Debug, Error)]
pub enum InvitationRepositoryError {
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

#[automock]
#[allow(async_fn_in_trait)]
pub trait InvitationRepository: Send + Sync + 'static {
    async fn create(&self, invitation: Invitation) -> Result<(), InvitationRepositoryError>;
}
