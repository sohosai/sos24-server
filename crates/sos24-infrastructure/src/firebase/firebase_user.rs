use rs_firebase_admin_sdk::auth::{FirebaseAuthService, NewUser};
use sos24_domain::{
    entity::firebase_user::{FirebaseUserId, NewFirebaseUser},
    repository::firebase_user::FirebaseUserRepository,
};

use super::FirebaseAuth;

pub struct FirebaseUserRepositoryImpl {
    auth: FirebaseAuth,
}

impl FirebaseUserRepositoryImpl {
    pub fn new(auth: FirebaseAuth) -> Self {
        Self { auth }
    }
}

impl FirebaseUserRepository for FirebaseUserRepositoryImpl {
    async fn create(&self, new_firebase_user: NewFirebaseUser) -> anyhow::Result<FirebaseUserId> {
        let created_user = self
            .auth
            .create_user(NewUser::email_and_password(
                new_firebase_user.email.value(),
                new_firebase_user.password.value(),
            ))
            .await
            .map_err(|err| anyhow::anyhow!("Failed to create firebase user: {err}"))?;

        Ok(FirebaseUserId::new(created_user.uid))
    }

    async fn delete_by_id(&self, id: FirebaseUserId) -> anyhow::Result<()> {
        self.auth
            .delete_user(id.value())
            .await
            .map_err(|err| anyhow::anyhow!("Failed to delete firebase user: {err}"))
    }
}
