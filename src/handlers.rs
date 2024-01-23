use axum::Router;

mod health;

pub fn create_app() -> Router {
    Router::new().route("/health", health::handle_get())
}
