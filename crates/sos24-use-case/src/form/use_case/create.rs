use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::{
        common::datetime::DateTime,
        file_data::FileId,
        form::{Form, FormDescription, FormItem, FormTitle},
        permission::Permissions,
        project::{ProjectAttributes, ProjectCategories},
    },
    repository::{form::FormRepository, Repositories},
};

use crate::{
    context::Context,
    form::dto::NewFormItemDto,
    project::dto::{ProjectAttributesDto, ProjectCategoriesDto},
};

use super::{FormUseCase, FormUseCaseError};

pub struct CreateFormCommand {
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
    pub async fn create(
        &self,
        ctx: &Context,
        form_data: CreateFormCommand,
    ) -> Result<String, FormUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::CREATE_FORM));

        let form = Form::create(
            FormTitle::new(form_data.title),
            FormDescription::new(form_data.description),
            DateTime::try_from(form_data.starts_at)?,
            DateTime::try_from(form_data.ends_at)?,
            ProjectCategories::from(form_data.categories),
            ProjectAttributes::from(form_data.attributes),
            form_data.items.into_iter().map(FormItem::from).collect(),
            form_data
                .attachments
                .into_iter()
                .map(FileId::try_from)
                .collect::<Result<_, _>>()?,
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
        context::Context,
        form::{
            dto::{FormItemKindDto, NewFormItemDto},
            use_case::{create::CreateFormCommand, FormUseCase, FormUseCaseError},
        },
        project::dto::{ProjectAttributesDto, ProjectCategoriesDto},
    };

    #[tokio::test]
    async fn 実委人は申請を作成できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = FormUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .create(
                &ctx,
                CreateFormCommand {
                    title: fixture::form::title1().value(),
                    description: fixture::form::description1().value(),
                    starts_at: fixture::form::starts_at1().value().to_rfc3339(),
                    ends_at: fixture::form::ends_at1().value().to_rfc3339(),
                    categories: ProjectCategoriesDto::from(fixture::form::categories1()),
                    attributes: ProjectAttributesDto::from(fixture::form::attributes1()),
                    items: vec![NewFormItemDto {
                        name: fixture::form::formitem_name1().value(),
                        description: fixture::form::description1().value(),
                        required: fixture::form::formitem_required1().value(),
                        kind: FormItemKindDto::from(fixture::form::formitem_kind1()),
                    }],
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
        let use_case = FormUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .create(
                &ctx,
                CreateFormCommand {
                    title: fixture::form::title1().value(),
                    description: fixture::form::description1().value(),
                    starts_at: fixture::form::starts_at1().value().to_rfc3339(),
                    ends_at: fixture::form::ends_at1().value().to_rfc3339(),
                    categories: ProjectCategoriesDto::from(fixture::form::categories1()),
                    attributes: ProjectAttributesDto::from(fixture::form::attributes1()),
                    items: vec![NewFormItemDto {
                        name: fixture::form::formitem_name1().value(),
                        description: fixture::form::description1().value(),
                        required: fixture::form::formitem_required1().value(),
                        kind: FormItemKindDto::from(fixture::form::formitem_kind1()),
                    }],
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
