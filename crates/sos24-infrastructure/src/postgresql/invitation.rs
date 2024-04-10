use anyhow::Context;
use futures_util::{StreamExt, TryStreamExt};
use sqlx::prelude::{FromRow, Type};

use sos24_domain::{
    entity::{
        common::date::WithDate,
        invitation::{Invitation, InvitationId, InvitationPosition},
        project::ProjectId,
        user::UserId,
    },
    repository::invitation::{InvitationRepository, InvitationRepositoryError},
};

use super::Postgresql;

#[derive(FromRow)]
pub struct InvitationRow {
    id: uuid::Uuid,
    inviter: String,
    project_id: uuid::Uuid,
    position: InvitationPositionRow,
    used_by: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<InvitationRow> for WithDate<Invitation> {
    fn from(row: InvitationRow) -> Self {
        WithDate::new(
            Invitation::new(
                InvitationId::new(row.id),
                UserId::new(row.inviter),
                ProjectId::new(row.project_id),
                InvitationPosition::from(row.position),
                row.used_by.map(UserId::new),
            ),
            row.created_at,
            row.updated_at,
            row.deleted_at,
        )
    }
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

impl From<InvitationPositionRow> for InvitationPosition {
    fn from(position: InvitationPositionRow) -> Self {
        match position {
            InvitationPositionRow::Owner => Self::Owner,
            InvitationPositionRow::SubOwner => Self::SubOwner,
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
    async fn list(&self) -> Result<Vec<WithDate<Invitation>>, InvitationRepositoryError> {
        let invitations_list = sqlx::query_as!(
            InvitationRow,
            r#"SELECT id, inviter, project_id, position AS "position: InvitationPositionRow", used_by, created_at, updated_at, deleted_at FROM invitations WHERE deleted_at IS NULL"#
        )
            .fetch(&*self.db)
            .map(|row| Ok::<_, anyhow::Error>(WithDate::from(row?)))
            .try_collect()
            .await
            .context("Failed to fetch invitations list")?;
        Ok(invitations_list)
    }

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

    async fn find_by_id(
        &self,
        id: InvitationId,
    ) -> Result<Option<WithDate<Invitation>>, InvitationRepositoryError> {
        let invitation_row = sqlx::query_as!(
            InvitationRow,
            r#"SELECT id, inviter, project_id, position AS "position: InvitationPositionRow", used_by, created_at, updated_at, deleted_at FROM invitations WHERE id = $1 AND deleted_at IS NULL"#,
            id.value()
        )
            .fetch_optional(&*self.db)
            .await
            .context("Failed to fetch invitation")?;
        Ok(invitation_row.map(WithDate::from))
    }

    async fn find_by_inviter(
        &self,
        inviter: UserId,
    ) -> Result<Vec<WithDate<Invitation>>, InvitationRepositoryError> {
        let invitation_list = sqlx::query_as!(
            InvitationRow,
            r#"SELECT id, inviter, project_id, position AS "position: InvitationPositionRow", used_by, created_at, updated_at, deleted_at FROM invitations WHERE inviter = $1 AND deleted_at IS NULL"#,
            inviter.value(),
        )
            .fetch(&*self.db)
            .map(|row| Ok::<_, anyhow::Error>(WithDate::from(row?)))
            .try_collect()
            .await
            .context("Failed to fetch invitation")?;
        Ok(invitation_list)
    }

    async fn update(&self, invitation: Invitation) -> Result<(), InvitationRepositoryError> {
        let invitation = invitation.destruct();
        sqlx::query!(
            r#"UPDATE invitations SET inviter = $2, project_id = $3, position = $4, used_by = $5 WHERE id = $1 AND deleted_at IS NULL"#,
            invitation.id.value(),
            invitation.inviter.value(),
            invitation.project_id.value(),
            InvitationPositionRow::from(invitation.position) as InvitationPositionRow,
            invitation.used_by.map(|id| id.value()),
        )
            .execute(&*self.db)
            .await
            .context("Failed to update invitation")?;
        Ok(())
    }

    async fn delete_by_id(&self, id: InvitationId) -> Result<(), InvitationRepositoryError> {
        sqlx::query!(
            r#"UPDATE invitations SET deleted_at = now() WHERE id = $1 AND deleted_at IS NULL"#,
            id.value()
        )
        .execute(&*self.db)
        .await
        .context("Failed to delete invitation")?;
        Ok(())
    }
}
