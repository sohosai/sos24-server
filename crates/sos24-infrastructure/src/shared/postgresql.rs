use std::ops::Deref;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[derive(Clone)]
pub struct Postgresql(pub(crate) PgPool);

impl Postgresql {
    pub async fn new(db_url: &str) -> anyhow::Result<Self> {
        tracing::info!("Connecting to PostgreSQL");

        let pool = PgPoolOptions::new()
            .max_connections(8)
            .connect(db_url)
            .await?;

        tracing::info!("Running migrations");
        sqlx::migrate!("../../migrations").run(&pool).await?;

        tracing::info!("Connected to PostgreSQL");
        Ok(Self(pool))
    }
}

impl Deref for Postgresql {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
