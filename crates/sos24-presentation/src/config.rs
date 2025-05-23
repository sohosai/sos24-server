#[derive(Default, Clone)]
pub struct Config {
    pub firebase_project_id: String,
    pub require_email_verification: bool,
    pub project_application_start_at: String,
    pub project_application_end_at: String,
    pub s3_bucket_name: String,
    pub file_upload_limit: usize,

    pub email_sender_address: String,
    pub email_reply_to_address: String,
    pub app_url: String,

    pub default_admin_email: String,
    pub default_admin_password: String,
}
