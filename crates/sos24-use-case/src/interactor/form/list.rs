use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::permission::Permissions,
    repository::{form::FormRepository, Repositories},
};

use crate::{
    context::Context,
    dto::{form::FormDto, FromEntity},
};

use super::{FormUseCase, FormUseCaseError};

impl<R: Repositories> FormUseCase<R> {
    pub async fn list(&self, ctx: &Context) -> Result<Vec<FormDto>, FormUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_FORM_ALL));

        let raw_form_list = self.repositories.form_repository().list().await?;
        let form_list = raw_form_list.into_iter().map(FormDto::from_entity);
        Ok(form_list.collect())
    }
}
