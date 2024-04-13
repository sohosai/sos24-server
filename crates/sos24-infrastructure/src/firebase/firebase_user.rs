use rs_firebase_admin_sdk::auth::UserUpdate;
use rs_firebase_admin_sdk::{
    auth::{FirebaseAuthService, NewUser},
    client::error::ApiClientError,
};

use sos24_domain::entity::firebase_user::FirebaseUserEmail;
use sos24_domain::{
    entity::firebase_user::{FirebaseUserId, NewFirebaseUser},
    repository::firebase_user::{FirebaseUserRepository, FirebaseUserRepositoryError},
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
    async fn create(
        &self,
        new_firebase_user: NewFirebaseUser,
    ) -> Result<FirebaseUserId, FirebaseUserRepositoryError> {
        let new_firebase_user = new_firebase_user.destruct();
        let created_user = self
            .auth
            .create_user(NewUser::email_and_password(
                new_firebase_user.email.clone().value(),
                new_firebase_user.password.value(),
            ))
            .await;

        match created_user {
            Ok(created_user) => Ok(FirebaseUserId::new(created_user.uid)),
            Err(err) => match err.current_context() {
                ApiClientError::ServerError(err) if err.message.as_str() == "EMAIL_EXISTS" => Err(
                    FirebaseUserRepositoryError::EmailExists(new_firebase_user.email),
                ),
                _ => Err(anyhow::anyhow!("Failed to create firebase user: {err}").into()),
            },
        }
    }

    async fn update_email_by_id(
        &self,
        id: FirebaseUserId,
        email: FirebaseUserEmail,
    ) -> Result<(), FirebaseUserRepositoryError> {
        let update = UserUpdate::builder(id.value()).email(email.value()).build();
        self.auth
            .update_user(update)
            .await
            .map(|_| ())
            .map_err(|err| anyhow::anyhow!("Failed to update firebase user: {err}").into())
    }

    async fn delete_by_id(&self, id: FirebaseUserId) -> Result<(), FirebaseUserRepositoryError> {
        self.auth
            .delete_user(id.value())
            .await
            .map_err(|err| anyhow::anyhow!("Failed to delete firebase user: {err}").into())
    }
}
