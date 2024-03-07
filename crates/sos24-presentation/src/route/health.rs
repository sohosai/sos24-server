use axum::response::IntoResponse;

pub async fn handle_get() -> impl IntoResponse {
    "OK"
}

#[cfg(test)]
mod test {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use sos24_domain::test::MockRepositories;
    use tower::ServiceExt;

    use crate::{module, route::create_app};

    #[tokio::test]
    async fn test_get_health() -> anyhow::Result<()> {
        let repositories = MockRepositories::default();
        let modules = module::new_test(repositories).await.unwrap();
        let app = create_app(modules);

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
