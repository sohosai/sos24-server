use std::ops::Deref;

use rs_firebase_admin_sdk::{App, CustomServiceAccount, LiveAuthAdmin};

pub mod firebase_user;

pub struct FirebaseAuth(LiveAuthAdmin);

impl FirebaseAuth {
    pub async fn new(service_account_key: &str) -> anyhow::Result<Self> {
        tracing::info!("Initializing Firebase Auth");

        let gcp_service_account = CustomServiceAccount::from_json(service_account_key)?;
        let live_app = App::live(gcp_service_account.into())
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
