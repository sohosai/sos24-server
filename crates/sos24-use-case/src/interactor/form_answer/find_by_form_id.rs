use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::{form::FormId, permission::Permissions},
    repository::{form::FormRepository, form_answer::FormAnswerRepository, Repositories},
};

use crate::{
    context::Context,
    dto::{form_answer::FormAnswerDto, FromEntity},
};

use super::{FormAnswerUseCase, FormAnswerUseCaseError};

impl<R: Repositories> FormAnswerUseCase<R> {
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
}

#[cfg(test)]
mod tests {}
