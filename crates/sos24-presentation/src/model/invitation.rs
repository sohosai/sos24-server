use serde::{Deserialize, Serialize};
use sos24_use_case::dto::invitation::{CreateInvitationDto, InvitationPositionDto};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInvitation {
    project_id: String,
    position: InvitationPosition,
}

pub trait ConvertToCreateInvitationDto {
    fn to_create_invitation_dto(self) -> CreateInvitationDto;
}

impl ConvertToCreateInvitationDto for (CreateInvitation, String) {
    fn to_create_invitation_dto(self) -> CreateInvitationDto {
        let (invitation, inviter) = self;
        CreateInvitationDto {
            inviter,
            project_id: invitation.project_id,
            position: InvitationPositionDto::from(invitation.position),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvitationPosition {
    Owner,
    SubOwner,
}

impl From<InvitationPosition> for InvitationPositionDto {
    fn from(position: InvitationPosition) -> Self {
        match position {
            InvitationPosition::Owner => Self::Owner,
            InvitationPosition::SubOwner => Self::SubOwner,
        }
    }
}
