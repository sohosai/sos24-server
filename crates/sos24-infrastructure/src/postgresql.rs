use std::ops::Deref;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub mod invitation;
pub mod news;
pub mod project;
pub mod user;

#[derive(Clone)]
pub struct Postgresql(pub(crate) PgPool);

impl Postgresql {
    pub async fn new(db_url: &str) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(8)
            .connect(db_url)
            .await?;

        sqlx::migrate!("../../migrations").run(&pool).await?;

        Ok(Self(pool))
    }
}

impl Deref for Postgresql {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
