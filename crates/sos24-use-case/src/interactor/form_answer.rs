use std::sync::Arc;

use thiserror::Error;

use sos24_domain::{
    ensure,
    entity::{
        form::{FormId, FormIdError},
        form_answer::{FormAnswerId, FormAnswerIdError},
        permission::{PermissionDeniedError, Permissions},
        project::{ProjectId, ProjectIdError},
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
    dto::{
        form_answer::{CreateFormAnswerDto, FormAnswerDto},
        FromEntity, ToEntity,
    },
};

use super::form::FormUseCaseError;

#[derive(Debug, Error)]
pub enum FormAnswerUseCaseError {
    #[error("Form answer not found: {0:?}")]
    NotFound(FormAnswerId),
    #[error("Project not found: {0:?}")]
    ProjectNotFound(ProjectId),
    #[error("Form not found: {0:?}")]
    FormNotFound(FormId),
    #[error("Already answered")]
    AlreadyAnswered,

    #[error(transparent)]
    FormIdError(#[from] FormIdError),
    #[error(transparent)]
    ProjectIdError(#[from] ProjectIdError),
    #[error(transparent)]
    FormAnswerIdError(#[from] FormAnswerIdError),
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

    pub async fn list(&self, ctx: &Context) -> Result<Vec<FormAnswerDto>, FormAnswerUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_FORM_ANSWER_ALL));

        let raw_form_answer_list = self.repositories.form_answer_repository().list().await?;
        let form_answer_list = raw_form_answer_list
            .into_iter()
            .map(FormAnswerDto::from_entity);
        Ok(form_answer_list.collect())
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

    pub async fn find_by_id(
        &self,
        ctx: &Context,
        id: String,
    ) -> Result<FormAnswerDto, FormAnswerUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;

        let id = FormAnswerId::try_from(id)?;
        let form_answer = self
            .repositories
            .form_answer_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(FormAnswerUseCaseError::NotFound(id))?;

        // TODO: 権限チェックを行う

        Ok(FormAnswerDto::from_entity(form_answer))
    }

    pub async fn find_by_project_id(
        &self,
        ctx: &Context,
        project_id: String,
    ) -> Result<Vec<FormAnswerDto>, FormAnswerUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;

        let project_id = ProjectId::try_from(project_id)?;
        let project = self
            .repositories
            .project_repository()
            .find_by_id(project_id.clone())
            .await?
            .ok_or(FormAnswerUseCaseError::ProjectNotFound(project_id.clone()))?;
        ensure!(project.value.is_visible_to(&actor));

        let raw_form_answer_list = self
            .repositories
            .form_answer_repository()
            .find_by_project_id(project_id.clone())
            .await?;

        let form_answer_list = raw_form_answer_list
            .into_iter()
            .map(FormAnswerDto::from_entity);
        Ok(form_answer_list.collect())
    }

    pub async fn find_by_form_id(
        &self,
        ctx: &Context,
        form_id: String,
    ) -> Result<Vec<FormAnswerDto>, FormAnswerUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_FORM_ANSWER_ALL));

        let form_id = FormId::try_from(form_id)?;
        let _form = self
            .repositories
            .form_repository()
            .find_by_id(form_id.clone())
            .await?
            .ok_or(FormAnswerUseCaseError::FormNotFound(form_id.clone()))?;
        // TODO: ensure!(form.value.is_visible_to(&actor));

        let raw_form_answer_list = self
            .repositories
            .form_answer_repository()
            .find_by_form_id(form_id.clone())
            .await?;

        let form_answer_list = raw_form_answer_list
            .into_iter()
            .map(FormAnswerDto::from_entity);
        Ok(form_answer_list.collect())
    }

    pub async fn update() {}
}
