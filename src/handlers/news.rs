use axum::extract::{Path, State};
use axum::{debug_handler, response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct News {
    id: Uuid,

    title: String,
    body: String,
    categories: i32,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

#[debug_handler]
pub async fn handle_get_news(
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    match sqlx::query_as::<_, News>(r#"SELECT * FROM news"#)
        .fetch_all(&app_state.pool)
        .await
    {
        Ok(news) => Ok(Json(news)),
        Err(e) => {
            tracing::error!("Failed to fetch news: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[debug_handler]
pub async fn handle_get_news_by_id(
    Path(uuid): Path<Uuid>,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    match sqlx::query_as::<_, News>(r#"SELECT * FROM news WHERE id = $1"#)
        .bind(uuid)
        .fetch_one(&app_state.pool)
        .await
    {
        Ok(news) => {
            if news.deleted_at.is_none() {
                Ok(Json(news))
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => Err(StatusCode::NOT_FOUND),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

#[debug_handler]
pub async fn handle_delete_news_by_id(
    Path(uuid): Path<Uuid>,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    match sqlx::query(r#"UPDATE news SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL"#)
        .bind(uuid)
        .execute(&app_state.pool)
        .await
    {
        Ok(res) => match res.rows_affected() {
            0 => Err(StatusCode::NOT_FOUND),
            1 => Ok(StatusCode::OK),
            affected => {
                tracing::error!("{affected} entry deleted!");
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Err(e) => {
            tracing::error!("Could not delete news: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
