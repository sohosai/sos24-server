use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

use crate::module::Modules;
use sos24_domain::repository::{HealthChecker, Repositories};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub postgresql: DatabaseStatus,
    pub mongodb: DatabaseStatus,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct DatabaseStatus {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// サーバーとデータベースの状態を確認する
#[utoipa::path(
    get,
    path = "/health",
    operation_id = "getHealth",
    tag = "meta",
    responses(
        (status = 200, description = "OK", body = HealthResponse),
        (status = 503, description = "Service Unavailable", body = HealthResponse)
    ),
    security(()),
)]
pub async fn handle_get(
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut overall_healthy = true;
    
    // Check PostgreSQL
    let postgresql_status = match modules.repositories().health_checker().check_postgresql().await {
        Ok(_) => DatabaseStatus {
            status: "healthy".to_string(),
            error: None,
        },
        Err(e) => {
            overall_healthy = false;
            DatabaseStatus {
                status: "unhealthy".to_string(),
                error: Some(e.to_string()),
            }
        }
    };

    // Check MongoDB
    let mongodb_status = match modules.repositories().health_checker().check_mongodb().await {
        Ok(_) => DatabaseStatus {
            status: "healthy".to_string(),
            error: None,
        },
        Err(e) => {
            overall_healthy = false;
            DatabaseStatus {
                status: "unhealthy".to_string(),
                error: Some(e.to_string()),
            }
        }
    };

    let response = HealthResponse {
        status: if overall_healthy { "healthy" } else { "unhealthy" }.to_string(),
        postgresql: postgresql_status,
        mongodb: mongodb_status,
    };

    let status_code = if overall_healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    Ok((status_code, Json(response)))
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
        let mut repositories = MockRepositories::default();
        
        // Setup mock expectations for health checks to return success
        repositories.health_checker_mut()
            .expect_check_postgresql()
            .returning(|| Ok(()));
        repositories.health_checker_mut()
            .expect_check_mongodb()
            .returning(|| Ok(()));
            
        let adapters = MockAdapters::default();
        let modules = module::new_test(repositories, adapters).await.unwrap();
        let app = create_app(Arc::new(modules));

        let resp = app
            .oneshot(Request::builder().uri("/health").body(Body::empty())?)
            .await?;

        assert_eq!(resp.status(), StatusCode::OK);
        
        // Check that response is JSON with expected structure
        let body_bytes = axum::body::to_bytes(resp.into_body(), usize::MAX).await?;
        let response: serde_json::Value = serde_json::from_slice(&body_bytes)?;
        
        assert_eq!(response["status"], "healthy");
        assert_eq!(response["postgresql"]["status"], "healthy");
        assert_eq!(response["mongodb"]["status"], "healthy");

        Ok(())
    }
}
