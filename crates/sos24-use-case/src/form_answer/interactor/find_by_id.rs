use sos24_domain::repository::form::FormRepository;
use sos24_domain::{
    ensure,
    entity::form_answer::FormAnswerId,
    repository::{form_answer::FormAnswerRepository, project::ProjectRepository, Repositories},
};

use crate::form_answer::dto::FormAnswerDto;
use crate::form_answer::{FormAnswerUseCase, FormAnswerUseCaseError};
use crate::shared::context::ContextProvider;

impl<R: Repositories> FormAnswerUseCase<R> {
    pub async fn find_by_id(
        &self,
        ctx: &impl ContextProvider,
        id: String,
    ) -> Result<FormAnswerDto, FormAnswerUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;

        let id = FormAnswerId::try_from(id)?;
        let form_answer = self
            .repositories
            .form_answer_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(FormAnswerUseCaseError::NotFound(id))?;

        let project_id = form_answer.project_id();
        let project_with_owners = self
            .repositories
            .project_repository()
            .find_by_id(project_id.clone())
            .await?
            .ok_or(FormAnswerUseCaseError::ProjectNotFound(project_id.clone()))?;
        ensure!(project_with_owners.project.is_visible_to(&actor));

        let form_id = form_answer.form_id();
        let raw_form = self
            .repositories
            .form_repository()
            .find_by_id(form_id.clone())
            .await?
            .ok_or(FormAnswerUseCaseError::FormNotFound(form_id.clone()))?;

        Ok(FormAnswerDto::from((
            form_answer,
            project_with_owners.project,
            raw_form,
        )))
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
        form_answer::{FormAnswerUseCase, FormAnswerUseCaseError},
        shared::context::TestContext,
    };

    #[tokio::test]
    async fn 一般ユーザーは自分の企画の回答を取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_answer_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::form_answer::form_answer1(
                    fixture::project::id1(),
                )))
            });
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user1(UserRole::General),
                )))
            });
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::form::form1_opened())));
        let use_case = FormAnswerUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_id(&ctx, fixture::form_answer::id1().value().to_string())
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn 一般ユーザーは他人の企画の回答を取得できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_answer_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::form_answer::form_answer1(
                    fixture::project::id1(),
                )))
            });
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user2(UserRole::General),
                )))
            });
        let use_case = FormAnswerUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_id(&ctx, fixture::form_answer::id1().value().to_string())
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
            .form_answer_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::form_answer::form_answer1(
                    fixture::project::id1(),
                )))
            });
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user2(UserRole::General),
                )))
            });
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::form::form1_opened())));
        let use_case = FormAnswerUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeViewer));
        let res = use_case
            .find_by_id(&ctx, fixture::form_answer::id1().value().to_string())
            .await;
        assert!(res.is_ok());
    }
}
