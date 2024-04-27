use sos24_domain::{
    ensure,
    entity::{
        common::datetime::DateTime,
        file_data::FileId,
        form::{Form, FormDescription, FormTitle},
        permission::Permissions,
    },
    repository::{form::FormRepository, Repositories},
};

use crate::{
    form::{dto::NewFormItemDto, FormUseCase, FormUseCaseError},
    project::dto::{ProjectAttributeDto, ProjectCategoryDto},
    shared::{adapter::Adapters, context::ContextProvider},
    ToEntity,
};

#[derive(Debug)]
pub struct CreateFormCommand {
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
    pub async fn create(
        &self,
        ctx: &impl ContextProvider,
        raw_form: CreateFormCommand,
    ) -> Result<String, FormUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
        ensure!(actor.has_permission(Permissions::CREATE_FORM));

        let form = Form::create(
            FormTitle::new(raw_form.title),
            FormDescription::new(raw_form.description),
            DateTime::try_from(raw_form.starts_at)?,
            DateTime::try_from(raw_form.ends_at)?,
            raw_form.categories.into_entity()?,
            raw_form.attributes.into_entity()?,
            raw_form
                .items
                .into_iter()
                .map(NewFormItemDto::into_entity)
                .collect::<Result<Vec<_>, _>>()?,
            raw_form
                .attachments
                .into_iter()
                .map(FileId::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        )?;

        let form_id = form.id().clone();
        self.repositories.form_repository().create(form).await?;
        Ok(form_id.value().to_string())
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
            interactor::create::CreateFormCommand,
            FormUseCase, FormUseCaseError,
        },
        shared::{adapter::MockAdapters, context::TestContext},
        FromEntity,
    };

    #[tokio::test]
    async fn 実委人は申請を作成できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let adapters = MockAdapters::default();
        let use_case = FormUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .create(
                &ctx,
                CreateFormCommand {
                    title: fixture::form::title1().value(),
                    description: fixture::form::description1().value(),
                    starts_at: fixture::form::starts_at1().value().to_rfc3339(),
                    ends_at: fixture::form::ends_at1().value().to_rfc3339(),
                    categories: Vec::from_entity(fixture::form::categories1()),
                    attributes: Vec::from_entity(fixture::form::attributes1()),
                    items: vec![NewFormItemDto::new(
                        fixture::form::formitem_name1().value(),
                        Some(fixture::form::description1().value()),
                        fixture::form::formitem_required1().value(),
                        FormItemKindDto::from_entity(fixture::form::formitem_kind1()),
                    )],
                    attachments: fixture::form::attachments1()
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
    async fn 実委人管理者は申請を作成できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let adapters = MockAdapters::default();
        let use_case = FormUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .create(
                &ctx,
                CreateFormCommand {
                    title: fixture::form::title1().value(),
                    description: fixture::form::description1().value(),
                    starts_at: fixture::form::starts_at1().value().to_rfc3339(),
                    ends_at: fixture::form::ends_at1().value().to_rfc3339(),
                    categories: Vec::from_entity(fixture::form::categories1()),
                    attributes: Vec::from_entity(fixture::form::attributes1()),
                    items: vec![NewFormItemDto::new(
                        fixture::form::formitem_name1().value(),
                        Some(fixture::form::description1().value()),
                        fixture::form::formitem_required1().value(),
                        FormItemKindDto::from_entity(fixture::form::formitem_kind1()),
                    )],
                    attachments: fixture::form::attachments1()
                        .into_iter()
                        .map(|it| it.value().to_string())
                        .collect(),
                },
            )
            .await;
        assert!(res.is_ok());
    }
}
