use sos24_domain::entity::{
    invitation::{Invitation, InvitationPosition},
    project::ProjectId,
    user::UserId,
};

use crate::interactor::invitation::InvitationUseCaseError;

use super::ToEntity;

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
