use anyhow::Context;
use sos24_domain::{
    entity::invitation::{Invitation, InvitationPosition},
    repository::invitation::{InvitationRepository, InvitationRepositoryError},
};
use sqlx::prelude::{FromRow, Type};

use super::Postgresql;

#[derive(FromRow)]
pub struct InvitationRow {
    id: uuid::Uuid,
    inviter: uuid::Uuid,
    project_id: uuid::Uuid,
    position: InvitationPositionRow,
}

#[derive(Type)]
#[sqlx(type_name = "invitation_position", rename_all = "snake_case")]
pub enum InvitationPositionRow {
    Owner,
    SubOwner,
}

impl From<InvitationPosition> for InvitationPositionRow {
    fn from(position: InvitationPosition) -> Self {
        match position {
            InvitationPosition::Owner => Self::Owner,
            InvitationPosition::SubOwner => Self::SubOwner,
        }
    }
}

pub struct PgInvitationRepository {
    db: Postgresql,
}

impl PgInvitationRepository {
    pub fn new(db: Postgresql) -> Self {
        Self { db }
    }
}

impl InvitationRepository for PgInvitationRepository {
    async fn create(&self, invitation: Invitation) -> Result<(), InvitationRepositoryError> {
        let invitation = invitation.destruct();
        sqlx::query!(
            r#"INSERT INTO invitations (id, inviter, project_id, position) VALUES ($1, $2, $3, $4)"#,
            invitation.id.value(),
            invitation.inviter.value(),
            invitation.project_id.value(),
            InvitationPositionRow::from(invitation.position) as InvitationPositionRow,
        )
        .execute(&*self.db)
        .await
        .context("Failed to create invitation")?;
        Ok(())
    }
}
