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
        let owned_project_id = ctx
            .project(Arc::clone(&self.repositories))
            .await?
            .map(|project| match project {
                OwnedProject::Owner(project) => project.value.id().clone(),
                OwnedProject::SubOwner(project) => project.value.id().clone(),
            });

        let id = FormAnswerId::try_from(form_answer_data.id)?;
        let form_answer = self
            .repositories
            .form_answer_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(FormAnswerUseCaseError::NotFound(id))?;

        ensure!(form_answer
            .value
            .is_updatable_by(&actor, owned_project_id.clone()));

        let mut new_form_answer = form_answer.value;
        let new_items = form_answer_data
            .items
            .into_iter()
            .map(|item| item.into_entity())
            .collect::<Result<_, _>>()?;
        new_form_answer.set_items(&actor, owned_project_id, new_items)?;

        let form_id = new_form_answer.form_id().clone();
        let form = self
            .repositories
            .form_repository()
            .find_by_id(form_id.clone())
            .await?
            .ok_or(FormAnswerUseCaseError::FormNotFound(form_id))?;

        verify_form_answer::verify(&form.value, &new_form_answer)?;

        self.repositories
            .form_answer_repository()
            .update(new_form_answer)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // TODO: 一般ユーザーは自分の企画の回答を更新できる
    // TODO: 一般ユーザーは他人の企画の回答を更新できない
    // TODO: 実委人は他人の企画の回答を更新できる
}
