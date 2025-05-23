use sos24_domain::repository::form_answer::FormAnswerRepository;
use sos24_domain::{
    ensure,
    entity::form::FormId,
    repository::{form::FormRepository, Repositories},
};

use crate::form::dto::FormDto;
use crate::form::{FormUseCase, FormUseCaseError};
use crate::shared::adapter::Adapters;
use crate::shared::context::ContextProvider;

impl<R: Repositories, A: Adapters> FormUseCase<R, A> {
    pub async fn find_by_id(
        &self,
        ctx: &impl ContextProvider,
        id: String,
    ) -> Result<FormDto, FormUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;

        let form_id = FormId::try_from(id)?;
        let raw_form = self
            .repositories
            .form_repository()
            .find_by_id(form_id.clone())
            .await?
            .ok_or(FormUseCaseError::NotFound(form_id.clone()))?;

        ensure!(raw_form.is_visible_to(&actor, ctx.requested_at()));

        let project_with_owners = ctx.project(&*self.repositories).await?;
        let project_id = project_with_owners.map(|it| it.project.id().clone());

        let raw_form_answer = match project_id {
            Some(project_id) => {
                self.repositories
                    .form_answer_repository()
                    .find_by_project_id_and_form_id(project_id, form_id)
                    .await?
            }
            None => None,
        };

        Ok(FormDto::from((raw_form, raw_form_answer)))
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
        shared::{adapter::MockAdapters, context::TestContext},
    };

    #[tokio::test]
    async fn 一般ユーザーは公開済みの申請を取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::form::form1_opened())));
        repositories
            .project_repository_mut()
            .expect_find_by_owner_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user1(UserRole::General),
                )))
            });
        repositories
            .form_answer_repository_mut()
            .expect_find_by_project_id_and_form_id()
            .returning(|_, _| Ok(None));
        let adapters = MockAdapters::default();
        let use_case = FormUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_id(&ctx, fixture::form::id1().value().to_string())
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn 一般ユーザーは下書きの申請を取得できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::form::form1_draft())));
        repositories
            .project_repository_mut()
            .expect_find_by_owner_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user1(UserRole::General),
                )))
            });
        repositories
            .form_answer_repository_mut()
            .expect_find_by_project_id_and_form_id()
            .returning(|_, _| Ok(None));
        let adapters = MockAdapters::default();
        let use_case = FormUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_id(&ctx, fixture::form::id1().value().to_string())
            .await;
        assert!(matches!(
            res,
            Err(FormUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn 一般ユーザーは予約済みの申請を取得できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::form::form1_scheduled())));
        repositories
            .project_repository_mut()
            .expect_find_by_owner_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user1(UserRole::General),
                )))
            });
        repositories
            .form_answer_repository_mut()
            .expect_find_by_project_id_and_form_id()
            .returning(|_, _| Ok(None));
        let adapters = MockAdapters::default();
        let use_case = FormUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_id(&ctx, fixture::form::id1().value().to_string())
            .await;
        assert!(matches!(
            res,
            Err(FormUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }
}
