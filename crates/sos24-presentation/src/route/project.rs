use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use sos24_domain::entity::actor::Actor;

use crate::{
    model::project::{ConvertToCreateProjectDto, CreateProject},
    module::Modules,
    status_code::ToStatusCode,
};

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
