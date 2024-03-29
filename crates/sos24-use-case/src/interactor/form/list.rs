use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::permission::Permissions,
    repository::{form::FormRepository, Repositories},
};

use crate::{
    context::Context,
    dto::{form::FormDto, FromEntity},
};

use super::{FormUseCase, FormUseCaseError};

impl<R: Repositories> FormUseCase<R> {
    pub async fn list(&self, ctx: &Context) -> Result<Vec<FormDto>, FormUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_FORM_ALL));

        let raw_form_list = self.repositories.form_repository().list().await?;
        let form_list = raw_form_list.into_iter().map(FormDto::from_entity);
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

    use crate::{context::Context, interactor::form::FormUseCase};

    #[tokio::test]
    async fn 一般ユーザーは申請一覧を取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_repository_mut()
            .expect_list()
            .returning(|| Ok(vec![]));
        let use_case = FormUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case.list(&ctx).await;
        assert!(matches!(res, Ok(_)));
    }
}
