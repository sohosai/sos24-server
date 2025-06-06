use sos24_domain::repository::{form::FormRepository, Repositories};

use crate::{
    form::{dto::FormSummaryDto, FormUseCase, FormUseCaseError},
    shared::adapter::Adapters,
    shared::context::ContextProvider,
};

impl<R: Repositories, A: Adapters> FormUseCase<R, A> {
    pub async fn list(
        &self,
        ctx: &impl ContextProvider,
    ) -> Result<Vec<FormSummaryDto>, FormUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;

        let raw_form_list = self
            .repositories
            .form_repository()
            .list()
            .await?
            .into_iter()
            .filter(|raw_form| raw_form.is_visible_to(&actor, ctx.requested_at()));

        let form_list = raw_form_list
            .into_iter()
            .map(|raw_form| FormSummaryDto::from((raw_form, None)));
        Ok(form_list.collect())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::{
        entity::user::UserRole,
        test::{fixture, repository::MockRepositories},
    };

    use crate::{form::FormUseCase, shared::adapter::MockAdapters, shared::context::TestContext};

    #[tokio::test]
    async fn 一般ユーザーは申請一覧を取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_repository_mut()
            .expect_list()
            .returning(|| Ok(vec![]));
        let adapters = MockAdapters::default();
        let use_case = FormUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case.list(&ctx).await;
        assert!(res.is_ok());
    }
}
