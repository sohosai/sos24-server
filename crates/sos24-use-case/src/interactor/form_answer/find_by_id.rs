use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::form_answer::FormAnswerId,
    repository::{form_answer::FormAnswerRepository, project::ProjectRepository, Repositories},
};

use crate::{
    context::Context,
    dto::{form_answer::FormAnswerDto, FromEntity},
};

use super::{FormAnswerUseCase, FormAnswerUseCaseError};

impl<R: Repositories> FormAnswerUseCase<R> {
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

        let project_id = form_answer.value.project_id();
        let project = self
            .repositories
            .project_repository()
            .find_by_id(project_id.clone())
            .await?
            .ok_or(FormAnswerUseCaseError::ProjectNotFound(project_id.clone()))?;
        ensure!(project.value.is_visible_to(&actor));

        Ok(FormAnswerDto::from_entity(form_answer))
    }
}

#[cfg(test)]
mod tests {
    // TODO: 一般ユーザーは自分の企画の回答を取得できる
    // TODO: 一般ユーザーは他人の企画の回答を取得できない
    // TODO: 実委人は他人の企画の回答を取得できる
}
