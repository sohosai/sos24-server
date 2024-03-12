use tokio::net::TcpListener;

use sos24_presentation::{config::Config, env, module, route::create_app};

#[tokio::main]
async fn main() {
    if let Err(e) = dotenvy::dotenv() {
        tracing::info!(".env file doesn't exist. skipped: {e}");
    }

    tracing_subscriber::fmt::init();

    let config = Config {
        firebase_project_id: env::firebase_project_id(),
        require_email_verification: true,
    };
    let modules = module::new(config).await.unwrap();
    let app = create_app(modules);

    let addr = format!("{}:{}", env::host(), env::port());
    let listener = TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Listening on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
