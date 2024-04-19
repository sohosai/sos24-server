use std::env;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;

pub fn host() -> String {
    env::var("HOST").unwrap_or({
        tracing::debug!(
            "The HOST environment variable is not set. Using the default value instead."
        );
        "127.0.0.1".to_string()
    })
}

pub fn port() -> String {
    env::var("PORT").unwrap_or({
        tracing::debug!(
            "The PORT environment variable is not set. Using the default value instead."
        );
        "3000".to_string()
    })
}

pub fn postgres_db_url() -> String {
    env::var("POSTGRES_DB_URL").expect("Env `POSTGRES_DB_URL` must be set")
}

pub fn firebase_service_account_key() -> String {
    let encoded_key = env::var("FIREBASE_SERVICE_ACCOUNT_KEY")
        .expect("Env `FIREBASE_SERVICE_ACCOUNT_KEY` must be set");
    let decoded_key = BASE64_STANDARD
        .decode(encoded_key.as_bytes())
        .expect("Failed to decode base64 encoded key");
    String::from_utf8(decoded_key).expect("Invalid UTF-8 encoded key")
}

pub fn firebase_project_id() -> String {
    env::var("FIREBASE_PROJECT_ID").expect("Env `FIREBASE_PROJECT_ID` must be set")
}

pub fn project_application_start_at() -> String {
    env::var("PROJECT_APPLICATION_START_AT")
        .expect("Env `PROJECT_APPLICATION_START_AT` must be set")
}

pub fn project_application_end_at() -> String {
    env::var("PROJECT_APPLICATION_END_AT").expect("Env `PROJECT_APPLICATION_END_AT` must be set")
}

pub fn require_email_verification() -> bool {
    env::var("REQUIRE_EMAIL_VERIFICATION")
        .expect("Env `REQUIRE_EMAIL_VERIFICATION` must be set")
        .parse::<bool>()
        .expect("Env `REQUIRE_EMAIL_VERIFICATION` must be a boolean")
}

pub fn mongodb_db_url() -> String {
    env::var("MONGO_DB_URL").expect("Env `MONGODB_DB_URL` must be set")
}

pub fn mongodb_db_name() -> String {
    env::var("MONGO_DB").expect("Env `MONGODB_DB_NAME` must be set")
}

pub fn s3_endpoint() -> String {
    env::var("S3_ENDPOINT").expect("Env `S3_ENDPOINT` must be set")
}

pub fn s3_region() -> String {
    env::var("S3_REGION").expect("Env `S3_REGION` must be set")
}

pub fn s3_bucket_name() -> String {
    env::var("S3_BUCKET").expect("Env `S3_BUCKET` must be set")
}

pub fn s3_access_key_id() -> String {
    env::var("S3_ACCESS_KEY_ID").expect("Env `S3_ACCESS_KEY_ID` must be set")
}

pub fn s3_secret_access_key() -> String {
    env::var("S3_SECRET_ACCESS_KEY").expect("Env `S3_SECRET_ACCESS_KEY` must be set")
}

pub fn send_grid_api_key() -> String {
    env::var("SEND_GRID_API_KEY").expect("Env `SEND_GRID_API_KEY` must be set")
}

pub fn send_grid_group_id() -> u32 {
    env::var("SEND_GRID_GROUP_ID")
        .expect("Env `SEND_GRID_GROUP_ID` must be set")
        .parse::<u32>()
        .expect("Env `SEND_GRID_GROUP_ID` must be a number")
}

pub fn email_sender_address() -> String {
    env::var("EMAIL_SENDER_ADDRESS").expect("Env `EMAIL_SENDER_ADDRESS` must be set")
}

pub fn email_reply_to_address() -> String {
    env::var("EMAIL_REPLY_TO_ADDRESS").expect("Env `EMAIL_REPLY_TO_ADDRESS` must be set")
}

pub fn app_url() -> String {
    env::var("APP_URL").expect("Env `APP_URL` must be set")
}
