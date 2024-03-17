use getset::Getters;
use thiserror::Error;

use crate::impl_value_object;

use super::{actor::Actor, permission::Permissions, project::ProjectId, user::UserId};

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
    #[getset(get = "pub")]
    used_by: Option<UserId>,
}

impl Invitation {
    pub fn new(
        id: InvitationId,
        inviter: UserId,
        project_id: ProjectId,
        position: InvitationPosition,
        used_by: Option<UserId>,
    ) -> Self {
        Self {
            id,
            inviter,
            project_id,
            position,
            used_by,
        }
    }

    pub fn create(inviter: UserId, project_id: ProjectId, position: InvitationPosition) -> Self {
        Self {
            id: InvitationId::new(uuid::Uuid::new_v4()),
            inviter,
            project_id,
            position,
            used_by: None,
        }
    }

    pub fn destruct(self) -> DestructedInvitation {
        DestructedInvitation {
            id: self.id,
            inviter: self.inviter,
            project_id: self.project_id,
            position: self.position,
            used_by: self.used_by,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestructedInvitation {
    pub id: InvitationId,
    pub inviter: UserId,
    pub project_id: ProjectId,
    pub position: InvitationPosition,
    pub used_by: Option<UserId>,
}

#[derive(Debug, Error)]
pub enum InvitationError {
    #[error("Invitation already used")]
    AlreadyUsed,
    #[error("Inviter and receiver are same")]
    InviterAndReceiverAreSame,
}

impl Invitation {
    pub fn is_visible_to(&self, actor: &Actor) -> bool {
        &self.inviter == actor.user_id() || actor.has_permission(Permissions::READ_INVITATION_ALL)
    }

    pub fn receive(&mut self, user: UserId) -> Result<(), InvitationError> {
        if self.used_by.is_some() {
            return Err(InvitationError::AlreadyUsed);
        }
        if self.inviter() == &user {
            return Err(InvitationError::InviterAndReceiverAreSame);
        }

        self.used_by.replace(user);
        Ok(())
    }
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
