use sos24_domain::entity::{
    common::date::WithDate,
    invitation::{Invitation, InvitationPosition},
    project::ProjectId,
    user::UserId,
};

use crate::interactor::invitation::InvitationUseCaseError;

use super::{FromEntity, ToEntity};

#[derive(Debug)]
pub struct CreateInvitationDto {
    pub inviter: String,
    pub project_id: String,
    pub position: InvitationPositionDto,
}

impl ToEntity for CreateInvitationDto {
    type Entity = Invitation;
    type Error = InvitationUseCaseError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        Ok(Invitation::create(
            UserId::new(self.inviter),
            ProjectId::try_from(self.project_id)?,
            self.position.into_entity()?,
        ))
    }
}

#[derive(Debug)]
pub struct InvitationDto {
    pub id: String,
    pub inviter: String,
    pub project_id: String,
    pub position: InvitationPositionDto,
    pub used_by: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl FromEntity for InvitationDto {
    type Entity = WithDate<Invitation>;
    fn from_entity(entity: Self::Entity) -> Self {
        let invitation = entity.value.destruct();
        Self {
            id: invitation.id.value().to_string(),
            inviter: invitation.inviter.value().to_string(),
            project_id: invitation.project_id.value().to_string(),
            position: InvitationPositionDto::from_entity(invitation.position),
            used_by: invitation.used_by.map(|id| id.value().to_string()),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
        }
    }
}

#[derive(Debug)]
pub enum InvitationPositionDto {
    Owner,
    SubOwner,
}

impl ToEntity for InvitationPositionDto {
    type Entity = InvitationPosition;
    type Error = InvitationUseCaseError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        match self {
            Self::Owner => Ok(InvitationPosition::Owner),
            Self::SubOwner => Ok(InvitationPosition::SubOwner),
        }
    }
}

impl FromEntity for InvitationPositionDto {
    type Entity = InvitationPosition;
    fn from_entity(entity: Self::Entity) -> Self {
        match entity {
            InvitationPosition::Owner => Self::Owner,
            InvitationPosition::SubOwner => Self::SubOwner,
        }
    }
}
