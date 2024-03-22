use anyhow::{anyhow, Context};
use futures_util::{StreamExt, TryStreamExt};
use sqlx::prelude::*;

use sos24_domain::entity::common::date::WithDate;
use sos24_domain::entity::news::{News, NewsBody, NewsId, NewsTitle};
use sos24_domain::entity::project::{ProjectAttributes, ProjectCategories};
use sos24_domain::repository::news::{NewsRepository, NewsRepositoryError};

use crate::postgresql::Postgresql;

#[derive(FromRow)]
pub struct NewsRow {
    id: uuid::Uuid,
    title: String,
    body: String,
    categories: i32,
    attributes: i32,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl TryFrom<NewsRow> for WithDate<News> {
    type Error = anyhow::Error;
    fn try_from(value: NewsRow) -> Result<Self, Self::Error> {
        Ok(WithDate::new(
            News::new(
                NewsId::new(value.id),
                NewsTitle::new(value.title),
                NewsBody::new(value.body),
                ProjectCategories::from_bits(value.categories as u32)
                    .ok_or(anyhow!("cannot convert project categories"))?,
                ProjectAttributes::from_bits(value.attributes as u32)
                    .ok_or(anyhow!("cannot convert project attributes"))?,
            ),
            value.created_at,
            value.updated_at,
            value.deleted_at,
        ))
    }
}

pub struct PgNewsRepository {
    db: Postgresql,
}

impl PgNewsRepository {
    pub fn new(db: Postgresql) -> Self {
        Self { db }
    }
}

impl NewsRepository for PgNewsRepository {
    async fn list(&self) -> Result<Vec<WithDate<News>>, NewsRepositoryError> {
        let news_list = sqlx::query_as!(NewsRow, r#"SELECT * FROM news WHERE deleted_at IS NULL"#)
            .fetch(&*self.db)
            .map(|row| WithDate::try_from(row?))
            .try_collect()
            .await
            .context("Failed to fetch news list")?;
        Ok(news_list)
    }

    async fn create(&self, news: News) -> Result<(), NewsRepositoryError> {
        let news = news.destruct();
        sqlx::query!(
            r#"INSERT INTO news (id, title, body, categories, attributes) VALUES ($1, $2, $3, $4, $5)"#,
            news.id.value(),
            news.title.value(),
            news.body.value(),
            news.categories.bits() as i32,
            news.attributes.bits() as i32,
        )
            .execute(&*self.db)
            .await
            .context("Failed to create news")?;
        Ok(())
    }

    async fn find_by_id(&self, id: NewsId) -> Result<Option<WithDate<News>>, NewsRepositoryError> {
        let news_row = sqlx::query_as!(
            NewsRow,
            r#"SELECT * FROM news WHERE id = $1 AND deleted_at IS NULL"#,
            id.value()
        )
        .fetch_optional(&*self.db)
        .await
        .context("Failed to fetch news")?;
        Ok(news_row.map(|news| WithDate::try_from(news)).transpose()?)
    }

    async fn update(&self, news: News) -> Result<(), NewsRepositoryError> {
        let news = news.destruct();
        sqlx::query!(
            r#"UPDATE news SET title = $2, body = $3, categories = $4, attributes = $5 WHERE id = $1 and deleted_at IS NULL"#,
            news.id.value(),
            news.title.value(),
            news.body.value(),
            news.categories.bits() as i32,
            news.attributes.bits() as i32,
        )
            .execute(&*self.db)
            .await
            .context("Failed to update news")?;
        Ok(())
    }

    async fn delete_by_id(&self, id: NewsId) -> Result<(), NewsRepositoryError> {
        sqlx::query!(
            r#"UPDATE news SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL"#,
            id.value()
        )
        .execute(&*self.db)
        .await
        .context("Failed to delete news")?;
        Ok(())
    }
}
