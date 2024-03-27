use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::permission::Permissions,
    repository::{
        form::FormRepository, form_answer::FormAnswerRepository, project::ProjectRepository,
        Repositories,
    },
    service::verify_form_answer,
};

use crate::{
    context::Context,
    dto::{form_answer::CreateFormAnswerDto, ToEntity},
};

use super::{FormAnswerUseCase, FormAnswerUseCaseError};

impl<R: Repositories> FormAnswerUseCase<R> {
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

        // TODO: 申請がその企画向けのものかどうかのチェックするとよいかもしれない

        verify_form_answer::verify(&form.value, &form_answer)?;

        self.repositories
            .form_answer_repository()
            .create(form_answer)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // TODO: 一般ユーザーは自分の企画の回答を作成できる
    // TODO: 企画が見つからない場合はエラーになる
    // TODO: 申請が見つからない場合はエラーになる
    // TODO: すでに回答がある場合はエラーになる
}
