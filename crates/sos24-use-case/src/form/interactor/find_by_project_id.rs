use sos24_domain::entity::project::ProjectId;
use sos24_domain::repository::form_answer::FormAnswerRepository;
use sos24_domain::repository::project::ProjectRepository;
use sos24_domain::{
    ensure,
    repository::{form::FormRepository, Repositories},
};

use crate::form::dto::FormSummaryDto;
use crate::form::{FormUseCase, FormUseCaseError};
use crate::shared::adapter::Adapters;
use crate::shared::context::ContextProvider;

impl<R: Repositories, A: Adapters> FormUseCase<R, A> {
    pub async fn find_by_project_id(
        &self,
        ctx: &impl ContextProvider,
        project_id: String,
    ) -> Result<Vec<FormSummaryDto>, FormUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;

        let project_id = ProjectId::try_from(project_id)?;
        let project_with_owners = self
            .repositories
            .project_repository()
            .find_by_id(project_id.clone())
            .await?
            .ok_or(FormUseCaseError::ProjectNotFound(project_id.clone()))?;
        ensure!(project_with_owners.project.is_visible_to(&actor));

        let forms = self.repositories.form_repository().list().await?;
        let filtered_forms = forms
            .into_iter()
            .filter(|form| form.is_sent_to(&project_with_owners.project))
            .filter(|form| form.is_visible_to(&actor, ctx.requested_at()));

        // FIXME : N+1
        let mut form_list = vec![];
        for form in filtered_forms {
            let form_id = form.id().clone();
            let form_answer = self
                .repositories
                .form_answer_repository()
                .find_by_project_id_and_form_id(project_id.clone(), form_id)
                .await?;
            form_list.push(FormSummaryDto::from((form, form_answer)));
        }
        Ok(form_list)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::entity::permission::PermissionDeniedError;
    use sos24_domain::entity::user::UserRole;
    use sos24_domain::test::fixture;
    use sos24_domain::test::repository::MockRepositories;

    use crate::form::{FormUseCase, FormUseCaseError};
    use crate::shared::adapter::MockAdapters;
    use crate::shared::context::TestContext;

    #[tokio::test]
    async fn 一般ユーザーは自分の企画を対象にした申請一覧を取得できる() {
        let mut repositories = MockRepositories::default();
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
            .expect_list()
            .returning(|| Ok(vec![]));
        let adapters = MockAdapters::default();
        let use_case = FormUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_project_id(&ctx, fixture::project::id1().value().to_string())
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn 一般ユーザーは他人の企画を対象にした申請一覧を取得できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user2(UserRole::General),
                )))
            });
        let adapters = MockAdapters::default();
        let use_case = FormUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_project_id(&ctx, fixture::project::id1().value().to_string())
            .await;
        assert!(matches!(
            res,
            Err(FormUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn 実委人は他人の企画を対象にした申請一覧を取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user1(UserRole::CommitteeViewer),
                )))
            });
        repositories
            .form_repository_mut()
            .expect_list()
            .returning(|| Ok(vec![]));
        let adapters = MockAdapters::default();
        let use_case = FormUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeViewer));
        let res = use_case
            .find_by_project_id(&ctx, fixture::project::id1().value().to_string())
            .await;
        assert!(res.is_ok());
    }
}
