use axum::{debug_handler, extract::State, http::StatusCode, response::IntoResponse, Json};
use rs_firebase_admin_sdk::{
    auth::{FirebaseAuthService, NewUser},
    App, CustomServiceAccount,
};
use serde::{Deserialize, Serialize};

use crate::{
    repository::{self, users::CreateUserInput},
    AppState,
};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub(crate) struct PostUsersInput {
    name: String,
    kana_name: String,

    email: String,
    password: String,

    phone_number: String,

    category: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FirebaseSignupResponse {
    id_token: String,
    email: String,
    refresh_token: String,
    expires_in: String,
    local_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) id: String,

    pub(crate) name: String,
    pub(crate) kana_name: String,

    pub(crate) email: String,
    pub(crate) phone_number: String,
    pub(crate) role: UserRole,
    pub(crate) category: UserCategory,

    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
    pub(crate) updated_at: chrono::DateTime<chrono::Utc>,
    pub(crate) deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub(crate) enum UserRole {
    General,
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "user_category", rename_all = "snake_case")]
pub(crate) enum UserCategory {
    UndergraduateStudent,
    GraduateStudent,
    AcademicStaff,
}

#[debug_handler]
pub(crate) async fn handle_post_users(
    State(app_state): State<AppState>,
    Json(input): Json<PostUsersInput>,
) -> Result<impl IntoResponse, StatusCode> {
    if !is_valid_email_format(&input.email) {
        tracing::error!("Invalid email format: {}", input.email);
        return Err(StatusCode::BAD_REQUEST);
    }

    if !&input.email.ends_with("tsukuba.ac.jp") {
        tracing::error!("Invalid email domain: {}", input.email);
        return Err(StatusCode::BAD_REQUEST);
    }

    let gcp_service_account =
        CustomServiceAccount::from_json(&app_state.config.firebase_service_account_key).unwrap();
    let live_app = App::live(gcp_service_account.into()).await.unwrap();
    let live_auth_admin = live_app.auth();

    let new_firebase_user = live_auth_admin
        .create_user(NewUser::email_and_password(
            input.email.clone(),
            input.password.clone(),
        ))
        .await
        .map_err(|e| {
            tracing::error!("Failed to create firebase user: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let resp = repository::users::create_user(
        &app_state.pool,
        CreateUserInput {
            id: new_firebase_user.uid.clone(),
            name: input.name,
            kana_name: input.kana_name,
            email: input.email,
            phone_number: input.phone_number,
            category: input.category,
        },
    )
    .await;

    match resp {
        Ok(user) => Ok(Json(user)),
        Err(e) => {
            tracing::error!("Failed to create user: {e}");

            live_auth_admin
                .delete_user(new_firebase_user.uid.clone())
                .await
                .map_err(|e| {
                    tracing::error!("Failed to delete firebase user: {e}");
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;

            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn is_valid_email_format(email: &str) -> bool {
    // https://html.spec.whatwg.org/multipage/input.html#valid-e-mail-address
    let email_re = regex::Regex::new(r"^[a-zA-Z0-9.!#$%&'*+\/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();
    email_re.is_match(email)
}

#[cfg(test)]
mod test {
    #[test]
    fn valid_email_format() {
        assert!(super::is_valid_email_format("s0000000@u.tsukuba.ac.jp"));
        assert!(super::is_valid_email_format("john.doe@example.jp"));
    }

    #[test]
    fn invalid_email_format() {
        assert!(!super::is_valid_email_format("s@@example.com"));
        assert!(!super::is_valid_email_format("s@u."));
        assert!(!super::is_valid_email_format("example.com"));
    }
}
