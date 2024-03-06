use anyhow::Context;
use futures_util::{StreamExt, TryStreamExt};
use sqlx::prelude::*;

use sos24_domain::entity::news::{News, NewsBody, NewsCategories, NewsId, NewsTitle};
use sos24_domain::repository::news::NewsRepository;

use crate::postgresql::Postgresql;

#[derive(FromRow)]
pub struct NewsRow {
    id: uuid::Uuid,
    title: String,
    body: String,
    categories: i32,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<NewsRow> for News {
    fn from(value: NewsRow) -> Self {
        News::new(
            NewsId::new(value.id),
            NewsTitle::new(value.title),
            NewsBody::new(value.body),
            NewsCategories::new(value.categories),
            value.created_at,
            value.updated_at,
            value.deleted_at,
        )
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
    async fn list(&self) -> anyhow::Result<Vec<News>> {
        let news_list = sqlx::query_as!(NewsRow, r#"SELECT * FROM news WHERE deleted_at IS NULL"#)
            .fetch(&*self.db)
            .map(|row| Ok::<_, anyhow::Error>(News::from(row?)))
            .try_collect()
            .await
            .context("Failed to fetch news list")?;
        Ok(news_list)
    }

    async fn create(&self, news: News) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO news (id, title, body, categories, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6)"#,
            news.id.value(),
            news.title.value(),
            news.body.value(),
            news.categories.value(),
            news.created_at,
            news.updated_at,
        )
        .execute(&*self.db)
        .await
        .context("Failed to create news")?;
        Ok(())
    }

    async fn find_by_id(&self, id: NewsId) -> anyhow::Result<Option<News>> {
        let news_row = sqlx::query_as!(
            NewsRow,
            r#"SELECT * FROM news WHERE id = $1 AND deleted_at IS NULL"#,
            id.value()
        )
        .fetch_optional(&*self.db)
        .await
        .context("Failed to fetch news")?;

        news_row
            .map(|row| Ok::<_, anyhow::Error>(News::from(row)))
            .transpose()
    }

    async fn update(&self, news: News) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE news SET title = $2, body = $3, categories = $4, updated_at = NOW() WHERE id = $1 and deleted_at IS NULL"#,
            news.id.value(),
            news.title.value(),
            news.body.value(),
            news.categories.value(),
        )
        .execute(&*self.db)
        .await
        .context("Failed to update news")?;
        Ok(())
    }

    async fn delete_by_id(&self, id: NewsId) -> anyhow::Result<()> {
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
