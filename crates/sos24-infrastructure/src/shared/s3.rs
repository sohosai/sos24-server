use std::{ops::Deref, time::Duration};

use aws_sdk_s3::{
    config::{
        timeout::TimeoutConfig, Builder, Credentials, Region, RequestChecksumCalculation,
        StalledStreamProtectionConfig,
    },
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
            .timeout_config(
                TimeoutConfig::builder()
                    .operation_attempt_timeout(Duration::from_secs(60 * 10))
                    .build(),
            )
            // ファイルアップロード時に以下のエラーが出る問題のワークラウンド
            // "minimum throughput was specified at 1 B/s, but throughput of 0 B/s was observed"
            .stalled_stream_protection(
                StalledStreamProtectionConfig::enabled()
                    .upload_enabled(false)
                    .build(),
            )
            .request_checksum_calculation(RequestChecksumCalculation::WhenRequired)
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
