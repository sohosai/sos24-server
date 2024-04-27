use std::sync::Arc;

use axum::response::Response;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use sos24_use_case::shared::context::ContextProvider;

use crate::context::Context;
use crate::csv::serialize_to_csv;
use crate::error::AppError;
use crate::model::project::{ConvertToCreateProjectDto, CreatedProject, ProjectToBeExported};
use crate::{
    model::project::{
        ConvertToUpdateProjectDto, CreateProject, Project, ProjectSummary, UpdateProject,
    },
    module::Modules,
};

/// 企画一覧の取得
#[utoipa::path(
    get,
    path = "/projects",
    operation_id = "getProjects",
    tag = "projects",
    responses(
        (status = 200, description = "OK", body = Vec<ProjectSummary>),
        (status = 401, description = "Unauthorized", body = Error),
        (status = 403, description = "Forbidden", body = Error),
        (status = 500, description = "Internal Server Error", body = Error),
    ),
    security(("jwt_token" = [])),
)]
pub async fn handle_get(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let raw_project_list = modules.project_use_case().list(&ctx).await;
    raw_project_list
        .map(|raw_project_list| {
            let project_list: Vec<ProjectSummary> = raw_project_list
                .into_iter()
                .map(ProjectSummary::from)
                .collect();
            (StatusCode::OK, Json(project_list))
        })
        .map_err(|err| {
            tracing::error!("Failed to list project: {err:?}");
            err.into()
        })
}

/// 企画の作成
#[utoipa::path(
    post,
    path = "/projects",
    operation_id = "postProject",
    tag = "projects",
    request_body(content = CreateProject),
    responses(
        (status = 201, description = "Created", body = CreatedProject),
        (status = 400, description = "Bad Request", body = Error),
        (status = 401, description = "Unauthorized", body = Error),
        (status = 422, description = "Unprocessable Entity", body = Error),
        (status = 500, description = "Internal Server Error", body = Error),
    ),
    security(("jwt_token" = [])),
)]
pub async fn handle_post(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(raw_project): Json<CreateProject>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = ctx.user_id();
    let project = (raw_project, user_id).to_create_project_dto();
    let res = modules.project_use_case().create(&ctx, project).await;
    res.map(|id| (StatusCode::CREATED, Json(CreatedProject { id })))
        .map_err(|err| {
            tracing::error!("Failed to create project: {err:?}");
            err.into()
        })
}

/// 企画一覧のエクスポート
#[utoipa::path(
    get,
    path = "/projects/export",
    operation_id = "getProjectsExport",
    tag = "projects",
    responses(
        (status = 200, description = "OK", content_type = "text/csv", body = String),
        (status = 401, description = "Unauthorized", body = Error),
        (status = 403, description = "Forbidden", body = Error),
        (status = 500, description = "Internal Server Error", body = Error),
    ),
    security(("jwt_token" = [])),
)]
pub async fn handle_export(
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

    let mut projects: Vec<ProjectToBeExported> = Vec::new();

    for project in raw_project_list {
        let owner = match modules
            .user_use_case()
            .find_by_id(&ctx, project.owner_id.clone())
            .await
        {
            Ok(user) => user,
            Err(err) => {
                tracing::error!("Failed to find user: {err:?}");
                return Err(err.into());
            }
        };

        let sub_owner = match project.sub_owner_id.clone() {
            Some(sub_owner_id) => {
                match modules.user_use_case().find_by_id(&ctx, sub_owner_id).await {
                    Ok(user) => Some(user),
                    Err(err) => {
                        tracing::error!("Failed to find user: {err:?}");
                        return Err(err.into());
                    }
                }
            }
            None => None,
        };

        projects.push(ProjectToBeExported::from((project, owner, sub_owner)));
    }

    let data = serialize_to_csv(projects).map_err(|err| {
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

/// 自分が企画責任者・副企画責任者になっている企画の取得
#[utoipa::path(
    get,
    path = "/projects/me",
    operation_id = "getMyProject",
    tag = "projects",
    responses(
        (status = 200, description = "OK", body = Project),
        (status = 401, description = "Unauthorized", body = Error),
        (status = 404, description = "Not Found", body = Error),
        (status = 500, description = "Internal Server Error", body = Error),
    ),
    security(("jwt_token" = [])),
)]
pub async fn handle_get_me(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, AppError> {
    let raw_project = modules.project_use_case().find_owned(&ctx).await;
    match raw_project {
        Ok(Some(raw_project)) => Ok((StatusCode::OK, Json(Project::from(raw_project)))),
        Ok(None) => Err(AppError::new(
            StatusCode::NOT_FOUND,
            "project/no-project-found".to_string(),
            "Project not found".to_string(),
        )),
        Err(err) => {
            tracing::error!("Failed to find me: {err}");
            Err(err.into())
        }
    }
}

/// 特定のIDの企画の取得
#[utoipa::path(
    get,
    path = "/projects/{project_id}",
    operation_id = "getProjectById",
    tag = "projects",
    params(("project_id" = String, Path, format="uuid")),
    responses(
        (status = 200, description = "OK", body = Project),
        (status = 401, description = "Unauthorized", body = Error),
        (status = 403, description = "Forbidden", body = Error),
        (status = 404, description = "Not Found", body = Error),
        (status = 500, description = "Internal Server Error", body = Error),
    ),
    security(("jwt_token" = [])),
)]
pub async fn handle_get_id(
    Path(id): Path<String>,
    Extension(ctx): Extension<Context>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, AppError> {
    let raw_project = modules.project_use_case().find_by_id(&ctx, id).await;
    raw_project
        .map(|raw_project| (StatusCode::OK, Json(Project::from(raw_project))))
        .map_err(|err| {
            tracing::error!("Failed to find project: {err:?}");
            err.into()
        })
}

/// 特定のIDの企画の削除
#[utoipa::path(
    delete,
    path = "/projects/{project_id}",
    operation_id = "deleteProjectById",
    tag = "projects",
    params(("project_id" = String, Path, format="uuid")),
    responses(
        (status = 200, description = "OK"),
        (status = 401, description = "Unauthorized", body = Error),
        (status = 403, description = "Forbidden", body = Error),
        (status = 404, description = "Not Found", body = Error),
        (status = 500, description = "Internal Server Error", body = Error),
    ),
    security(("jwt_token" = [])),
)]
pub async fn handle_delete_id(
    Path(id): Path<String>,
    Extension(ctx): Extension<Context>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, AppError> {
    let res = modules.project_use_case().delete_by_id(&ctx, id).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to delete project: {err:?}");
        err.into()
    })
}

/// 特定のIDの企画を更新
#[utoipa::path(
    put,
    path = "/projects/{project_id}",
    operation_id = "putProjectById",
    tag = "projects",
    params(("project_id" = String, Path, format="uuid")),
    request_body(content = UpdateProject),
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = Error),
        (status = 401, description = "Unauthorized", body = Error),
        (status = 403, description = "Forbidden", body = Error),
        (status = 404, description = "Not Found", body = Error),
        (status = 422, description = "Unprocessable Entity", body = Error),
        (status = 500, description = "Internal Server Error", body = Error),
    ),
    security(("jwt_token" = [])),
)]
pub async fn handle_put_id(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(raw_project): Json<UpdateProject>,
) -> Result<impl IntoResponse, AppError> {
    let project = (raw_project, id).to_update_project_dto();
    let res = modules.project_use_case().update(&ctx, project).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to update project: {err:?}");
        err.into()
    })
}
