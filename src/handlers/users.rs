use axum::{debug_handler, extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    auth,
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
    role: String,
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
    pub(crate) role: String,
    pub(crate) category: String,

    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
    pub(crate) updated_at: chrono::DateTime<chrono::Utc>,
    pub(crate) deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[debug_handler]
pub(crate) async fn handle_post_users(
    State(app_state): State<AppState>,
    Json(input): Json<PostUsersInput>,
) -> Result<impl IntoResponse, StatusCode> {
    let client = reqwest::Client::new();

    let resp = client
        .post("https://identitytoolkit.googleapis.com/v1/accounts:signUp")
        .query(&[("key", &app_state.config.firebase_admin_api_key)])
        .json(&json!({
            "email": input.email,
            "password": input.password,
            "returnSecureToken": true,
        }))
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Failed to create firebase user: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .json::<FirebaseSignupResponse>()
        .await
        .map_err(|e| {
            tracing::error!("Failed to parse firebase response: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let user_token = auth::verify_id_token(&resp.id_token).await.map_err(|e| {
        tracing::error!("Failed to verify firebase token: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let created_user = repository::users::create_user(
        &app_state.pool,
        CreateUserInput {
            id: user_token.claims.sub,
            name: input.name,
            kana_name: input.kana_name,
            email: input.email,
            phone_number: input.phone_number,
            role: input.role,
            category: input.category,
        },
    )
    .await
    .map_err(|e| {
        tracing::error!("Failed to create user: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(created_user))
}
