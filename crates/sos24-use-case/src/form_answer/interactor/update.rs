use sos24_domain::{
    ensure,
    entity::form_answer::{FormAnswerId, FormAnswerItem},
    repository::{form::FormRepository, form_answer::FormAnswerRepository, Repositories},
    service::verify_form_answer,
};

use crate::{
    form_answer::{dto::FormAnswerItemDto, FormAnswerUseCase, FormAnswerUseCaseError},
    shared::context::ContextProvider,
};

pub struct UpdateFormAnswerCommand {
    pub id: String,
    pub items: Vec<FormAnswerItemDto>,
}

impl<R: Repositories> FormAnswerUseCase<R> {
    pub async fn update(
        &self,
        ctx: &impl ContextProvider,
        form_answer_data: UpdateFormAnswerCommand,
    ) -> Result<(), FormAnswerUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;

        let project_with_owners = ctx.project(&*self.repositories).await?;
        let owned_project_id = project_with_owners.as_ref().map(|p| p.project.id().clone());

        let id = FormAnswerId::try_from(form_answer_data.id)?;
        let form_answer = self
            .repositories
            .form_answer_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(FormAnswerUseCaseError::NotFound(id))?;

        ensure!(form_answer.is_updatable_by(&actor, owned_project_id.clone()));

        let mut new_form_answer = form_answer;
        let new_items = form_answer_data
            .items
            .into_iter()
            .map(FormAnswerItem::try_from)
            .collect::<Result<_, _>>()?;
        new_form_answer.set_items(&actor, owned_project_id, new_items)?;

        let form_id = new_form_answer.form_id().clone();
        let form = self
            .repositories
            .form_repository()
            .find_by_id(form_id.clone())
            .await?
            .ok_or(FormAnswerUseCaseError::FormNotFound(form_id))?;

        verify_form_answer::verify(&form, &new_form_answer)?;

        self.repositories
            .form_answer_repository()
            .update(new_form_answer)
            .await?;
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
        form_answer::{
            dto::FormAnswerItemDto, interactor::update::UpdateFormAnswerCommand, FormAnswerUseCase,
            FormAnswerUseCaseError,
        },
        shared::context::TestContext,
    };

    #[tokio::test]
    async fn 一般ユーザーは自分の企画の回答を更新できる() {
        let mut repositories = MockRepositories::default();
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
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::form_answer::form_answer1(
                    fixture::project::id1(),
                )))
            });
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::form::form1())));
        repositories
            .form_answer_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let use_case = FormAnswerUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .update(
                &ctx,
                UpdateFormAnswerCommand {
                    id: fixture::form_answer::id1().value().to_string(),
                    items: fixture::form_answer::items2()
                        .into_iter()
                        .map(FormAnswerItemDto::from)
                        .collect(),
                },
            )
            .await;
        assert!(matches!(res, Ok(())));
    }

    #[tokio::test]
    async fn 一般ユーザーは他人の企画の回答を更新できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_owner_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners2(
                    fixture::user::user1(UserRole::General),
                )))
            });
        repositories
            .form_answer_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::form_answer::form_answer1(
                    fixture::project::id1(),
                )))
            });
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::form::form1())));
        repositories
            .form_answer_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let use_case = FormAnswerUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .update(
                &ctx,
                UpdateFormAnswerCommand {
                    id: fixture::form_answer::id1().value().to_string(),
                    items: fixture::form_answer::items2()
                        .into_iter()
                        .map(FormAnswerItemDto::from)
                        .collect(),
                },
            )
            .await;
        assert!(matches!(
            res,
            Err(FormAnswerUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn 実委人管理者は他人の企画の回答を更新できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_owner_id()
            .returning(|_| Ok(None));
        repositories
            .project_repository_mut()
            .expect_find_by_sub_owner_id()
            .returning(|_| Ok(None));
        repositories
            .form_answer_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::form_answer::form_answer1(
                    fixture::project::id1(),
                )))
            });
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::form::form1())));
        repositories
            .form_answer_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let use_case = FormAnswerUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .update(
                &ctx,
                UpdateFormAnswerCommand {
                    id: fixture::form_answer::id1().value().to_string(),
                    items: fixture::form_answer::items2()
                        .into_iter()
                        .map(FormAnswerItemDto::from)
                        .collect(),
                },
            )
            .await;
        assert!(matches!(res, Ok(())));
    }
}
