use sos24_domain::repository::HealthChecker;

use crate::shared::{mongodb::MongoDb, postgresql::Postgresql};

pub struct DatabaseHealthChecker {
    postgresql: Postgresql,
    mongodb: MongoDb,
}

impl DatabaseHealthChecker {
    pub fn new(postgresql: Postgresql, mongodb: MongoDb) -> Self {
        Self {
            postgresql,
            mongodb,
        }
    }
}

impl HealthChecker for DatabaseHealthChecker {
    async fn check_postgresql(&self) -> anyhow::Result<()> {
        self.postgresql.health_check().await
    }

    async fn check_mongodb(&self) -> anyhow::Result<()> {
        self.mongodb.health_check().await
    }
}