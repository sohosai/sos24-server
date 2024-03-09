use anyhow::Context;
use futures_util::{StreamExt, TryStreamExt};
use sos24_domain::{
    entity::{
        common::date::WithDate,
        news::NewsId,
        news_attachment::{NewsAttachment, NewsAttachmentId, NewsAttachmentUrl},
    },
    repository::news_attachment::NewsAttachmentRepository,
};
use sqlx::prelude::*;

use crate::postgresql::Postgresql;

#[derive(FromRow)]
pub struct NewsAttachmentRow {
    id: uuid::Uuid,
    news_id: uuid::Uuid,
    url: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl TryFrom<NewsAttachmentRow> for WithDate<NewsAttachment> {
    type Error = anyhow::Error;

    fn try_from(value: NewsAttachmentRow) -> Result<Self, Self::Error> {
        Ok(WithDate::new(
            NewsAttachment::new(
                NewsAttachmentId::new(value.id),
                NewsId::new(value.news_id),
                NewsAttachmentUrl::new(url::Url::parse(&value.url)?),
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
    async fn list(&self) -> anyhow::Result<Vec<WithDate<NewsAttachment>>> {
        let news_attachment_list = sqlx::query_as!(
            NewsAttachmentRow,
            r#"SELECT * FROM news_attachments WHERE deleted_at IS NULL"#
        )
        .fetch(&*self.db)
        .map(|row| WithDate::try_from(row?))
        .try_collect()
        .await
        .context("Failed to fetch news_attachment list")?;

        Ok(news_attachment_list)
    }

    async fn create(&self, news_attachment: NewsAttachment) -> anyhow::Result<()> {
        let news_attachment = news_attachment.destruct();

        sqlx::query!(
            r#"INSERT INTO news_attachments (id, news_id, url) VALUES ($1, $2, $3)"#,
            news_attachment.id.value(),
            news_attachment.news_id.value(),
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
    ) -> anyhow::Result<Option<WithDate<NewsAttachment>>> {
        let news_attachment_row = sqlx::query_as!(
            NewsAttachmentRow,
            r#"SELECT * FROM news_attachments WHERE id = $1 AND deleted_at IS NULL"#,
            id.value()
        )
        .fetch_optional(&*self.db)
        .await
        .context("Failed to fetch news_attachment")?;

        news_attachment_row.map(WithDate::try_from).transpose()
    }

    async fn delete_by_id(&self, _id: NewsAttachmentId) -> anyhow::Result<()> {
        unimplemented!("delete_by_id()");
    }
}
