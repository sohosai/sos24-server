use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::{form::FormId, permission::Permissions},
    repository::{form::FormRepository, Repositories},
};

use crate::{
    context::Context,
    dto::{form::FormDto, FromEntity},
};

use super::{FormUseCase, FormUseCaseError};

impl<R: Repositories> FormUseCase<R> {
    pub async fn find_by_id(&self, ctx: &Context, id: String) -> Result<FormDto, FormUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_FORM_ALL));

        let id = FormId::try_from(id)?;
        let form = self
            .repositories
            .form_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(FormUseCaseError::NotFound(id))?;

        Ok(FormDto::from_entity(form))
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
    async fn 一般ユーザーは申請を取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::date::with(fixture::form::form1()))));
        let use_case = FormUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_id(&ctx, fixture::form::id1().value().to_string())
            .await;
        assert!(res.is_ok());
    }
}
