use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::permission::Permissions,
    repository::{form::FormRepository, Repositories},
};

use crate::{
    context::Context,
    dto::{form::CreateFormDto, ToEntity},
};

use super::{FormUseCase, FormUseCaseError};

impl<R: Repositories> FormUseCase<R> {
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
}
