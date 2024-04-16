use std::sync::Arc;

use axum::response::Response;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension};

use serde::Serialize;
use sos24_use_case::context::Context;
use sos24_use_case::project::dto::ProjectWithOwnersDto;

use crate::error::AppError;
use crate::module::Modules;
use crate::route::shared::csv::serialize_to_csv;

use super::{ProjectAttributes, ProjectCategory};

#[derive(Debug, Serialize)]
pub struct ProjectToBeExported {
    #[serde(rename(serialize = "企画番号"))]
    index: i32,
    #[serde(rename(serialize = "企画名"))]
    title: String,
    #[serde(rename(serialize = "きかくめい"))]
    kana_title: String,
    #[serde(rename(serialize = "企画団体名"))]
    group_name: String,
    #[serde(rename(serialize = "企画責任者"))]
    owner_name: String,
    #[serde(rename(serialize = "企画責任者メールアドレス"))]
    owner_email: String,
    #[serde(rename(serialize = "企画責任者電話番号"))]
    owner_phone_number: String,
    #[serde(rename(serialize = "副企画責任者"))]
    sub_owner_name: Option<String>,
    #[serde(rename(serialize = "副企画責任者メールアドレス"))]
    sub_owner_email: Option<String>,
    #[serde(rename(serialize = "副企画責任者電話番号"))]
    sub_owner_phone_number: Option<String>,
    #[serde(rename(serialize = "企画区分"))]
    category: String,
    #[serde(rename(serialize = "企画属性"))]
    attributes: String,
    #[serde(rename(serialize = "備考"))]
    remarks: Option<String>,
    #[serde(rename(serialize = "作成日時"))]
    created_at: String,
}

impl From<ProjectWithOwnersDto> for ProjectToBeExported {
    fn from(dto: ProjectWithOwnersDto) -> Self {
        let (sub_owner_name, sub_owner_email, sub_owner_phone_number) =
            dto.sub_owner.map_or((None, None, None), |it| {
                (Some(it.name), Some(it.email), Some(it.phone_number))
            });
        ProjectToBeExported {
            index: dto.project.index,
            title: dto.project.title,
            kana_title: dto.project.kana_title,
            group_name: dto.project.group_name,
            owner_name: dto.owner.name,
            owner_email: dto.owner.email,
            owner_phone_number: dto.owner.phone_number,
            sub_owner_name,
            sub_owner_email,
            sub_owner_phone_number,
            category: ProjectCategory::from(dto.project.category).to_string(),
            attributes: ProjectAttributes::from(dto.project.attributes).to_string(),
            remarks: dto.project.remarks,
            created_at: dto.project.created_at.to_rfc3339(),
        }
    }
}

pub async fn handle(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let raw_project_list = match modules.project_use_case().list(&ctx).await {
        Ok(list) => list,
        Err(err) => {
            tracing::error!("Failed to list project: {err:?}");
            return Err(err.into());
        }
    };

    let project_list = raw_project_list
        .into_iter()
        .map(ProjectToBeExported::from)
        .collect();
    let data = serialize_to_csv(project_list).map_err(|err| {
        tracing::error!("Failed to serialize to csv: {err:?}");
        AppError::from(err)
    })?;

    Response::builder()
        .header("Content-Type", "text/csv")
        .header("Content-Disposition", "attachment; filename=projects.csv")
        .body(data)
        .map_err(|err| {
            tracing::error!("Failed to create response: {err:?}");
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "csv/failed-to-create-response".to_string(),
                format!("{err:?}"),
            )
        })
}
