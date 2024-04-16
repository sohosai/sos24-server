use std::sync::Arc;

use axum::{
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
    Extension,
};
use chrono_tz::Asia::Tokyo;
use serde::Serialize;
use sos24_use_case::{context::Context, user::dto::UserDto};

use crate::{error::AppError, module::Modules, route::shared::csv::serialize_to_csv};

use super::UserRole;

#[derive(Debug, Serialize)]
pub struct UserToBeExported {
    #[serde(rename(serialize = "ID"))]
    id: String,
    #[serde(rename(serialize = "名前"))]
    name: String,
    #[serde(rename(serialize = "なまえ"))]
    kana_name: String,
    #[serde(rename(serialize = "メールアドレス"))]
    email: String,
    #[serde(rename(serialize = "権限"))]
    role: String,
    #[serde(rename(serialize = "作成日時"))]
    created_at: String,
}

impl From<UserDto> for UserToBeExported {
    fn from(user: UserDto) -> Self {
        UserToBeExported {
            id: user.id,
            name: user.name,
            kana_name: user.kana_name,
            email: user.email,
            role: UserRole::from(user.role).to_string(),
            created_at: user.created_at.with_timezone(&Tokyo).to_rfc3339(),
        }
    }
}

pub async fn handle(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let raw_user_list = modules.user_use_case().list(&ctx).await;
    let user_list = match raw_user_list {
        Ok(user_list) => user_list
            .into_iter()
            .map(UserToBeExported::from)
            .collect::<Vec<UserToBeExported>>(),
        Err(err) => {
            tracing::error!("Failed to list user: {err:?}");
            return Err(err.into());
        }
    };

    let data = serialize_to_csv(user_list).map_err(|err| {
        tracing::error!("Failed to serialize to csv: {err:?}");
        AppError::from(err)
    })?;

    Response::builder()
        .header("Content-Type", "text/csv")
        .header("Content-Disposition", "attachment; filename=users.csv")
        .body(data)
        .map_err(|err| {
            tracing::error!("Failed to create response: {err:?}");
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "csv/failed-to-convert".to_string(),
                format!("{err:?}"),
            )
        })
}
