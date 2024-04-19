use sos24_domain::entity::form_answer::FormAnswerItemKind;
use sos24_domain::repository::file_data::FileDataRepository;
use sos24_domain::{
    ensure,
    entity::permission::Permissions,
    repository::{
        form::FormRepository, form_answer::FormAnswerRepository, project::ProjectRepository,
        Repositories,
    },
    service::verify_form_answer,
};

use crate::context::{ContextProvider, OwnedProject};
use crate::dto::{form_answer::CreateFormAnswerDto, ToEntity};

use super::{FormAnswerUseCase, FormAnswerUseCaseError};

impl<R: Repositories> FormAnswerUseCase<R> {
    pub async fn create(
        &self,
        ctx: &impl ContextProvider,
        form_answer: CreateFormAnswerDto,
    ) -> Result<String, FormAnswerUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
        ensure!(actor.has_permission(Permissions::CREATE_FORM_ANSWER));

        let project_id = match ctx.project(&*self.repositories).await? {
            Some(OwnedProject::Owner(project)) => project.value.id().clone(),
            Some(OwnedProject::SubOwner(project)) => project.value.id().clone(),
            None => return Err(FormAnswerUseCaseError::NotProjectOwner),
        };

        let project_id_str = project_id.value().to_string();
        let form_answer = (project_id_str, form_answer).into_entity()?;

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
                    let _ = self
                        .repositories
                        .file_data_repository()
                        .find_by_id(file_id.clone())
                        .await?
                        .ok_or(FormAnswerUseCaseError::FileNotFound(file_id))?;
                }
            }
        }

        verify_form_answer::verify(&form.value, &form_answer)?;

        let form_answer_id = form_answer.id().clone();
        self.repositories
            .form_answer_repository()
            .create(form_answer)
            .await?;

        Ok(form_answer_id.value().to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::{
        entity::user::UserRole,
        test::{fixture, repository::MockRepositories},
    };

    use crate::{
        context::TestContext,
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

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .create(
                &ctx,
                CreateFormAnswerDto::new(
                    fixture::form::id1().value().to_string(),
                    fixture::form_answer::items1()
                        .into_iter()
                        .map(FormAnswerItemDto::from_entity)
                        .collect(),
                ),
            )
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn 企画責任者でないならば回答を作成できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_find_by_owner_id()
            .returning(|_| Ok(None));
        repositories
            .project_repository_mut()
            .expect_find_by_sub_owner_id()
            .returning(|_| Ok(None));
        let use_case = FormAnswerUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .create(
                &ctx,
                CreateFormAnswerDto::new(
                    fixture::form::id1().value().to_string(),
                    fixture::form_answer::items1()
                        .into_iter()
                        .map(FormAnswerItemDto::from_entity)
                        .collect(),
                ),
            )
            .await;
        assert!(matches!(res, Err(FormAnswerUseCaseError::NotProjectOwner)));
    }

    #[tokio::test]
    async fn すでに回答がある場合はエラーになる() {
        let mut repositories = MockRepositories::default();
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

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .create(
                &ctx,
                CreateFormAnswerDto::new(
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
