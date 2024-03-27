use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::{
        common::datetime::DateTime,
        form::{FormDescription, FormId, FormTitle},
        permission::Permissions,
    },
    repository::{form::FormRepository, Repositories},
};

use crate::{
    context::Context,
    dto::{form::UpdateFormDto, ToEntity},
};

use super::{FormUseCase, FormUseCaseError};

impl<R: Repositories> FormUseCase<R> {
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
}

#[cfg(test)]
mod tests {
    // TODO: 実委人は申請を更新できない
    // TODO: 実委人管理者は申請を更新できる
}
