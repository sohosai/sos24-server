use std::sync::Arc;

use thiserror::Error;

use sos24_domain::{
    ensure,
    entity::{
        common::datetime::{DateTime, DateTimeError},
        form::{FormDescription, FormId, FormIdError, FormItemIdError, FormTitle},
        permission::{PermissionDeniedError, Permissions},
        project::ProjectIdError,
    },
    repository::{
        form::{FormRepository, FormRepositoryError},
        Repositories,
    },
};

use crate::{
    context::{Context, ContextError},
    dto::{
        form::{CreateFormDto, FormDto},
        FromEntity, ToEntity,
    },
};
use crate::{dto::form::UpdateFormDto, interactor::project::ProjectUseCaseError};

#[derive(Debug, Error)]
pub enum FormUseCaseError {
    #[error("Form not found: {0:?}")]
    NotFound(FormId),

    #[error(transparent)]
    ProjectUseCaseError(#[from] ProjectUseCaseError),
    #[error(transparent)]
    ProjectIdError(#[from] ProjectIdError),
    #[error(transparent)]
    FormIdError(#[from] FormIdError),
    #[error(transparent)]
    FormItemIdError(#[from] FormItemIdError),
    #[error(transparent)]
    DateTimeError(#[from] DateTimeError),
    #[error(transparent)]
    FormRepositoryError(#[from] FormRepositoryError),
    #[error(transparent)]
    ContextError(#[from] ContextError),
    #[error(transparent)]
    PermissionDeniedError(#[from] PermissionDeniedError),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

pub struct FormUseCase<R: Repositories> {
    repositories: Arc<R>,
}

impl<R: Repositories> FormUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }

    pub async fn list(&self, ctx: &Context) -> Result<Vec<FormDto>, FormUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_FORM_ALL));

        let raw_form_list = self.repositories.form_repository().list().await?;
        let form_list = raw_form_list.into_iter().map(FormDto::from_entity);
        Ok(form_list.collect())
    }

    pub async fn create(
        &self,
        ctx: &Context,
        raw_form: CreateFormDto,
    ) -> Result<(), FormUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::CREATE_FORM));

        let form = raw_form.into_entity()?;
        self.repositories.form_repository().create(form).await?;
        Ok(())
    }

    pub async fn find_by_id(&self, ctx: &Context, id: String) -> Result<FormDto, FormUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_FORM_ALL));

        let id = FormId::try_from(id)?;
        let form = self
            .repositories
            .form_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(FormUseCaseError::NotFound(id))?;

        // TODO: 権限チェックを行う

        Ok(FormDto::from_entity(form))
    }

    pub async fn update(
        &self,
        ctx: &Context,
        form_data: UpdateFormDto,
    ) -> Result<(), FormUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::UPDATE_FORM_ALL));

        let id = FormId::try_from(form_data.id)?;
        let form = self
            .repositories
            .form_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(FormUseCaseError::NotFound(id))?;

        let mut new_form = form.value;
        new_form.set_title(&actor, FormTitle::new(form_data.title))?;
        new_form.set_description(&actor, FormDescription::new(form_data.description))?;
        new_form.set_starts_at(&actor, DateTime::try_from(form_data.starts_at)?)?;
        new_form.set_ends_at(&actor, DateTime::try_from(form_data.ends_at)?)?;
        new_form.set_categories(&actor, form_data.categories.into_entity()?)?;
        new_form.set_attributes(&actor, form_data.attributes.into_entity()?)?;
        let new_items = form_data
            .items
            .into_iter()
            .map(|item| item.into_entity())
            .collect::<Result<_, _>>()?;
        new_form.set_items(&actor, new_items)?;

        self.repositories.form_repository().update(new_form).await?;
        Ok(())
    }

    pub async fn delete_by_id(&self, ctx: &Context, id: String) -> Result<(), FormUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::DELETE_FORM_ALL));

        let id = FormId::try_from(id)?;
        self.repositories
            .form_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(FormUseCaseError::NotFound(id.clone()))?;

        // TODO: 権限チェックを行う

        self.repositories.form_repository().delete_by_id(id).await?;
        Ok(())
    }
}
