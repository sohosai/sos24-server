use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::{
        common::datetime::DateTimeError,
        permission::{PermissionDeniedError, Permissions},
    },
    repository::{
        form::{FormRepository, FormRepositoryError},
        Repositories,
    },
};
use thiserror::Error;

use crate::{
    context::{Context, ContextError},
    dto::{
        form::{CreateFormDto, FormDto},
        FromEntity, ToEntity,
    },
};

#[derive(Debug, Error)]
pub enum FormUseCaseError {
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
    repositors: Arc<R>,
}

impl<R: Repositories> FormUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self {
            repositors: repositories,
        }
    }

    pub async fn list(&self, ctx: &Context) -> Result<Vec<FormDto>, FormUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositors)).await?;
        ensure!(actor.has_permission(Permissions::READ_FORM_ALL));

        let raw_form_list = self.repositors.form_repository().list().await?;
        let form_list = raw_form_list.into_iter().map(FormDto::from_entity);
        Ok(form_list.collect())
    }

    pub async fn create(
        &self,
        ctx: &Context,
        raw_form: CreateFormDto,
    ) -> Result<(), FormUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositors)).await?;
        ensure!(actor.has_permission(Permissions::CREATE_FORM));

        let form = raw_form.into_entity()?;
        self.repositors.form_repository().create(form).await?;
        Ok(())
    }
}
