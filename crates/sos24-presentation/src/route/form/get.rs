use std::sync::Arc;

use axum::extract::Query;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

use serde::Deserialize;
use sos24_use_case::context::Context;

use crate::error::AppError;
use crate::module::Modules;

use super::FormSummary;

#[derive(Debug, Deserialize)]
pub struct FormQuery {
    pub project_id: Option<String>,
}

pub async fn handle(
    Query(query): Query<FormQuery>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let res = match query.project_id {
        Some(project_id) => {
            modules
                .form_use_case()
                .find_by_project_id(&ctx, project_id)
                .await
        }
        None => modules.form_use_case().list(&ctx).await,
    };
    match res {
        Ok(raw_form_list) => {
            let form_list: Vec<_> = raw_form_list.into_iter().map(FormSummary::from).collect();
            Ok((StatusCode::OK, Json(form_list)))
        }
        Err(err) => {
            tracing::error!("Failed to find form by project id: {err:?}");
            Err(err.into())
        }
    }
}
