use crate::entity::{
    invitation::{Invitation, InvitationId, InvitationPosition},
    project::ProjectId,
    user::UserId,
};

use super::datetime;

pub fn id() -> InvitationId {
    InvitationId::new(uuid::Uuid::from_u128(1))
}

pub fn invitation(
    inviter: UserId,
    project_id: ProjectId,
    position: InvitationPosition,
) -> Invitation {
    Invitation::new(
        id(),
        inviter,
        project_id,
        position,
        None,
        datetime::now(),
        datetime::now(),
    )
}
