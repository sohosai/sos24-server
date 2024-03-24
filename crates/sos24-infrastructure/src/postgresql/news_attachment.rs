use anyhow::Context;
use futures_util::{StreamExt, TryStreamExt};
use sos24_domain::{
    entity::{
        common::date::WithDate,
        news_attachment_data::{NewsAttachmentData, NewsAttachmentFilename, NewsAttachmentId},
        news_attachment_object::NewsAttachmentObjectKey,
    },
    repository::news_attachment_data::{NewsAttachmentRepository, NewsAttachmentRepositoryError},
};
use sqlx::prelude::*;

use crate::postgresql::Postgresql;

#[derive(FromRow)]
pub struct NewsAttachmentRow {
    id: uuid::Uuid,
    name: String,
    url: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl TryFrom<NewsAttachmentRow> for WithDate<NewsAttachmentData> {
    type Error = anyhow::Error;

    fn try_from(value: NewsAttachmentRow) -> Result<Self, Self::Error> {
        Ok(WithDate::new(
            NewsAttachmentData::new(
                NewsAttachmentId::new(value.id),
                NewsAttachmentFilename::new(value.name),
                NewsAttachmentObjectKey::new(value.url),
            ),
            value.created_at,
            value.updated_at,
            value.deleted_at,
        ))
    }
}

pub struct PgNewsAttachmentRepository {
    db: Postgresql,
}

impl PgNewsAttachmentRepository {
    pub fn new(db: Postgresql) -> Self {
        Self { db }
    }
}

impl NewsAttachmentRepository for PgNewsAttachmentRepository {
    async fn list(
        &self,
    ) -> Result<Vec<WithDate<NewsAttachmentData>>, NewsAttachmentRepositoryError> {
        let news_attachment_list = sqlx::query_as!(
            NewsAttachmentRow,
            r#"SELECT * FROM files WHERE deleted_at IS NULL"#
        )
        .fetch(&*self.db)
        .map(|row| WithDate::try_from(row?))
        .try_collect()
        .await
        .context("Failed to fetch news_attachment list")?;

        Ok(news_attachment_list)
    }

    async fn create(
        &self,
        news_attachment: NewsAttachmentData,
    ) -> Result<(), NewsAttachmentRepositoryError> {
        let news_attachment = news_attachment.destruct();

        sqlx::query!(
            r#"INSERT INTO files (id, name, url) VALUES ($1, $2, $3)"#,
            news_attachment.id.value(),
            news_attachment.name.value(),
            news_attachment.url.value().to_string(),
        )
        .execute(&*self.db)
        .await
        .context("Failed to create news_attachment")?;

        Ok(())
    }

    async fn find_by_id(
        &self,
        id: NewsAttachmentId,
    ) -> Result<Option<WithDate<NewsAttachmentData>>, NewsAttachmentRepositoryError> {
        let news_attachment_row = sqlx::query_as!(
            NewsAttachmentRow,
            r#"SELECT * FROM files WHERE id = $1 AND deleted_at IS NULL"#,
            id.value()
        )
        .fetch_optional(&*self.db)
        .await
        .context("Failed to fetch news_attachment")?;

        Ok(news_attachment_row.map(WithDate::try_from).transpose()?)
    }

    async fn delete_by_id(
        &self,
        id: NewsAttachmentId,
    ) -> Result<(), NewsAttachmentRepositoryError> {
        sqlx::query!(
            r#"UPDATE files SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL"#,
            id.value()
        )
        .execute(&*self.db)
        .await
        .context("Failed to delete news attachment")?;
        Ok(())
    }
}
