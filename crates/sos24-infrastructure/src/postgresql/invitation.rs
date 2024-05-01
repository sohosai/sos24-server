use anyhow::Context;
use futures_util::{StreamExt, TryStreamExt};
use sqlx::prelude::{FromRow, Type};

use sos24_domain::{
    entity::{
        common::datetime::DateTime,
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
}

impl From<InvitationRow> for Invitation {
    fn from(row: InvitationRow) -> Self {
        Invitation::new(
            InvitationId::new(row.id),
            UserId::new(row.inviter),
            ProjectId::new(row.project_id),
            InvitationPosition::from(row.position),
            row.used_by.map(UserId::new),
            DateTime::new(row.created_at),
            DateTime::new(row.updated_at),
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
    async fn list(&self) -> Result<Vec<Invitation>, InvitationRepositoryError> {
        tracing::info!("招待一覧を取得します");

        let invitations_list = sqlx::query_as!(
            InvitationRow,
            r#"SELECT id, inviter, project_id, position AS "position: InvitationPositionRow", used_by, created_at, updated_at FROM invitations WHERE deleted_at IS NULL"#
        )
            .fetch(&*self.db)
            .map(|row| Ok::<_, anyhow::Error>(Invitation::from(row?)))
            .try_collect()
            .await
            .context("Failed to fetch invitations list")?;

        tracing::info!("招待一覧を取得しました");
        Ok(invitations_list)
    }

    async fn create(&self, invitation: Invitation) -> Result<(), InvitationRepositoryError> {
        tracing::info!("招待を作成します");

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

        tracing::info!("招待を作成しました");
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: InvitationId,
    ) -> Result<Option<Invitation>, InvitationRepositoryError> {
        tracing::info!("招待を取得します: {id:?}");

        let invitation_row = sqlx::query_as!(
            InvitationRow,
            r#"SELECT id, inviter, project_id, position AS "position: InvitationPositionRow", used_by, created_at, updated_at FROM invitations WHERE id = $1 AND deleted_at IS NULL"#,
            id.clone().value()
        )
            .fetch_optional(&*self.db)
            .await
            .context("Failed to fetch invitation")?;

        tracing::info!("招待を取得しました: {id:?}");
        Ok(invitation_row.map(Invitation::from))
    }

    async fn find_by_inviter(
        &self,
        inviter: UserId,
    ) -> Result<Vec<Invitation>, InvitationRepositoryError> {
        tracing::info!("招待を取得します: {inviter:?}");

        let invitation_list = sqlx::query_as!(
            InvitationRow,
            r#"SELECT id, inviter, project_id, position AS "position: InvitationPositionRow", used_by, created_at, updated_at FROM invitations WHERE inviter = $1 AND deleted_at IS NULL"#,
            inviter.clone().value(),
        )
            .fetch(&*self.db)
            .map(|row| Ok::<_, anyhow::Error>(Invitation::from(row?)))
            .try_collect()
            .await
            .context("Failed to fetch invitation")?;

        tracing::info!("招待を取得しました: {inviter:?}");
        Ok(invitation_list)
    }

    async fn update(&self, invitation: Invitation) -> Result<(), InvitationRepositoryError> {
        tracing::info!("招待を更新します");

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

        tracing::info!("招待を更新しました");
        Ok(())
    }

    async fn delete_by_id(&self, id: InvitationId) -> Result<(), InvitationRepositoryError> {
        tracing::info!("招待を削除します: {id:?}");

        sqlx::query!(
            r#"UPDATE invitations SET deleted_at = now() WHERE id = $1 AND deleted_at IS NULL"#,
            id.clone().value()
        )
        .execute(&*self.db)
        .await
        .context("Failed to delete invitation")?;

        tracing::info!("招待を削除しました: {id:?}");
        Ok(())
    }

    async fn delete_by_project_id(&self, id: ProjectId) -> Result<(), InvitationRepositoryError> {
        sqlx::query!(
            r#"UPDATE invitations SET deleted_at = now() WHERE project_id = $1 AND deleted_at IS NULL"#,
            id.value()
        )
            .execute(&*self.db)
            .await
            .context("Failed to delete invitation by project id")?;
        Ok(())
    }
}
