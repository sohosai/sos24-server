use getset::Getters;
use thiserror::Error;

use crate::impl_value_object;

use super::{common::datetime::DateTime, project::ProjectId, user::UserId};

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
    #[getset(get = "pub")]
    created_at: DateTime,
    #[getset(get = "pub")]
    updated_at: DateTime,
}

impl Invitation {
    pub fn new(
        id: InvitationId,
        inviter: UserId,
        project_id: ProjectId,
        position: InvitationPosition,
        used_by: Option<UserId>,
        created_at: DateTime,
        updated_at: DateTime,
    ) -> Self {
        Self {
            id,
            inviter,
            project_id,
            position,
            used_by,
            created_at,
            updated_at,
        }
    }

    pub fn create(inviter: UserId, project_id: ProjectId, position: InvitationPosition) -> Self {
        let now = DateTime::now();
        Self {
            id: InvitationId::new(uuid::Uuid::new_v4()),
            inviter,
            project_id,
            position,
            used_by: None,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn destruct(self) -> DestructedInvitation {
        DestructedInvitation {
            id: self.id,
            inviter: self.inviter,
            project_id: self.project_id,
            position: self.position,
            used_by: self.used_by,
            created_at: self.created_at,
            updated_at: self.updated_at,
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
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Error)]
pub enum InvitationError {
    #[error("Invitation already used")]
    AlreadyUsed,
    #[error("Inviter and receiver are same")]
    InviterAndReceiverAreSame,
}

impl Invitation {
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

    pub fn is_used(&self) -> bool {
        self.used_by.is_some()
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
