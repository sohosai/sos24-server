use std::sync::Arc;

use sos24_domain::repository::form::FormRepository;
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

        let mut form_answer_list = Vec::new();
        for raw_form_answer in raw_form_answer_list {
            let form_id = raw_form_answer.value.form_id();
            let raw_form = self
                .repositories
                .form_repository()
                .find_by_id(form_id.clone())
                .await?
                .ok_or(FormAnswerUseCaseError::FormNotFound(form_id.clone()))?;
            form_answer_list.push(FormAnswerDto::from_entity((
                raw_form_answer,
                project.clone(),
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
        context::Context,
        interactor::form_answer::{FormAnswerUseCase, FormAnswerUseCaseError},
    };

    #[tokio::test]
    async fn 一般ユーザーは自分の企画の回答一覧を取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project1(
                    fixture::user::id1(),
                ))))
            });
        repositories
            .form_answer_repository_mut()
            .expect_find_by_project_id()
            .returning(|_| Ok(vec![]));
        let use_case = FormAnswerUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_project_id(&ctx, fixture::project::id1().value().to_string())
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn 一般ユーザーは他人の企画の回答を取得できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project1(
                    fixture::user::id2(),
                ))))
            });
        repositories
            .form_answer_repository_mut()
            .expect_find_by_project_id()
            .returning(|_| Ok(vec![]));
        let use_case = FormAnswerUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_project_id(&ctx, fixture::project::id1().value().to_string())
            .await;
        assert!(matches!(
            res,
            Err(FormAnswerUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn 実委人は他人の企画の回答を取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project1(
                    fixture::user::id2(),
                ))))
            });
        repositories
            .form_answer_repository_mut()
            .expect_find_by_project_id()
            .returning(|_| Ok(vec![]));
        let use_case = FormAnswerUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .find_by_project_id(&ctx, fixture::project::id1().value().to_string())
            .await;
        assert!(res.is_ok());
    }
}
