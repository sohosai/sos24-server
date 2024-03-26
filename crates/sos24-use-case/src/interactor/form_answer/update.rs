use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::{form::FormId, form_answer::FormAnswerId, project::ProjectId},
    repository::{form_answer::FormAnswerRepository, Repositories},
};

use crate::{
    context::Context,
    dto::{form_answer::UpdateFormAnswerDto, ToEntity},
};

use super::{FormAnswerUseCase, FormAnswerUseCaseError};

impl<R: Repositories> FormAnswerUseCase<R> {
    pub async fn update(
        &self,
        ctx: &Context,
        form_answer_data: UpdateFormAnswerDto,
    ) -> Result<(), FormAnswerUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;

        let id = FormAnswerId::try_from(form_answer_data.id)?;
        let form_answer = self
            .repositories
            .form_answer_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(FormAnswerUseCaseError::NotFound(id))?;

        ensure!(form_answer.value.is_updatable_by(&actor));

        let mut new_form_answer = form_answer.value;
        new_form_answer
            .set_project_id(&actor, ProjectId::try_from(form_answer_data.project_id)?)?;
        new_form_answer.set_form_id(&actor, FormId::try_from(form_answer_data.form_id)?)?;
        let new_items = form_answer_data
            .items
            .into_iter()
            .map(|item| item.into_entity())
            .collect::<Result<_, _>>()?;
        new_form_answer.set_items(&actor, new_items)?;

        self.repositories
            .form_answer_repository()
            .update(new_form_answer)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
