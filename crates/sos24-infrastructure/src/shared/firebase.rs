use std::ops::Deref;

use rs_firebase_admin_sdk::{credentials_provider, App, LiveAuthAdmin};

pub struct FirebaseAuth(LiveAuthAdmin);

const ENV_KEY: &str = "GOOGLE_APPLICATION_CREDENTIALS";

impl FirebaseAuth {
    pub async fn new(service_account_key: &str) -> anyhow::Result<Self> {
        tracing::info!("Initializing Firebase Auth");

        // credentials_provider()は環境変数からクレデンシャルを取得するので、一時的に環境変数を設定する
        // ref: https://docs.rs/gcp_auth/0.12.3/gcp_auth/fn.provider.html
        std::env::set_var(ENV_KEY, service_account_key);
        let gcp_service_account = credentials_provider().await?;
        std::env::remove_var(ENV_KEY);

        let live_app = App::live(gcp_service_account)
            .await
            .map_err(|err| anyhow::anyhow!("{}", err))?;
        let live_auth_app = live_app.auth();

        tracing::info!("Firebase Auth initialized");
        Ok(Self(live_auth_app))
    }
}

impl Deref for FirebaseAuth {
    type Target = LiveAuthAdmin;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
