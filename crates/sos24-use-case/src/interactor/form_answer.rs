use std::sync::Arc;

use thiserror::Error;

use sos24_domain::{
    ensure,
    entity::{
        permission::{PermissionDeniedError, Permissions},
        project::ProjectId,
    },
    repository::{
        form::{FormRepository, FormRepositoryError},
        form_answer::{FormAnswerRepository, FormAnswerRepositoryError},
        project::{ProjectRepository, ProjectRepositoryError},
        Repositories,
    },
    service::verify_form_answer::{self, VerifyFormAnswerError},
};

use crate::{
    context::{Context, ContextError},
    dto::{form_answer::CreateFormAnswerDto, ToEntity},
};

use super::form::FormUseCaseError;

#[derive(Debug, Error)]
pub enum FormAnswerUseCaseError {
    #[error("Project not found: {0:?}")]
    ProjectNotFound(ProjectId),
    #[error("Already answered")]
    AlreadyAnswered,

    #[error(transparent)]
    VerifyFormAnswerError(#[from] VerifyFormAnswerError),
    #[error(transparent)]
    FormRepositoryError(#[from] FormRepositoryError),
    #[error(transparent)]
    FormUseCaseError(#[from] FormUseCaseError),
    #[error(transparent)]
    FormAnswerRepositoryError(#[from] FormAnswerRepositoryError),
    #[error(transparent)]
    ProjectRepositoryError(#[from] ProjectRepositoryError),
    #[error(transparent)]
    ContextError(#[from] ContextError),
    #[error(transparent)]
    PermissionDeniedError(#[from] PermissionDeniedError),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

pub struct FormAnswerUseCase<R: Repositories> {
    repositories: Arc<R>,
}

impl<R: Repositories> FormAnswerUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }

    pub async fn create(
        &self,
        ctx: &Context,
        form_answer: CreateFormAnswerDto,
    ) -> Result<(), FormAnswerUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::CREATE_FORM_ANSWER));

        let form_answer = form_answer.into_entity()?;

        let prev_form_answer = self
            .repositories
            .form_answer_repository()
            .find_by_project_id_and_form_id(
                form_answer.project_id().clone(),
                form_answer.form_id().clone(),
            )
            .await?;
        if let Some(_) = prev_form_answer {
            return Err(FormAnswerUseCaseError::AlreadyAnswered);
        }

        let project = self
            .repositories
            .project_repository()
            .find_by_id(form_answer.project_id().clone())
            .await?
            .ok_or(FormAnswerUseCaseError::ProjectNotFound(
                form_answer.project_id().clone(),
            ))?;

        ensure!(project.value.is_visible_to(&actor));

        let form = self
            .repositories
            .form_repository()
            .find_by_id(form_answer.form_id().clone())
            .await?
            .ok_or(FormAnswerUseCaseError::ProjectNotFound(
                form_answer.project_id().clone(),
            ))?;

        // TODO: 申請がその企画向けのものかどうかのチェック

        verify_form_answer::verify(&form.value, &form_answer)?;

        self.repositories
            .form_answer_repository()
            .create(form_answer)
            .await?;

        Ok(())
    }

    pub async fn find_by_id() {}

    pub async fn find_by_project_id() {}

    pub async fn find_by_form_id() {}

    pub async fn update() {}
}
