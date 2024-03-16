use getset::Getters;
use thiserror::Error;

use crate::impl_value_object;

use super::{project::ProjectId, user::UserId};

#[derive(Debug, PartialEq, Eq, Getters)]
pub struct Invitation {
    #[getset(get = "pub")]
    id: InvitationId,
    #[getset(get = "pub")]
    inviter: UserId,
    #[getset(get = "pub")]
    project_id: ProjectId,
    #[getset(get = "pub")]
    position: InvitationPosition,
}

impl Invitation {
    pub fn create(inviter: UserId, project_id: ProjectId, position: InvitationPosition) -> Self {
        Self {
            id: InvitationId::new(uuid::Uuid::new_v4()),
            inviter,
            project_id,
            position,
        }
    }

    pub fn destruct(self) -> DestructedInvitation {
        DestructedInvitation {
            id: self.id,
            inviter: self.inviter,
            project_id: self.project_id,
            position: self.position,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestructedInvitation {
    pub id: InvitationId,
    pub inviter: UserId,
    pub project_id: ProjectId,
    pub position: InvitationPosition,
}

impl_value_object!(InvitationId(uuid::Uuid));
#[derive(Debug, Error)]
pub enum InvitationIdError {
    #[error("Invalid UUID")]
    InvalidUuid,
}
impl TryFrom<String> for InvitationId {
    type Error = InvitationIdError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let uuid = uuid::Uuid::parse_str(&value).map_err(|_| InvitationIdError::InvalidUuid)?;
        Ok(Self(uuid))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvitationPosition {
    Owner,
    SubOwner,
}
