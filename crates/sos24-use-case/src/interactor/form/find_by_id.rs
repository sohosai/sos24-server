use std::sync::Arc;

use sos24_domain::repository::form_answer::FormAnswerRepository;
use sos24_domain::{
    ensure,
    entity::{form::FormId, permission::Permissions},
    repository::{form::FormRepository, Repositories},
};

use crate::adapter::Adapters;
use crate::context::{ContextProvider, OwnedProject};
use crate::dto::{form::FormDto, FromEntity};

use super::{FormUseCase, FormUseCaseError};

impl<R: Repositories, A: Adapters> FormUseCase<R, A> {
    pub async fn find_by_id(
        &self,
        ctx: &impl ContextProvider,
        id: String,
    ) -> Result<FormDto, FormUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_FORM_ALL));

        let form_id = FormId::try_from(id)?;
        let raw_form = self
            .repositories
            .form_repository()
            .find_by_id(form_id.clone())
            .await?
            .ok_or(FormUseCaseError::NotFound(form_id.clone()))?;

        let project =
            ctx.project(Arc::clone(&self.repositories))
                .await?
                .map(|project| match project {
                    OwnedProject::Owner(project) => project,
                    OwnedProject::SubOwner(project) => project,
                });
        let project_id = project.map(|it| it.value.id().clone());

        let raw_form_answer = match project_id {
            Some(project_id) => {
                self.repositories
                    .form_answer_repository()
                    .find_by_project_id_and_form_id(project_id, form_id)
                    .await?
            }
            None => None,
        };

        Ok(FormDto::from_entity((raw_form, raw_form_answer)))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::{
        entity::user::UserRole,
        test::{fixture, repository::MockRepositories},
    };

    use crate::{adapter::MockAdapters, context::TestContext, interactor::form::FormUseCase};

    #[tokio::test]
    async fn 一般ユーザーは申請を取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::date::with(fixture::form::form1()))));
        repositories
            .project_repository_mut()
            .expect_find_by_owner_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project1(
                    fixture::user::id1(),
                ))))
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
}
