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
        tracing::info!("Firebaseのユーザーを作成します");

        let new_firebase_user = new_firebase_user.destruct();
        let created_user = self
            .auth
            .create_user(NewUser::email_and_password(
                new_firebase_user.email.clone().value(),
                new_firebase_user.password.value(),
            ))
            .await;

        tracing::info!("Firebaseのユーザー作成が完了しました");
        match created_user {
            Ok(created_user) => Ok(FirebaseUserId::new(created_user.uid)),
            Err(err) => match err.current_context() {
                ApiClientError::ServerError(err) if err.message.as_str() == "EMAIL_EXISTS" => Err(
                    FirebaseUserRepositoryError::EmailExists(new_firebase_user.email),
                ),
                ApiClientError::ServerError(err) if err.message.as_str() == "WEAK_PASSWORD" => {
                    Err(FirebaseUserRepositoryError::WeakPassword)
                }
                _ => Err(anyhow::anyhow!("Failed to create firebase user: {err}").into()),
            },
        }
    }

    async fn update_email_by_id(
        &self,
        id: FirebaseUserId,
        email: FirebaseUserEmail,
    ) -> Result<(), FirebaseUserRepositoryError> {
        tracing::info!("Firebaseのユーザーのメールアドレスを更新します: {id:?}");

        let update = UserUpdate::builder(id.clone().value())
            .email(email.value())
            .build();

        let res = self.auth.update_user(update).await;

        match res {
            Ok(_) => {
                tracing::info!("Firebaseのユーザーのメールアドレスの更新が完了しました: {id:?}");
                Ok(())
            }
            Err(err) => Err(anyhow::anyhow!("Failed to update firebase user: {err}").into()),
        }
    }

    async fn delete_by_id(&self, id: FirebaseUserId) -> Result<(), FirebaseUserRepositoryError> {
        tracing::info!("Firebaseのユーザーを削除します: {id:?}");

        let res = self.auth.delete_user(id.clone().value()).await;

        match res {
            Ok(_) => {
                tracing::info!("Firebaseのユーザーの削除が完了しました: {id:?}");
                Ok(())
            }
            Err(err) => Err(anyhow::anyhow!("Failed to delete firebase user: {err}").into()),
        }
    }
}
