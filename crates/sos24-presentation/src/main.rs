use std::sync::Arc;

use tokio::net::TcpListener;

use sos24_presentation::{config::Config, env, module, route::create_app};
use tokio_cron_scheduler::{Job, JobScheduler};

#[tokio::main]
async fn main() {
    if let Err(e) = dotenvy::dotenv() {
        tracing::info!(".env file doesn't exist. skipped: {e}");
    }

    tracing_subscriber::fmt::init();

    tracing::info!("Initializing server");

    let config = Config {
        firebase_project_id: env::firebase_project_id(),
        require_email_verification: env::require_email_verification(),
        project_application_start_at: env::project_application_start_at(),
        project_application_end_at: env::project_application_end_at(),
        s3_bucket_name: env::s3_bucket_name(),
        // 1GB
        file_upload_limit: 1e+9 as usize,
    };
    let modules = Arc::new(module::new(config).await.unwrap());
    let app = create_app(Arc::clone(&modules));

    let sched = JobScheduler::new()
        .await
        .expect("Failed to create job scheduler");
    let job = Job::new_async("1/10 * * * * *", move |_, _| {
        let modules = Arc::clone(&modules);
        Box::pin(async move {
            tracing::info!("cronjobを実行します");
            modules
                .form_use_case()
                .check_form_and_send_notify(chrono::Utc::now())
                .await
                .expect("Failed to check form and send notify");
            tracing::info!("cronjobを実行しました");
        })
    })
    .expect("Failed to create job");
    sched.add(job).await.expect("Failed to add job");
    sched.start().await.expect("Failed to start job scheduler");

    tracing::info!("Server initialized");

    let addr = format!("{}:{}", env::host(), env::port());
    let listener = TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Listening on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
