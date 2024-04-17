use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::permission::Permissions,
    repository::{form::FormRepository, Repositories},
};

use crate::{
    context::Context,
    dto::{form::CreateFormDto, ToEntity},
};

use super::{FormUseCase, FormUseCaseError};

impl<R: Repositories> FormUseCase<R> {
    pub async fn create(
        &self,
        ctx: &Context,
        raw_form: CreateFormDto,
    ) -> Result<String, FormUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::CREATE_FORM));

        let form = raw_form.into_entity()?;
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
        dto::{
            form::{CreateFormDto, FormItemKindDto, NewFormItemDto},
            FromEntity,
        },
        interactor::form::{FormUseCase, FormUseCaseError},
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
                CreateFormDto::new(
                    fixture::form::title1().value(),
                    fixture::form::description1().value(),
                    fixture::form::starts_at1().value().to_rfc3339(),
                    fixture::form::ends_at1().value().to_rfc3339(),
                    Vec::from_entity(fixture::form::categories1()),
                    Vec::from_entity(fixture::form::attributes1()),
                    vec![NewFormItemDto::new(
                        fixture::form::formitem_name1().value(),
                        Some(fixture::form::description1().value()),
                        fixture::form::formitem_required1().value(),
                        FormItemKindDto::from_entity(fixture::form::formitem_kind1()),
                    )],
                    fixture::form::attachments1()
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
                CreateFormDto::new(
                    fixture::form::title1().value(),
                    fixture::form::description1().value(),
                    fixture::form::starts_at1().value().to_rfc3339(),
                    fixture::form::ends_at1().value().to_rfc3339(),
                    Vec::from_entity(fixture::form::categories1()),
                    Vec::from_entity(fixture::form::attributes1()),
                    vec![NewFormItemDto::new(
                        fixture::form::formitem_name1().value(),
                        Some(fixture::form::description1().value()),
                        fixture::form::formitem_required1().value(),
                        FormItemKindDto::from_entity(fixture::form::formitem_kind1()),
                    )],
                    fixture::form::attachments1()
                        .into_iter()
                        .map(|it| it.value().to_string())
                        .collect(),
                ),
            )
            .await;
        assert!(res.is_ok());
    }
}
