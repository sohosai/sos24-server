use anyhow::{anyhow, Context};
use futures_util::{StreamExt, TryStreamExt};
use sos24_domain::entity::common::datetime::DateTime;
use sqlx::prelude::*;

use sos24_domain::entity::file_data::FileId;
use sos24_domain::entity::news::{News, NewsBody, NewsId, NewsState, NewsTitle};
use sos24_domain::entity::project::{ProjectAttributes, ProjectCategories};
use sos24_domain::repository::news::{NewsRepository, NewsRepositoryError};
use sqlx::types::chrono;

use crate::shared::postgresql::Postgresql;

#[derive(FromRow)]
pub struct NewsRow {
    id: uuid::Uuid,
    state: NewsStateRow,
    title: String,
    body: String,
    attachments: Vec<uuid::Uuid>,
    categories: i32,
    attributes: i32,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    scheduled_at: Option<chrono::DateTime<chrono::Utc>>, // SQL NULL maps Rust Option::None,
                                                         // required only when state is Scheduled,
                                                         // otherwise, None
}

#[derive(Type)]
#[sqlx(type_name = "news_state", rename_all = "snake_case")]
pub enum NewsStateRow {
    Draft,
    Scheduled,
    Published,
}

impl From<NewsState> for NewsStateRow {
    fn from(value: NewsState) -> Self {
        match value {
            NewsState::Draft => NewsStateRow::Draft,
            NewsState::Scheduled(_) => NewsStateRow::Scheduled,
            NewsState::Published => NewsStateRow::Published,
        }
    }
}

impl TryFrom<NewsRow> for News {
    type Error = anyhow::Error;
    fn try_from(value: NewsRow) -> Result<Self, Self::Error> {
        Ok(News::new(
            NewsId::new(value.id),
            match value.state {
                NewsStateRow::Draft => NewsState::Draft,
                NewsStateRow::Scheduled => NewsState::Scheduled(DateTime::new(
                    value
                        .scheduled_at
                        .ok_or(anyhow!("cannot convert news state"))?,
                )),
                NewsStateRow::Published => NewsState::Published,
            },
            NewsTitle::new(value.title),
            NewsBody::new(value.body),
            value.attachments.into_iter().map(FileId::new).collect(),
            ProjectCategories::from_bits(value.categories as u32)
                .ok_or(anyhow!("cannot convert project categories"))?,
            ProjectAttributes::from_bits(value.attributes as u32)
                .ok_or(anyhow!("cannot convert project attributes"))?,
            DateTime::new(value.created_at),
            DateTime::new(value.updated_at),
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
    async fn list(&self) -> Result<Vec<News>, NewsRepositoryError> {
        tracing::info!("お知らせ一覧を取得します");

        let news_list = sqlx::query_as!(
            NewsRow,
            r#"SELECT id, state AS "state: NewsStateRow", title, body, attachments, categories, attributes, created_at, updated_at, scheduled_at FROM news WHERE deleted_at IS NULL ORDER BY created_at DESC"#
        )
        .fetch(&*self.db)
        .map(|row| News::try_from(row?))
        .try_collect()
        .await
        .context("Failed to fetch news list")?;

        tracing::info!("お知らせ一覧を取得しました");
        Ok(news_list)
    }

    async fn create(&self, news: News) -> Result<(), NewsRepositoryError> {
        tracing::info!("お知らせを作成します");

        let news = news.destruct();
        sqlx::query!(
            r#"INSERT INTO news (id, state, title, body, attachments, categories, attributes, scheduled_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
            news.id.value(),
            NewsStateRow::from(news.state.clone()) as NewsStateRow,
            news.title.value(),
            news.body.value(),
            &news.attachments.into_iter().map(|id| id.value()).collect::<Vec<_>>(),
            news.categories.bits() as i32,
            news.attributes.bits() as i32,
            match news.state {
                NewsState::Scheduled(date) => Some(date.value()),
                _ => None
            }
        )
            .execute(&*self.db)
            .await
            .context("Failed to create news")?;

        tracing::info!("お知らせを作成しました");
        Ok(())
    }

    async fn find_by_id(&self, id: NewsId) -> Result<Option<News>, NewsRepositoryError> {
        tracing::info!("お知らせを取得します: {id:?}");

        let news_row = sqlx::query_as!(
            NewsRow,
            r#"SELECT id,  state AS "state: NewsStateRow", title, body, attachments, categories, attributes, created_at, updated_at, scheduled_at FROM news WHERE id = $1 AND deleted_at IS NULL"#,
            id.clone().value()
        )
        .fetch_optional(&*self.db)
        .await
        .context("Failed to fetch news")?;

        tracing::info!("お知らせを取得しました: {id:?}");
        Ok(news_row.map(News::try_from).transpose()?)
    }

    async fn update(&self, news: News) -> Result<(), NewsRepositoryError> {
        tracing::info!("お知らせを更新します");

        let news = news.destruct();
        sqlx::query!(
            r#"UPDATE news SET state = $2, title = $3, body = $4, attachments = $5, categories = $6, attributes = $7, scheduled_at = $8 WHERE id = $1 and deleted_at IS NULL"#,
            news.id.value(),
            NewsStateRow::from(news.state.clone()) as NewsStateRow,
            news.title.value(),
            news.body.value(),
            &news.attachments.into_iter().map(|id| id.value()).collect::<Vec<_>>(),
            news.categories.bits() as i32,
            news.attributes.bits() as i32,
            match news.state {
                NewsState::Scheduled(date) => Some(date.value()),
                _ => None
            }
        )
            .execute(&*self.db)
            .await
            .context("Failed to update news")?;

        tracing::info!("お知らせを更新しました");
        Ok(())
    }

    async fn delete_by_id(&self, id: NewsId) -> Result<(), NewsRepositoryError> {
        tracing::info!("お知らせを削除します: {id:?}");

        sqlx::query!(
            r#"UPDATE news SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL"#,
            id.clone().value()
        )
        .execute(&*self.db)
        .await
        .context("Failed to delete news")?;

        tracing::info!("お知らせを削除しました: {id:?}");
        Ok(())
    }
}
