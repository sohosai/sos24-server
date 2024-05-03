use std::time::Duration;

use anyhow::Context;
use reqwest::ClientBuilder;
use serde_json::json;
use sos24_use_case::shared::adapter::notification::Notifier;

pub struct SlackNotifier {
    webhook_url: Option<String>,
}

impl SlackNotifier {
    pub fn new(webhook_url: Option<String>) -> Self {
        let webhook_url = match webhook_url {
            Some(url) => {
                // 有効なURLかどうかをチェックする
                let _ = reqwest::Url::parse(&url).expect("Invalid URL");
                Some(url)
            }
            None => None,
        };

        Self { webhook_url }
    }
}

impl Notifier for SlackNotifier {
    async fn notify(&self, message: String) -> anyhow::Result<()> {
        tracing::info!("Slack通知を送信します");

        let Some(ref webhook_url) = self.webhook_url else {
            tracing::warn!("SlackのWebhook URLが設定されていないため、通知を送信しませんでした");
            return Ok(());
        };

        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(60))
            .build()
            .context("Failed to create HTTP client")?;

        let body = json!({"text": message});
        client.post(webhook_url).json(&body).send().await?;

        tracing::info!("Slack通知を送信しました");
        Ok(())
    }
}
