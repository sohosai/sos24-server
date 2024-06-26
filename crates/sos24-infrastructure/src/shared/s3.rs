use std::ops::Deref;

use aws_sdk_s3::{
    config::{Builder, Credentials, Region},
    Client,
};

#[derive(Clone)]
pub struct S3(Client);

impl S3 {
    pub async fn new(
        endpoint: &str,
        region: &str,
        access_key_id: &str,
        secret_access_key: &str,
    ) -> Self {
        tracing::info!("Initializing S3 client");

        let credential = Credentials::new(
            access_key_id,
            secret_access_key,
            None,
            None,
            "loaded-from-env",
        );
        let config = Builder::new()
            .endpoint_url(endpoint)
            .region(Region::new(region.to_string()))
            .credentials_provider(credential)
            .behavior_version_latest()
            .build();

        tracing::info!("S3 client initialized");
        Self(Client::from_conf(config))
    }
}

impl Deref for S3 {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
