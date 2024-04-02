use serde::{Deserialize, Serialize};

use sos24_use_case::dto::invitation::{CreateInvitationDto, InvitationDto, InvitationPositionDto};

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

#[derive(Debug, Serialize)]
pub struct CreatedInvitation {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Invitation {
    id: String,
    inviter: String,
    inviter_name: String,
    project_id: String,
    project_title: String,
    position: InvitationPosition,
    used_by: Option<String>,
    created_at: String,
    updated_at: String,
    deleted_at: Option<String>,
}

impl From<InvitationDto> for Invitation {
    fn from(dto: InvitationDto) -> Self {
        Self {
            id: dto.id,
            inviter: dto.inviter,
            inviter_name: dto.inviter_name,
            project_id: dto.project_id,
            project_title: dto.project_title,
            position: InvitationPosition::from(dto.position),
            used_by: dto.used_by,
            created_at: dto.created_at.to_rfc3339(),
            updated_at: dto.updated_at.to_rfc3339(),
            deleted_at: dto.deleted_at.map(|it| it.to_rfc3339()),
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

impl From<InvitationPositionDto> for InvitationPosition {
    fn from(position: InvitationPositionDto) -> Self {
        match position {
            InvitationPositionDto::Owner => Self::Owner,
            InvitationPositionDto::SubOwner => Self::SubOwner,
        }
    }
}
