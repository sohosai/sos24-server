use std::sync::Arc;

use sos24_domain::entity::file_data::FileId;
use sos24_domain::entity::form::FormItem;
use sos24_domain::entity::project::{ProjectAttributes, ProjectCategories};
use sos24_domain::{
    ensure,
    entity::{
        common::datetime::DateTime,
        form::{FormDescription, FormId, FormTitle},
        permission::Permissions,
    },
    repository::{form::FormRepository, form_answer::FormAnswerRepository, Repositories},
};

use crate::context::Context;
use crate::form::dto::NewFormItemDto;
use crate::project::dto::{ProjectAttributesDto, ProjectCategoriesDto};

use super::{FormUseCase, FormUseCaseError};

pub struct UpdateFormCommand {
    pub title: String,
    pub description: String,
    pub starts_at: String,
    pub ends_at: String,
    pub categories: ProjectCategoriesDto,
    pub attributes: ProjectAttributesDto,
    pub items: Vec<NewFormItemDto>,
    pub attachments: Vec<String>,
}

impl<R: Repositories> FormUseCase<R> {
    pub async fn update(
        &self,
        ctx: &Context,
        form_id: String,
        form_data: UpdateFormCommand,
    ) -> Result<(), FormUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::UPDATE_FORM_ALL));

        let id = FormId::try_from(form_id)?;
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
        new_form.set_categories(&actor, ProjectCategories::from(form_data.categories))?;
        new_form.set_attributes(&actor, ProjectAttributes::from(form_data.attributes))?;
        let new_items = form_data.items.into_iter().map(FormItem::from).collect();
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
        context::Context,
        form::{
            dto::{FormItemKindDto, NewFormItemDto},
            use_case::{update::UpdateFormCommand, FormUseCase, FormUseCaseError},
        },
        project::dto::{ProjectAttributesDto, ProjectCategoriesDto},
    };

    #[tokio::test]
    async fn 実委人は申請を更新できない() {
        let repositories = MockRepositories::default();
        let use_case = FormUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .update(
                &ctx,
                fixture::form::id1().value().to_string(),
                UpdateFormCommand {
                    title: fixture::form::title2().value(),
                    description: fixture::form::description2().value(),
                    starts_at: fixture::form::starts_at2().value().to_rfc3339(),
                    ends_at: fixture::form::ends_at2().value().to_rfc3339(),
                    categories: ProjectCategoriesDto::from(fixture::form::categories2()),
                    attributes: ProjectAttributesDto::from(fixture::form::attributes2()),
                    items: vec![NewFormItemDto {
                        name: fixture::form::formitem_name2().value(),
                        description: fixture::form::description2().value(),
                        required: fixture::form::formitem_required2().value(),
                        kind: FormItemKindDto::from(fixture::form::formitem_kind2()),
                    }],
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
        let use_case = FormUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .update(
                &ctx,
                fixture::form::id1().value().to_string(),
                UpdateFormCommand {
                    title: fixture::form::title2().value(),
                    description: fixture::form::description2().value(),
                    starts_at: fixture::form::starts_at2().value().to_rfc3339(),
                    ends_at: fixture::form::ends_at2().value().to_rfc3339(),
                    categories: ProjectCategoriesDto::from(fixture::form::categories2()),
                    attributes: ProjectAttributesDto::from(fixture::form::attributes2()),
                    items: vec![NewFormItemDto {
                        name: fixture::form::formitem_name2().value(),
                        description: fixture::form::description2().value(),
                        required: fixture::form::formitem_required2().value(),
                        kind: FormItemKindDto::from(fixture::form::formitem_kind2()),
                    }],
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
        let use_case = FormUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .update(
                &ctx,
                fixture::form::id1().value().to_string(),
                UpdateFormCommand {
                    title: fixture::form::title2().value(),
                    description: fixture::form::description2().value(),
                    starts_at: fixture::form::starts_at2().value().to_rfc3339(),
                    ends_at: fixture::form::ends_at2().value().to_rfc3339(),
                    categories: ProjectCategoriesDto::from(fixture::form::categories2()),
                    attributes: ProjectAttributesDto::from(fixture::form::attributes2()),
                    items: vec![NewFormItemDto {
                        name: fixture::form::formitem_name2().value(),
                        description: fixture::form::description2().value(),
                        required: fixture::form::formitem_required2().value(),
                        kind: FormItemKindDto::from(fixture::form::formitem_kind2()),
                    }],
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
