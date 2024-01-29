use axum::{debug_handler, response::IntoResponse};
use hyper::StatusCode;

#[debug_handler]
pub async fn handle_get() -> Result<impl IntoResponse, StatusCode> {
    Ok("OK")
}

#[cfg(test)]
mod test {
    use crate::{create_app, repository::get_mongo_db};

    use anyhow::Result;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use sqlx::PgPool;
    use tower::ServiceExt;

    #[sqlx::test]
    async fn test_get_health(pool: PgPool) -> Result<()> {
        let mongo_db = get_mongo_db().await?;

        let app = create_app(pool, mongo_db);
        let resp = app
            .oneshot(Request::builder().uri("/health").body(Body::empty())?)
            .await?;

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            axum::body::to_bytes(resp.into_body(), usize::MAX).await?,
            b"OK".as_ref()
        );

        Ok(())
    }
}
