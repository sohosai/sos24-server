use std::sync::Arc;

use anyhow::Context as _;
use axum::{
    body::Body,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension,
};

use csv::Writer;
use percent_encoding::NON_ALPHANUMERIC;
use serde::Deserialize;
use sos24_use_case::context::Context;

use crate::{error::AppError, module::Modules, route::shared::csv::CsvSerializationError};

#[derive(Debug, Deserialize)]
pub struct ExportFormAnswerQuery {
    pub form_id: Option<String>,
}

pub async fn handle(
    State(modules): State<Arc<Modules>>,
    Query(query): Query<ExportFormAnswerQuery>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let Some(form_id) = query.form_id else {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "form-answer/invalid-query".to_string(),
            "Invalid query".to_string(),
        ));
    };

    let form_answer_list = modules
        .form_answer_use_case()
        .export_by_form_id(&ctx, form_id)
        .await
        .map_err(|err| {
            tracing::error!("Failed to export form answer: {err:?}");
            AppError::from(err)
        })?;

    let data = (|| -> Result<Vec<u8>, CsvSerializationError> {
        let mut wrt = Writer::from_writer(vec![]);

        let header = ["企画番号", "企画名", "企画団体名", "回答日時"]
            .into_iter()
            .chain(form_answer_list.form_item_names.iter().map(String::as_str));
        wrt.write_record(header).context("Failed to write header")?;

        for form_answer in form_answer_list.form_answers {
            let record = [
                form_answer.project_index.to_string(),
                form_answer.project_title,
                form_answer.project_group_name,
                form_answer.created_at,
            ]
            .into_iter()
            .chain(
                form_answer
                    .form_answer_item_values
                    .into_iter()
                    .map(|it| it.unwrap_or_default()),
            );
            wrt.write_record(record).context("Failed to write record")?;
        }

        let csv = wrt.into_inner().context("Failed to write csv")?;
        Ok(csv)
    })()
    .map_err(|err| {
        tracing::error!("Failed to serialize to csv: {err:?}");
        AppError::from(err)
    })?;

    let filename = format!("{}_回答一覧.csv", form_answer_list.form_title);
    let encoded_filename = percent_encoding::percent_encode(filename.as_bytes(), NON_ALPHANUMERIC);
    Response::builder()
        .header("Content-Type", "text/csv")
        .header(
            "Content-Disposition",
            format!(
                "attachment; filename=\"{}\" filename*=UTF-8''{}",
                filename, encoded_filename
            ),
        )
        .body(Body::from(data))
        .map_err(|err| {
            tracing::error!("Failed to create response: {err:?}");
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "csv/failed-to-create-response".to_string(),
                format!("{err:?}"),
            )
        })
}
