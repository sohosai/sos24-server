use std::sync::Arc;

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

use crate::adapter::Adapters;
use crate::{
    context::Context,
    dto::{form::UpdateFormDto, ToEntity},
};

use super::{FormUseCase, FormUseCaseError};

impl<R: Repositories, A: Adapters> FormUseCase<R, A> {
    pub async fn update(
        &self,
        ctx: &Context,
        form_data: UpdateFormDto,
    ) -> Result<(), FormUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
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
        adapter::MockAdapters,
        context::Context,
        dto::{
            form::{FormItemKindDto, NewFormItemDto, UpdateFormDto},
            FromEntity,
        },
        interactor::form::{FormUseCase, FormUseCaseError},
    };

    #[tokio::test]
    async fn 実委人は申請を更新できない() {
        let repositories = MockRepositories::default();
        let adapters = MockAdapters::default();
        let use_case = FormUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .update(
                &ctx,
                UpdateFormDto::new(
                    fixture::form::id1().value().to_string(),
                    fixture::form::title2().value(),
                    fixture::form::description2().value(),
                    fixture::form::starts_at2().value().to_rfc3339(),
                    fixture::form::ends_at2().value().to_rfc3339(),
                    Vec::from_entity(fixture::form::categories2()),
                    Vec::from_entity(fixture::form::attributes2()),
                    vec![NewFormItemDto::new(
                        fixture::form::formitem_name2().value(),
                        fixture::form::description2().value(),
                        fixture::form::formitem_required2().value(),
                        FormItemKindDto::from_entity(fixture::form::formitem_kind2()),
                    )],
                    fixture::form::attachments2()
                        .into_iter()
                        .map(|it| it.value().to_string())
                        .collect(),
                ),
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

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .update(
                &ctx,
                UpdateFormDto::new(
                    fixture::form::id1().value().to_string(),
                    fixture::form::title2().value(),
                    fixture::form::description2().value(),
                    fixture::form::starts_at2().value().to_rfc3339(),
                    fixture::form::ends_at2().value().to_rfc3339(),
                    Vec::from_entity(fixture::form::categories2()),
                    Vec::from_entity(fixture::form::attributes2()),
                    vec![NewFormItemDto::new(
                        fixture::form::formitem_name2().value(),
                        fixture::form::description2().value(),
                        fixture::form::formitem_required2().value(),
                        FormItemKindDto::from_entity(fixture::form::formitem_kind2()),
                    )],
                    fixture::form::attachments2()
                        .into_iter()
                        .map(|it| it.value().to_string())
                        .collect(),
                ),
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

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .update(
                &ctx,
                UpdateFormDto::new(
                    fixture::form::id1().value().to_string(),
                    fixture::form::title2().value(),
                    fixture::form::description2().value(),
                    fixture::form::starts_at2().value().to_rfc3339(),
                    fixture::form::ends_at2().value().to_rfc3339(),
                    Vec::from_entity(fixture::form::categories2()),
                    Vec::from_entity(fixture::form::attributes2()),
                    vec![NewFormItemDto::new(
                        fixture::form::formitem_name2().value(),
                        fixture::form::description2().value(),
                        fixture::form::formitem_required2().value(),
                        FormItemKindDto::from_entity(fixture::form::formitem_kind2()),
                    )],
                    fixture::form::attachments2()
                        .into_iter()
                        .map(|it| it.value().to_string())
                        .collect(),
                ),
            )
            .await;
        assert!(matches!(res, Err(FormUseCaseError::HasAnswers)));
    }
}
