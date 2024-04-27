use sos24_domain::repository::form::FormRepository;
use sos24_domain::repository::project::ProjectRepository;
use sos24_domain::{
    ensure,
    entity::permission::Permissions,
    repository::{form_answer::FormAnswerRepository, Repositories},
};

use crate::context::ContextProvider;
use crate::form_answer::dto::FormAnswerDto;
use crate::form_answer::{FormAnswerUseCase, FormAnswerUseCaseError};
use crate::FromEntity;

impl<R: Repositories> FormAnswerUseCase<R> {
    pub async fn list(
        &self,
        ctx: &impl ContextProvider,
    ) -> Result<Vec<FormAnswerDto>, FormAnswerUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
        ensure!(actor.has_permission(Permissions::READ_FORM_ANSWER_ALL));

        let raw_form_answer_list = self.repositories.form_answer_repository().list().await?;

        let mut form_answer_list = Vec::new();
        for raw_form_answer in raw_form_answer_list {
            let project_id = raw_form_answer.value.project_id();
            let raw_project = self
                .repositories
                .project_repository()
                .find_by_id(project_id.clone())
                .await?
                .ok_or(FormAnswerUseCaseError::ProjectNotFound(project_id.clone()))?;

            let form_id = raw_form_answer.value.form_id();
            let raw_form = self
                .repositories
                .form_repository()
                .find_by_id(form_id.clone())
                .await?
                .ok_or(FormAnswerUseCaseError::FormNotFound(form_id.clone()))?;

            form_answer_list.push(FormAnswerDto::from_entity((
                raw_form_answer,
                raw_project,
                raw_form,
            )));
        }

        Ok(form_answer_list)
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
        context::TestContext,
        form_answer::{FormAnswerUseCase, FormAnswerUseCaseError},
    };

    #[tokio::test]
    async fn 一般ユーザーは回答一覧を取得できない() {
        let repositories = MockRepositories::default();
        let use_case = FormAnswerUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
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

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::Committee));
        let res = use_case.list(&ctx).await;
        assert!(res.is_ok());
    }
}
