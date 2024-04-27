use axum::{http::StatusCode, response::IntoResponse};

/// サーバーの状態を確認する
#[utoipa::path(
    get,
    path = "/health",
    operation_id = "getHealth",
    tag = "meta",
    responses((status = 200, description = "OK")),
    security(()),
)]
pub async fn handle_get() -> Result<impl IntoResponse, StatusCode> {
    Ok("OK")
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use sos24_domain::test::repository::MockRepositories;
    use sos24_use_case::shared::adapter::MockAdapters;
    use tower::ServiceExt;

    use crate::{module, route::create_app};

    #[tokio::test]
    async fn test_get_health() -> anyhow::Result<()> {
        let repositories = MockRepositories::default();
        let adapters = MockAdapters::default();
        let modules = module::new_test(repositories, adapters).await.unwrap();
        let app = create_app(Arc::new(modules));

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
