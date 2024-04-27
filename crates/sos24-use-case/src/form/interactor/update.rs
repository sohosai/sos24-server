use sos24_domain::entity::file_data::FileId;
use sos24_domain::{
    ensure,
    entity::{
        common::datetime::DateTime,
        form::{FormDescription, FormId, FormTitle},
        permission::Permissions,
    },
    repository::{form::FormRepository, form_answer::FormAnswerRepository, Repositories},
};

use crate::form::dto::NewFormItemDto;
use crate::form::{FormUseCase, FormUseCaseError};
use crate::project::dto::{ProjectAttributeDto, ProjectCategoryDto};
use crate::shared::adapter::Adapters;
use crate::shared::context::ContextProvider;
use crate::ToEntity;

#[derive(Debug)]
pub struct UpdateFormCommand {
    pub id: String,
    pub title: String,
    pub description: String,
    pub starts_at: String,
    pub ends_at: String,
    pub categories: Vec<ProjectCategoryDto>,
    pub attributes: Vec<ProjectAttributeDto>,
    pub items: Vec<NewFormItemDto>,
    pub attachments: Vec<String>,
}

impl<R: Repositories, A: Adapters> FormUseCase<R, A> {
    pub async fn update(
        &self,
        ctx: &impl ContextProvider,
        form_data: UpdateFormCommand,
    ) -> Result<(), FormUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
        ensure!(actor.has_permission(Permissions::UPDATE_FORM_ALL));

        let id = FormId::try_from(form_data.id)?;
        let form = self
            .repositories
            .form_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(FormUseCaseError::NotFound(id.clone()))?;

        let answers = self
            .repositories
            .form_answer_repository()
            .find_by_form_id(id.clone())
            .await?;
        if !answers.is_empty() {
            return Err(FormUseCaseError::HasAnswers);
        }

        let mut new_form = form.value;
        new_form.set_title(&actor, FormTitle::new(form_data.title))?;
        new_form.set_description(&actor, FormDescription::new(form_data.description))?;
        new_form.set_starts_at(&actor, DateTime::try_from(form_data.starts_at)?)?;
        new_form.set_ends_at(&actor, DateTime::try_from(form_data.ends_at)?)?;
        new_form.set_categories(&actor, form_data.categories.into_entity()?)?;
        new_form.set_attributes(&actor, form_data.attributes.into_entity()?)?;
        let new_items = form_data
            .items
            .into_iter()
            .map(|item| item.into_entity())
            .collect::<Result<_, _>>()?;
        new_form.set_items(&actor, new_items)?;
        let new_attachments = form_data
            .attachments
            .into_iter()
            .map(FileId::try_from)
            .collect::<Result<_, _>>()?;
        new_form.set_attachments(&actor, new_attachments)?;

        self.repositories.form_repository().update(new_form).await?;
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
        form::{
            dto::{FormItemKindDto, NewFormItemDto},
            interactor::update::UpdateFormCommand,
            FormUseCase, FormUseCaseError,
        },
        shared::{adapter::MockAdapters, context::TestContext},
        FromEntity,
    };

    #[tokio::test]
    async fn 実委人は申請を更新できない() {
        let repositories = MockRepositories::default();
        let adapters = MockAdapters::default();
        let use_case = FormUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .update(
                &ctx,
                UpdateFormCommand {
                    id: fixture::form::id1().value().to_string(),
                    title: fixture::form::title2().value(),
                    description: fixture::form::description2().value(),
                    starts_at: fixture::form::starts_at2().value().to_rfc3339(),
                    ends_at: fixture::form::ends_at2().value().to_rfc3339(),
                    categories: Vec::from_entity(fixture::form::categories2()),
                    attributes: Vec::from_entity(fixture::form::attributes2()),
                    items: vec![NewFormItemDto::new(
                        fixture::form::formitem_name2().value(),
                        Some(fixture::form::description2().value()),
                        fixture::form::formitem_required2().value(),
                        FormItemKindDto::from_entity(fixture::form::formitem_kind2()),
                    )],
                    attachments: fixture::form::attachments2()
                        .into_iter()
                        .map(|it| it.value().to_string())
                        .collect(),
                },
            )
            .await;
        assert!(matches!(
            res,
            Err(FormUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            )),
        ));
    }

    #[tokio::test]
    async fn 実委人管理者は申請を更新できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::date::with(fixture::form::form1()))));
        repositories
            .form_answer_repository_mut()
            .expect_find_by_form_id()
            .returning(|_| Ok(vec![]));
        repositories
            .form_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let adapters = MockAdapters::default();
        let use_case = FormUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .update(
                &ctx,
                UpdateFormCommand {
                    id: fixture::form::id1().value().to_string(),
                    title: fixture::form::title2().value(),
                    description: fixture::form::description2().value(),
                    starts_at: fixture::form::starts_at2().value().to_rfc3339(),
                    ends_at: fixture::form::ends_at2().value().to_rfc3339(),
                    categories: Vec::from_entity(fixture::form::categories2()),
                    attributes: Vec::from_entity(fixture::form::attributes2()),
                    items: vec![NewFormItemDto::new(
                        fixture::form::formitem_name2().value(),
                        Some(fixture::form::description2().value()),
                        fixture::form::formitem_required2().value(),
                        FormItemKindDto::from_entity(fixture::form::formitem_kind2()),
                    )],
                    attachments: fixture::form::attachments2()
                        .into_iter()
                        .map(|it| it.value().to_string())
                        .collect(),
                },
            )
            .await;
        assert!(matches!(res, Ok(())));
    }

    #[tokio::test]
    async fn 回答がある申請は更新できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::date::with(fixture::form::form1()))));
        repositories
            .form_answer_repository_mut()
            .expect_find_by_form_id()
            .returning(|_| {
                Ok(vec![fixture::date::with(
                    fixture::form_answer::form_answer1(fixture::project::id1()),
                )])
            });
        let adapters = MockAdapters::default();
        let use_case = FormUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .update(
                &ctx,
                UpdateFormCommand {
                    id: fixture::form::id1().value().to_string(),
                    title: fixture::form::title2().value(),
                    description: fixture::form::description2().value(),
                    starts_at: fixture::form::starts_at2().value().to_rfc3339(),
                    ends_at: fixture::form::ends_at2().value().to_rfc3339(),
                    categories: Vec::from_entity(fixture::form::categories2()),
                    attributes: Vec::from_entity(fixture::form::attributes2()),
                    items: vec![NewFormItemDto::new(
                        fixture::form::formitem_name2().value(),
                        Some(fixture::form::description2().value()),
                        fixture::form::formitem_required2().value(),
                        FormItemKindDto::from_entity(fixture::form::formitem_kind2()),
                    )],
                    attachments: fixture::form::attachments2()
                        .into_iter()
                        .map(|it| it.value().to_string())
                        .collect(),
                },
            )
            .await;
        assert!(matches!(res, Err(FormUseCaseError::HasAnswers)));
    }
}
