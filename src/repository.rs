use std::env;

use anyhow::{Context, Result};
use mongodb::{options::ClientOptions, Client, Database};
use sqlx::{postgres::PgPoolOptions, PgPool};

pub(crate) mod users;

pub async fn get_pg_pool() -> Result<PgPool> {
    let pg_db_url = env::var("POSTGRES_DB_URL").expect("env `POSTGRES_DB_URL` must be set");

    let pg_pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&pg_db_url)
        .await
        .context("Couldn't connect to the DB")?;

    sqlx::migrate!().run(&pg_pool).await?;

    Ok(pg_pool)
}

pub async fn get_mongo_db() -> Result<Database> {
    let mongo_database_url = env::var("MONGO_DB_URL").context("env `MONGO_DB_URL` doesn't set")?;
    let client_options = ClientOptions::parse(mongo_database_url).await?;
    let client = Client::with_options(client_options)?;

    let mongo_db_name = env::var("MONGO_DB").context("env `MONGO_DB` must be set")?;
    let mongo_db = client.database(&mongo_db_name);

    Ok(mongo_db)
}
