use sos24_domain::entity::form::FormId;
use sos24_domain::entity::form_answer::{FormAnswer, FormAnswerItem, FormAnswerItemKind};
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

use crate::form_answer::dto::FormAnswerItemDto;
use crate::form_answer::{FormAnswerUseCase, FormAnswerUseCaseError};
use crate::shared::context::ContextProvider;

#[derive(Debug)]
pub struct CreateFormAnswerCommand {
    pub form_id: String,
    pub items: Vec<FormAnswerItemDto>,
}

impl<R: Repositories> FormAnswerUseCase<R> {
    pub async fn create(
        &self,
        ctx: &impl ContextProvider,
        form_answer: CreateFormAnswerCommand,
    ) -> Result<String, FormAnswerUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
        ensure!(actor.has_permission(Permissions::CREATE_FORM_ANSWER));

        let Some(project_with_owners) = ctx.project(&*self.repositories).await? else {
            return Err(FormAnswerUseCaseError::NotProjectOwner);
        };
        let project_id = project_with_owners.project.id().clone();

        let form_answer = FormAnswer::create(
            project_id,
            FormId::try_from(form_answer.form_id)?,
            form_answer
                .items
                .into_iter()
                .map(FormAnswerItem::try_from)
                .collect::<Result<_, _>>()?,
        );

        let project_with_owners = self
            .repositories
            .project_repository()
            .find_by_id(form_answer.project_id().clone())
            .await?
            .ok_or(FormAnswerUseCaseError::ProjectNotFound(
                form_answer.project_id().clone(),
            ))?;

        ensure!(project_with_owners.project.is_visible_to(&actor));

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

        verify_form_answer::verify(&form, &form_answer)?;

        let form_answer_id = {
            let lock = self.creation_lock.lock().await;

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

            let form_answer_id = form_answer.id().clone();
            self.repositories
                .form_answer_repository()
                .create(form_answer)
                .await?;

            drop(lock);
            form_answer_id
        };

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
        form_answer::{
            dto::FormAnswerItemDto, interactor::create::CreateFormAnswerCommand, FormAnswerUseCase,
            FormAnswerUseCaseError,
        },
        shared::context::TestContext,
    };

    #[tokio::test]
    async fn 一般ユーザーは自分の企画の回答を作成できる() {
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
            .form_answer_repository_mut()
            .expect_find_by_project_id_and_form_id()
            .returning(|_, _| Ok(None));
        repositories
            .project_repository_mut()
            .expect_find_by_owner_id()
            .returning(|_| {
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user1(UserRole::General),
                )))
            });
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::form::form1_opened())));
        repositories
            .form_answer_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = FormAnswerUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .create(
                &ctx,
                CreateFormAnswerCommand {
                    form_id: fixture::form::id1().value().to_string(),
                    items: fixture::form_answer::items1()
                        .into_iter()
                        .map(FormAnswerItemDto::from)
                        .collect(),
                },
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
                CreateFormAnswerCommand {
                    form_id: fixture::form::id1().value().to_string(),
                    items: fixture::form_answer::items1()
                        .into_iter()
                        .map(FormAnswerItemDto::from)
                        .collect(),
                },
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
                Ok(Some(fixture::project::project_with_owners1(
                    fixture::user::user1(UserRole::General),
                )))
            });
        repositories
            .form_answer_repository_mut()
            .expect_find_by_project_id_and_form_id()
            .returning(|_, _| {
                Ok(Some(fixture::form_answer::form_answer1(
                    fixture::project::id1(),
                )))
            });
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
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::form::form1_opened())));
        repositories
            .form_answer_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = FormAnswerUseCase::new(Arc::new(repositories));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .create(
                &ctx,
                CreateFormAnswerCommand {
                    form_id: fixture::form::id1().value().to_string(),
                    items: fixture::form_answer::items1()
                        .into_iter()
                        .map(FormAnswerItemDto::from)
                        .collect(),
                },
            )
            .await;
        assert!(matches!(res, Err(FormAnswerUseCaseError::AlreadyAnswered)));
    }
}
