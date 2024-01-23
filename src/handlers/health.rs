use axum::routing::{get, MethodRouter};

pub fn handle_get() -> MethodRouter {
    get(|| async { "OK" })
}

#[cfg(test)]
mod test {
    use crate::handlers::create_app;

    use anyhow::Result;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_health() -> Result<()> {
        let app = create_app();
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
