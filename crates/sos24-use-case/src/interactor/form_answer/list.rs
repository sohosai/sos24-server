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
mod tests {
    use std::sync::Arc;

    use sos24_domain::{
        entity::{permission::PermissionDeniedError, user::UserRole},
        test::{fixture, repository::MockRepositories},
    };

    use crate::{
        context::Context,
        interactor::form_answer::{FormAnswerUseCase, FormAnswerUseCaseError},
    };

    #[tokio::test]
    async fn 一般ユーザーは回答一覧を取得できない() {
        let repositories = MockRepositories::default();
        let use_case = FormAnswerUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case.list(&ctx).await;
        assert!(matches!(
            res,
            Err(FormAnswerUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn 実委人は回答一覧を取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_answer_repository_mut()
            .expect_list()
            .returning(|| Ok(vec![]));
        let use_case = FormAnswerUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case.list(&ctx).await;
        assert!(res.is_ok());
    }
}
