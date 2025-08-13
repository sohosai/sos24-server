use std::ops::Deref;

use mongodb::{options::ClientOptions, Client};

#[derive(Clone)]
pub struct MongoDb(mongodb::Database);

impl MongoDb {
    pub async fn new(db_url: &str, db_name: &str) -> anyhow::Result<Self> {
        tracing::info!("Connecting to MongoDB");

        let client_options = ClientOptions::parse(db_url).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(db_name);

        tracing::info!("Connected to MongoDB");
        Ok(Self(db))
    }

    /// Check if the MongoDB connection is healthy
    pub async fn health_check(&self) -> anyhow::Result<()> {
        self.0.run_command(mongodb::bson::doc! { "ping": 1 }).await?;
        Ok(())
    }
}

impl Deref for MongoDb {
    type Target = mongodb::Database;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
