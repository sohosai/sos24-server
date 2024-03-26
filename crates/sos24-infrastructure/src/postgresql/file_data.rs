use anyhow::Context;
use futures_util::{StreamExt, TryStreamExt};
use sos24_domain::{
    entity::{
        common::date::WithDate,
        file_data::{FileData, FileId, FileName},
        file_object::FileObjectKey,
    },
    repository::file_data::{FileDataRepository, FileDataRepositoryError},
};
use sqlx::prelude::*;

use crate::postgresql::Postgresql;

#[derive(FromRow)]
pub struct FileDataRow {
    id: uuid::Uuid,
    name: String,
    url: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl TryFrom<FileDataRow> for WithDate<FileData> {
    type Error = anyhow::Error;

    fn try_from(value: FileDataRow) -> Result<Self, Self::Error> {
        Ok(WithDate::new(
            FileData::new(
                FileId::new(value.id),
                FileName::new(value.name),
                FileObjectKey::new(value.url),
            ),
            value.created_at,
            value.updated_at,
            value.deleted_at,
        ))
    }
}

pub struct PgFileDataRepository {
    db: Postgresql,
}

impl PgFileDataRepository {
    pub fn new(db: Postgresql) -> Self {
        Self { db }
    }
}

impl FileDataRepository for PgFileDataRepository {
    async fn list(&self) -> Result<Vec<WithDate<FileData>>, FileDataRepositoryError> {
        let file_data_list = sqlx::query_as!(
            FileDataRow,
            r#"SELECT * FROM files WHERE deleted_at IS NULL"#
        )
        .fetch(&*self.db)
        .map(|row| WithDate::try_from(row?))
        .try_collect()
        .await
        .context("Failed to fetch file data list")?;

        Ok(file_data_list)
    }

    async fn create(&self, file_data: FileData) -> Result<(), FileDataRepositoryError> {
        let file_data = file_data.destruct();

        sqlx::query!(
            r#"INSERT INTO files (id, name, url) VALUES ($1, $2, $3)"#,
            file_data.id.value(),
            file_data.name.value(),
            file_data.url.value().to_string(),
        )
        .execute(&*self.db)
        .await
        .context("Failed to create file data")?;

        Ok(())
    }

    async fn find_by_id(
        &self,
        id: FileId,
    ) -> Result<Option<WithDate<FileData>>, FileDataRepositoryError> {
        let file_data_row = sqlx::query_as!(
            FileDataRow,
            r#"SELECT * FROM files WHERE id = $1 AND deleted_at IS NULL"#,
            id.value()
        )
        .fetch_optional(&*self.db)
        .await
        .context("Failed to fetch file data")?;

        Ok(file_data_row.map(WithDate::try_from).transpose()?)
    }

    async fn delete_by_id(&self, id: FileId) -> Result<(), FileDataRepositoryError> {
        sqlx::query!(
            r#"UPDATE files SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL"#,
            id.value()
        )
        .execute(&*self.db)
        .await
        .context("Failed to delete file data")?;
        Ok(())
    }
}
