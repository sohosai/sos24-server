use axum::{debug_handler, response::IntoResponse};

#[debug_handler]
pub async fn handle_get() -> impl IntoResponse {
    "OK"
}
