use axum::{debug_handler, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct PostUsersInput {}

#[debug_handler]
async fn handle_post_users(
    Json(input): Json<PostUsersInput>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(())
}
