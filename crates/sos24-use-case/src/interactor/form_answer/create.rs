use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::permission::Permissions,
    repository::{
        form::FormRepository, form_answer::FormAnswerRepository, project::ProjectRepository,
        Repositories,
    },
    service::verify_form_answer,
};
use sos24_domain::entity::form_answer::FormAnswerItemKind;
use sos24_domain::repository::file_data::FileDataRepository;

use crate::{
    context::Context,
    dto::{form_answer::CreateFormAnswerDto, ToEntity},
};

use super::{FormAnswerUseCase, FormAnswerUseCaseError};

impl<R: Repositories> FormAnswerUseCase<R> {
    pub async fn create(
        &self,
        ctx: &Context,
        form_answer: CreateFormAnswerDto,
    ) -> Result<(), FormAnswerUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::CREATE_FORM_ANSWER));

        let form_answer = form_answer.into_entity()?;

        let prev_form_answer = self
            .repositories
            .form_answer_repository()
            .find_by_project_id_and_form_id(
                form_answer.project_id().clone(),
                form_answer.form_id().clone(),
            )
            .await?;
        if prev_form_answer.is_some() {
            return Err(FormAnswerUseCaseError::AlreadyAnswered);
        }

        let project = self
            .repositories
            .project_repository()
            .find_by_id(form_answer.project_id().clone())
            .await?
            .ok_or(FormAnswerUseCaseError::ProjectNotFound(
                form_answer.project_id().clone(),
            ))?;

        ensure!(project.value.is_visible_to(&actor));

        let form = self
            .repositories
            .form_repository()
            .find_by_id(form_answer.form_id().clone())
            .await?
            .ok_or(FormAnswerUseCaseError::FormNotFound(
                form_answer.form_id().clone(),
            ))?;

        for item in form_answer.items() {
            if let FormAnswerItemKind::File(value) = item.kind() {
                for file_id in value.clone().value() {
                    let _ = self.repositories.file_data_repository().find_by_id(file_id.clone()).await?.ok_or(FormAnswerUseCaseError::FileNotFound(file_id))?;
                }
            }
        }

        verify_form_answer::verify(&form.value, &form_answer)?;

        self.repositories
            .form_answer_repository()
            .create(form_answer)
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
        context::Context,
        dto::{
            form_answer::{CreateFormAnswerDto, FormAnswerItemDto},
            FromEntity,
        },
        interactor::form_answer::{FormAnswerUseCase, FormAnswerUseCaseError},
    };

    #[tokio::test]
    async fn 一般ユーザーは自分の企画の回答を作成できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_answer_repository_mut()
            .expect_find_by_project_id_and_form_id()
            .returning(|_, _| Ok(None));
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project1(
                    fixture::user::id1(),
                ))))
            });
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::date::with(fixture::form::form1()))));
        repositories
            .form_answer_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = FormAnswerUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .create(
                &ctx,
                CreateFormAnswerDto::new(
                    fixture::project::id1().value().to_string(),
                    fixture::form::id1().value().to_string(),
                    fixture::form_answer::items1()
                        .into_iter()
                        .map(FormAnswerItemDto::from_entity)
                        .collect(),
                ),
            )
            .await;
        assert!(matches!(res, Ok(())));
    }

    #[tokio::test]
    async fn 一般ユーザーは他人の企画の回答を作成できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_answer_repository_mut()
            .expect_find_by_project_id_and_form_id()
            .returning(|_, _| Ok(None));
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project1(
                    fixture::user::id2(),
                ))))
            });
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::date::with(fixture::form::form1()))));
        repositories
            .form_answer_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = FormAnswerUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .create(
                &ctx,
                CreateFormAnswerDto::new(
                    fixture::project::id1().value().to_string(),
                    fixture::form::id1().value().to_string(),
                    fixture::form_answer::items1()
                        .into_iter()
                        .map(FormAnswerItemDto::from_entity)
                        .collect(),
                ),
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
    async fn 実委人管理者は他人の企画の回答を作成できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_answer_repository_mut()
            .expect_find_by_project_id_and_form_id()
            .returning(|_, _| Ok(None));
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project1(
                    fixture::user::id2(),
                ))))
            });
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::date::with(fixture::form::form1()))));
        repositories
            .form_answer_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = FormAnswerUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .create(
                &ctx,
                CreateFormAnswerDto::new(
                    fixture::project::id1().value().to_string(),
                    fixture::form::id1().value().to_string(),
                    fixture::form_answer::items1()
                        .into_iter()
                        .map(FormAnswerItemDto::from_entity)
                        .collect(),
                ),
            )
            .await;
        assert!(matches!(res, Ok(())));
    }

    #[tokio::test]
    async fn すでに回答がある場合はエラーになる() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_answer_repository_mut()
            .expect_find_by_project_id_and_form_id()
            .returning(|_, _| {
                Ok(Some(fixture::date::with(
                    fixture::form_answer::form_answer1(fixture::project::id1()),
                )))
            });
        repositories
            .project_repository_mut()
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project1(
                    fixture::user::id1(),
                ))))
            });
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::date::with(fixture::form::form1()))));
        repositories
            .form_answer_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = FormAnswerUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .create(
                &ctx,
                CreateFormAnswerDto::new(
                    fixture::project::id1().value().to_string(),
                    fixture::form::id1().value().to_string(),
                    fixture::form_answer::items1()
                        .into_iter()
                        .map(FormAnswerItemDto::from_entity)
                        .collect(),
                ),
            )
            .await;
        assert!(matches!(res, Err(FormAnswerUseCaseError::AlreadyAnswered)));
    }
}
