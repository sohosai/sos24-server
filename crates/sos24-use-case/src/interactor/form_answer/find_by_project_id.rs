use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::project::ProjectId,
    repository::{form_answer::FormAnswerRepository, project::ProjectRepository, Repositories},
};

use crate::{
    context::Context,
    dto::{form_answer::FormAnswerDto, FromEntity},
};

use super::{FormAnswerUseCase, FormAnswerUseCaseError};

impl<R: Repositories> FormAnswerUseCase<R> {
    pub async fn find_by_project_id(
        &self,
        ctx: &Context,
        project_id: String,
    ) -> Result<Vec<FormAnswerDto>, FormAnswerUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;

        let project_id = ProjectId::try_from(project_id)?;
        let project = self
            .repositories
            .project_repository()
            .find_by_id(project_id.clone())
            .await?
            .ok_or(FormAnswerUseCaseError::ProjectNotFound(project_id.clone()))?;
        ensure!(project.value.is_visible_to(&actor));

        let raw_form_answer_list = self
            .repositories
            .form_answer_repository()
            .find_by_project_id(project_id.clone())
            .await?;

        let form_answer_list = raw_form_answer_list
            .into_iter()
            .map(FormAnswerDto::from_entity);
        Ok(form_answer_list.collect())
    }
}

#[cfg(test)]
mod tests {
    // TODO: 一般ユーザーは自分の企画の回答を取得できる
    // TODO: 一般ユーザーは他人の企画の回答を取得できない
    // TODO: 実委人は他人の企画の回答を取得できる
}
