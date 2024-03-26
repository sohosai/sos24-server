use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::permission::Permissions,
    repository::{form_answer::FormAnswerRepository, Repositories},
};

use crate::{
    context::Context,
    dto::{form_answer::FormAnswerDto, FromEntity},
};

use super::{FormAnswerUseCase, FormAnswerUseCaseError};

impl<R: Repositories> FormAnswerUseCase<R> {
    pub async fn list(&self, ctx: &Context) -> Result<Vec<FormAnswerDto>, FormAnswerUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_FORM_ANSWER_ALL));

        let raw_form_answer_list = self.repositories.form_answer_repository().list().await?;
        let form_answer_list = raw_form_answer_list
            .into_iter()
            .map(FormAnswerDto::from_entity);
        Ok(form_answer_list.collect())
    }
}

#[cfg(test)]
mod tests {}
