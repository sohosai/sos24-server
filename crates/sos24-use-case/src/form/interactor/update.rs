use sos24_domain::entity::file_data::FileId;
use sos24_domain::entity::form::{FormIsDraft, FormItem};
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

use crate::form::dto::{FormIsDraftDto, NewFormItemDto};
use crate::form::{FormUseCase, FormUseCaseError};
use crate::project::dto::{ProjectAttributesDto, ProjectCategoriesDto};
use crate::shared::adapter::Adapters;
use crate::shared::context::ContextProvider;

#[derive(Debug)]
pub struct UpdateFormCommand {
    pub id: String,
    pub title: String,
    pub description: String,
    pub is_draft: FormIsDraftDto,
    pub starts_at: String,
    pub ends_at: String,
    pub categories: ProjectCategoriesDto,
    pub attributes: ProjectAttributesDto,
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
        let has_answer = !answers.is_empty();

        // 回答がある場合、SOS管理者以外は変更できない
        if has_answer && !actor.has_permission(Permissions::UPDATE_FORM_ALL_ANSWERED) {
            return Err(FormUseCaseError::HasAnswers);
        }

        let mut new_form = form.clone();
        new_form.set_title(&actor, FormTitle::new(form_data.title))?;
        new_form.set_description(&actor, FormDescription::new(form_data.description))?;
        new_form.set_starts_at(&actor, DateTime::try_from(form_data.starts_at)?)?;
        new_form.set_ends_at(&actor, DateTime::try_from(form_data.ends_at)?)?;
        new_form.set_categories(&actor, ProjectCategories::from(form_data.categories))?;
        new_form.set_attributes(&actor, ProjectAttributes::from(form_data.attributes))?;
        new_form.set_is_draft(
            &actor,
            FormIsDraft::from(form_data.is_draft),
            ctx.requested_at(),
        )?;
        // 下書き <-> 公開 の遷移の権限はset_is_draft内部で確認される

        {
            let new_attachments = form_data
                .attachments
                .into_iter()
                .map(FileId::try_from)
                .collect::<Result<_, _>>()?;
            new_form.set_attachments(&actor, new_attachments)?;
        }
        // 回答がない場合のみ、申請項目を更新
        if !has_answer {
            let new_items = form_data
                .items
                .into_iter()
                .map(FormItem::try_from)
                .collect::<Result<_, _>>()?;
            new_form.set_items(&actor, new_items)?;
        }

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
            dto::{FormIsDraftDto, FormItemKindDto, NewFormItemDto},
            interactor::update::UpdateFormCommand,
            FormUseCase, FormUseCaseError,
        },
        project::dto::{ProjectAttributesDto, ProjectCategoriesDto},
        shared::{adapter::MockAdapters, context::TestContext},
    };

    #[tokio::test]
    async fn 実委人閲覧者は受付中の申請を更新できない() {
        let repositories = MockRepositories::default();
        let adapters = MockAdapters::default();
        let use_case = FormUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeViewer));
        let res = use_case
            .update(
                &ctx,
                UpdateFormCommand {
                    id: fixture::form::id1().value().to_string(),
                    title: fixture::form::title2().value(),
                    description: fixture::form::description2().value(),
                    is_draft: FormIsDraftDto::from(fixture::form::is_draft1()),
                    starts_at: fixture::form::starts_at2().value().to_rfc3339(),
                    ends_at: fixture::form::ends_at2().value().to_rfc3339(),
                    categories: ProjectCategoriesDto::from(fixture::form::categories2()),
                    attributes: ProjectAttributesDto::from(fixture::form::attributes2()),
                    items: vec![NewFormItemDto::new(
                        fixture::form::formitem_name2().value(),
                        Some(fixture::form::description2().value()),
                        fixture::form::formitem_required2().value(),
                        FormItemKindDto::from(fixture::form::formitem_kind2()),
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
    async fn 実委人起草者は受付中の申請を更新できない() {
        let repositories = MockRepositories::default();
        let adapters = MockAdapters::default();
        let use_case = FormUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeDrafter));
        let res = use_case
            .update(
                &ctx,
                UpdateFormCommand {
                    id: fixture::form::id1().value().to_string(),
                    title: fixture::form::title2().value(),
                    description: fixture::form::description2().value(),
                    is_draft: FormIsDraftDto::from(fixture::form::is_draft1()),
                    starts_at: fixture::form::starts_at2().value().to_rfc3339(),
                    ends_at: fixture::form::ends_at2().value().to_rfc3339(),
                    categories: ProjectCategoriesDto::from(fixture::form::categories2()),
                    attributes: ProjectAttributesDto::from(fixture::form::attributes2()),
                    items: vec![NewFormItemDto::new(
                        fixture::form::formitem_name2().value(),
                        Some(fixture::form::description2().value()),
                        fixture::form::formitem_required2().value(),
                        FormItemKindDto::from(fixture::form::formitem_kind2()),
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
    async fn 実委人編集者は受付中の申請を更新できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::form::form1_opened())));
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

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeEditor));
        let res = use_case
            .update(
                &ctx,
                UpdateFormCommand {
                    id: fixture::form::id1().value().to_string(),
                    title: fixture::form::title2().value(),
                    description: fixture::form::description2().value(),
                    is_draft: FormIsDraftDto::from(fixture::form::is_draft1()),
                    starts_at: fixture::form::starts_at2().value().to_rfc3339(),
                    ends_at: fixture::form::ends_at2().value().to_rfc3339(),
                    categories: ProjectCategoriesDto::from(fixture::form::categories2()),
                    attributes: ProjectAttributesDto::from(fixture::form::attributes2()),
                    items: vec![NewFormItemDto::new(
                        fixture::form::formitem_name2().value(),
                        Some(fixture::form::description2().value()),
                        fixture::form::formitem_required2().value(),
                        FormItemKindDto::from(fixture::form::formitem_kind2()),
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
    async fn 実委人管理者は受付中の申請を更新できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::form::form1_opened())));
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
                    is_draft: FormIsDraftDto::from(fixture::form::is_draft1()),
                    starts_at: fixture::form::starts_at2().value().to_rfc3339(),
                    ends_at: fixture::form::ends_at2().value().to_rfc3339(),
                    categories: ProjectCategoriesDto::from(fixture::form::categories2()),
                    attributes: ProjectAttributesDto::from(fixture::form::attributes2()),
                    items: vec![NewFormItemDto::new(
                        fixture::form::formitem_name2().value(),
                        Some(fixture::form::description2().value()),
                        fixture::form::formitem_required2().value(),
                        FormItemKindDto::from(fixture::form::formitem_kind2()),
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
    async fn 実委人管理者は回答がある申請を更新できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::form::form1_opened())));
        repositories
            .form_answer_repository_mut()
            .expect_find_by_form_id()
            .returning(|_| {
                Ok(vec![fixture::form_answer::form_answer1(
                    fixture::project::id1(),
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
                    is_draft: FormIsDraftDto::from(fixture::form::is_draft1()),
                    starts_at: fixture::form::starts_at2().value().to_rfc3339(),
                    ends_at: fixture::form::ends_at2().value().to_rfc3339(),
                    categories: ProjectCategoriesDto::from(fixture::form::categories2()),
                    attributes: ProjectAttributesDto::from(fixture::form::attributes2()),
                    items: vec![NewFormItemDto::new(
                        fixture::form::formitem_name2().value(),
                        Some(fixture::form::description2().value()),
                        fixture::form::formitem_required2().value(),
                        FormItemKindDto::from(fixture::form::formitem_kind2()),
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

    #[tokio::test]
    #[allow(non_snake_case)]
    async fn SOS管理者は回答がある申請を更新できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::form::form1_opened())));
        repositories
            .form_answer_repository_mut()
            .expect_find_by_form_id()
            .returning(|_| {
                Ok(vec![fixture::form_answer::form_answer1(
                    fixture::project::id1(),
                )])
            });
        repositories
            .form_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let adapters = MockAdapters::default();
        let use_case = FormUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::Administrator));
        let res = use_case
            .update(
                &ctx,
                UpdateFormCommand {
                    id: fixture::form::id1().value().to_string(),
                    title: fixture::form::title2().value(),
                    description: fixture::form::description2().value(),
                    is_draft: FormIsDraftDto::from(fixture::form::is_draft1()),
                    starts_at: fixture::form::starts_at2().value().to_rfc3339(),
                    ends_at: fixture::form::ends_at2().value().to_rfc3339(),
                    categories: ProjectCategoriesDto::from(fixture::form::categories2()),
                    attributes: ProjectAttributesDto::from(fixture::form::attributes2()),
                    items: vec![NewFormItemDto::new(
                        fixture::form::formitem_name1().value(),
                        Some(fixture::form::description1().value()),
                        fixture::form::formitem_required1().value(),
                        FormItemKindDto::from(fixture::form::formitem_kind1()),
                    )],
                    attachments: fixture::form::attachments2()
                        .into_iter()
                        .map(|it| it.value().to_string())
                        .collect(),
                },
            )
            .await;
        assert!(res.is_ok());
    }
}
