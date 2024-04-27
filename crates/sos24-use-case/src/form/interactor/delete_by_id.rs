use sos24_domain::{
    ensure,
    entity::{form::FormId, permission::Permissions},
    repository::{form::FormRepository, form_answer::FormAnswerRepository, Repositories},
};

use crate::{
    form::{FormUseCase, FormUseCaseError},
    shared::adapter::Adapters,
    shared::context::ContextProvider,
};

impl<R: Repositories, A: Adapters> FormUseCase<R, A> {
    pub async fn delete_by_id(
        &self,
        ctx: &impl ContextProvider,
        id: String,
    ) -> Result<(), FormUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
        ensure!(actor.has_permission(Permissions::DELETE_FORM_ALL));

        let id = FormId::try_from(id)?;
        self.repositories
            .form_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(FormUseCaseError::NotFound(id.clone()))?;

        let answers = self
            .repositories
            .form_answer_repository()
            .find_by_form_id(id.clone())
            .await?;
        if !answers.is_empty() {
            return Err(FormUseCaseError::HasAnswers);
        }

        self.repositories.form_repository().delete_by_id(id).await?;
        Ok(())
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
        form::{FormUseCase, FormUseCaseError},
        shared::adapter::MockAdapters,
        shared::context::TestContext,
    };

    #[tokio::test]
    async fn 実委人は申請を削除できない() {
        let repositories = MockRepositories::default();
        let adapters = MockAdapters::default();
        let use_case = FormUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .delete_by_id(&ctx, fixture::form::id1().value().to_string())
            .await;
        assert!(matches!(
            res,
            Err(FormUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            )),
        ));
    }

    #[tokio::test]
    async fn 実委人管理者は申請を削除できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::date::with(fixture::form::form1()))));
        repositories
            .form_answer_repository_mut()
            .expect_find_by_form_id()
            .returning(|_| Ok(vec![]));
        repositories
            .form_repository_mut()
            .expect_delete_by_id()
            .returning(|_| Ok(()));
        let adapters = MockAdapters::default();
        let use_case = FormUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .delete_by_id(&ctx, fixture::form::id1().value().to_string())
            .await;
        assert!(matches!(res, Ok(())));
    }

    #[tokio::test]
    async fn 回答がある申請は削除できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::date::with(fixture::form::form1()))));
        repositories
            .form_answer_repository_mut()
            .expect_find_by_form_id()
            .returning(|_| {
                Ok(vec![fixture::date::with(
                    fixture::form_answer::form_answer1(fixture::project::id1()),
                )])
            });
        let adapters = MockAdapters::default();
        let use_case = FormUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .delete_by_id(&ctx, fixture::form::id1().value().to_string())
            .await;
        assert!(matches!(res, Err(FormUseCaseError::HasAnswers)));
    }
}
