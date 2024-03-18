use std::sync::Arc;

use axum::response::Response;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use csv::Writer;
use sos24_use_case::context::Context;

use crate::model::project::ProjectToBeExport;
use crate::{
    model::project::{
        ConvertToCreateProjectDto, ConvertToUpdateProjectDto, CreateProject, Project, UpdateProject,
    },
    module::Modules,
    status_code::ToStatusCode,
};

pub async fn handle_get(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, StatusCode> {
    let raw_project_list = modules.project_use_case().list(&ctx).await;
    raw_project_list
        .map(|raw_project_list| {
            let project_list: Vec<Project> =
                raw_project_list.into_iter().map(Project::from).collect();
            (StatusCode::OK, Json(project_list))
        })
        .map_err(|err| {
            tracing::error!("Failed to list project: {err:?}");
            err.status_code()
        })
}

pub async fn handle_post(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(raw_project): Json<CreateProject>,
) -> Result<impl IntoResponse, StatusCode> {
    let user_id = ctx.user_id().clone().value();
    let project = (raw_project, user_id).to_create_project_dto();
    let res = modules.project_use_case().create(&ctx, project).await;
    res.map(|_| StatusCode::CREATED).map_err(|err| {
        tracing::error!("Failed to create project: {err:?}");
        err.status_code()
    })
}

pub async fn handle_export(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, StatusCode> {
    let raw_project_list = match modules.project_use_case().list(&ctx).await {
        Ok(list) => list,
        Err(err) => {
            tracing::error!("Failed to list project: {err:?}");
            return Err(err.status_code());
        }
    };

    let mut project_with_user: Vec<ProjectToBeExport> = Vec::new();

    for project in raw_project_list {
        let owner = match modules
            .user_use_case()
            .find_by_id(&ctx, project.clone().owner_id)
            .await
        {
            Ok(user) => user,
            Err(err) => {
                tracing::error!("Failed to find user: {err:?}");
                return Err(err.status_code());
            }
        };

        let sub_owner = match project.clone().sub_owner_id {
            Some(sub_owner_id) => match modules
                .user_use_case()
                .find_by_id(&ctx, sub_owner_id)
                .await
            {
                Ok(user) => Some(user),
                Err(err) => {
                    tracing::error!("Failed to find user: {err:?}");
                    return Err(err.status_code());
                }
            },
            None => None,
        };

        project_with_user.push(ProjectToBeExport::from((project, owner, sub_owner)));
    }

    let mut wrt = Writer::from_writer(vec![]);
    for user_with_project in project_with_user {
        match wrt.serialize(user_with_project) {
            Ok(result) => result,
            Err(err) => {
                tracing::error!("Failed to serialize: {err:?}");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
    }

    let csv = match wrt.into_inner() {
        Ok(csv) => csv,
        Err(err) => {
            tracing::error!("Failed to write csv: {err:?}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let data = match String::from_utf8(csv) {
        Ok(data) => data,
        Err(err) => {
            tracing::error!("Failed to convert csv to string: {err:?}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Response::builder()
        .header("Content-Type", "text/csv")
        .header("Content-Disposition", "attachment; filename=projects.csv")
        .body(data)
        .map(|response| response)
        .map_err(|err| {
            tracing::error!("Failed to create response: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn handle_get_me(
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
) -> Result<impl IntoResponse, StatusCode> {
    let raw_project = modules.project_use_case().find_owned(&ctx).await;
    match raw_project {
        Ok(Some(raw_project)) => Ok((StatusCode::OK, Json(Project::from(raw_project)))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(err) => {
            tracing::error!("Failed to find me: {err}");
            Err(err.status_code())
        }
    }
}

pub async fn handle_get_id(
    Path(id): Path<String>,
    Extension(ctx): Extension<Context>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let raw_project = modules.project_use_case().find_by_id(&ctx, id).await;
    match raw_project {
        Ok(raw_project) => Ok((StatusCode::OK, Json(Project::from(raw_project)))),
        Err(err) => {
            tracing::error!("Failed to find project: {err:?}");
            Err(err.status_code())
        }
    }
}

pub async fn handle_delete_id(
    Path(id): Path<String>,
    Extension(ctx): Extension<Context>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.project_use_case().delete_by_id(&ctx, id).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to delete project: {err:?}");
        err.status_code()
    })
}

pub async fn handle_put_id(
    Path(id): Path<String>,
    State(modules): State<Arc<Modules>>,
    Extension(ctx): Extension<Context>,
    Json(raw_project): Json<UpdateProject>,
) -> Result<impl IntoResponse, StatusCode> {
    let project = (raw_project, id).to_update_project_dto();
    let res = modules.project_use_case().update(&ctx, project).await;
    res.map(|_| StatusCode::OK).map_err(|err| {
        tracing::error!("Failed to update project: {err:?}");
        err.status_code()
    })
}
