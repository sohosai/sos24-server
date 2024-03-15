use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use sos24_domain::entity::actor::Actor;

use crate::{
    model::project::{ConvertToCreateProjectDto, CreateProject, Project},
    module::Modules,
    status_code::ToStatusCode,
};

pub async fn handle_get(
    State(modules): State<Arc<Modules>>,
    Extension(actor): Extension<Actor>,
) -> Result<impl IntoResponse, StatusCode> {
    let raw_project_list = modules.project_use_case().list(&actor).await;
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
    Extension(actor): Extension<Actor>,
    Json(raw_project): Json<CreateProject>,
) -> Result<impl IntoResponse, StatusCode> {
    let user_id = actor.user_id().clone().value();
    let project = (raw_project, user_id).to_create_project_dto();
    let res = modules.project_use_case().create(&actor, project).await;
    res.map(|_| StatusCode::CREATED).map_err(|err| {
        tracing::error!("Failed to create project: {err:?}");
        err.status_code()
    })
}

pub async fn handle_get_id(
    Path(id): Path<String>,
    Extension(actor): Extension<Actor>,
    State(modules): State<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let raw_project = modules.project_use_case().find_by_id(&actor, id).await;
    match raw_project {
        Ok(raw_project) => Ok((StatusCode::OK, Json(Project::from(raw_project)))),
        Err(err) => {
            tracing::error!("Failed to find project: {err:?}");
            Err(err.status_code())
        }
    }
}
