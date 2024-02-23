use axum::{debug_handler, response::IntoResponse};
use hyper::StatusCode;

#[debug_handler]
pub async fn handle_get() -> Result<impl IntoResponse, StatusCode> {
    Ok("AUTHORISED")
}
