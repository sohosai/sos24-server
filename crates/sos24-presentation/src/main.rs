use tokio::net::TcpListener;

use sos24_presentation::{env, route::create_app};

#[tokio::main]
async fn main() {
    if let Err(e) = dotenvy::dotenv() {
        tracing::info!(".env file doesn't exist. skipped: {e}");
    }

    tracing_subscriber::fmt::init();

    let app = create_app();

    let addr = format!("{}:{}", env::host(), env::port());
    let listener = TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Listening on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
