use sos24_domain::entity::project::Project;
use sos24_domain::entity::user::User;
use sos24_domain::entity::{
    common::date::WithDate,
    invitation::{Invitation, InvitationPosition},
};

#[derive(Debug)]
pub struct InvitationDto {
    pub id: String,
    pub inviter: String,
    pub inviter_name: String,
    pub project_id: String,
    pub project_title: String,
    pub position: InvitationPositionDto,
    pub used_by: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<(WithDate<Invitation>, WithDate<User>, WithDate<Project>)> for InvitationDto {
    fn from(
        (invitation_entity, user_entity, project_entity): (
            WithDate<Invitation>,
            WithDate<User>,
            WithDate<Project>,
        ),
    ) -> Self {
        let invitation = invitation_entity.value.destruct();
        let inviter = user_entity.value.destruct();
        let project = project_entity.value.destruct();

        Self {
            id: invitation.id.value().to_string(),
            inviter: invitation.inviter.value().to_string(),
            inviter_name: inviter.name.value().to_string(),
            project_id: invitation.project_id.value().to_string(),
            project_title: project.title.value().to_string(),
            position: InvitationPositionDto::from(invitation.position),
            used_by: invitation.used_by.map(|id| id.value().to_string()),
            created_at: invitation_entity.created_at,
            updated_at: invitation_entity.updated_at,
            deleted_at: invitation_entity.deleted_at,
        }
    }
}

#[derive(Debug)]
pub enum InvitationPositionDto {
    Owner,
    SubOwner,
}

impl From<InvitationPositionDto> for InvitationPosition {
    fn from(value: InvitationPositionDto) -> Self {
        match value {
            InvitationPositionDto::Owner => Self::Owner,
            InvitationPositionDto::SubOwner => Self::SubOwner,
        }
    }
}

impl From<InvitationPosition> for InvitationPositionDto {
    fn from(entity: InvitationPosition) -> Self {
        match entity {
            InvitationPosition::Owner => Self::Owner,
            InvitationPosition::SubOwner => Self::SubOwner,
        }
    }
}
