use sos24_domain::{
    ensure,
    entity::{
        file_data::FileId,
        news::{News, NewsBody, NewsTitle},
        permission::Permissions,
        project::{ProjectAttributes, ProjectCategories},
    },
    repository::{
        file_data::FileDataRepository, news::NewsRepository, project::ProjectRepository,
        Repositories,
    },
};

use crate::{
    news::{NewsUseCase, NewsUseCaseError},
    project::dto::{ProjectAttributesDto, ProjectCategoriesDto},
    shared::{
        adapter::{
            email::{Email, EmailSender, SendEmailCommand},
            notification::Notifier,
            Adapters,
        },
        app_url,
        context::ContextProvider,
    },
};

#[derive(Debug)]
pub struct CreateNewsCommand {
    pub title: String,
    pub body: String,
    pub attachments: Vec<String>,
    pub categories: ProjectCategoriesDto,
    pub attributes: ProjectAttributesDto,
}

impl<R: Repositories, A: Adapters> NewsUseCase<R, A> {
    pub async fn create(
        &self,
        ctx: &impl ContextProvider,
        raw_news: CreateNewsCommand,
    ) -> Result<String, NewsUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
        ensure!(actor.has_permission(Permissions::CREATE_NEWS));

        let news = News::create(
            NewsTitle::new(raw_news.title),
            NewsBody::new(raw_news.body),
            raw_news
                .attachments
                .into_iter()
                .map(FileId::try_from)
                .collect::<Result<_, _>>()?,
            ProjectCategories::from(raw_news.categories),
            ProjectAttributes::from(raw_news.attributes),
        );

        for file_id in news.attachments() {
            let _ = self
                .repositories
                .file_data_repository()
                .find_by_id(file_id.clone())
                .await?
                .ok_or(NewsUseCaseError::FileNotFound(file_id.clone()))?;
        }

        let news_id = news.id().clone();
        self.repositories
            .news_repository()
            .create(news.clone())
            .await?;

        let project_list = self.repositories.project_repository().list().await?;
        let target_project_list = project_list
            .into_iter()
            .filter(|project_with_owners| news.is_sent_to(&project_with_owners.project));

        let emails = target_project_list
            .flat_map(|project_with_owners| {
                [
                    Some(project_with_owners.owner.email().clone().value()),
                    project_with_owners
                        .sub_owner
                        .as_ref()
                        .map(|it| it.email().clone().value()),
                ]
            })
            .flatten()
            .collect::<Vec<_>>();

        let command = SendEmailCommand {
            from: Email {
                address: ctx.config().email_sender_address.clone(),
                name: String::from("雙峰祭オンラインシステム"),
            },
            to: emails,
            reply_to: Some(ctx.config().email_reply_to_address.clone()),
            subject: format!(
                "お知らせ「{title}」が公開されました",
                title = news.title().clone().value()
            ),
            body: format!(
                r#"雙峰祭オンラインシステムでお知らせが公開されました。

タイトル: {title}
本文:
{body}

詳細は以下のリンクから確認できます。
{url}

※このメールは雙峰祭オンラインシステムが自動送信しています。
＿＿＿
筑波大学学園祭実行委員会
Email : {email}
電話 : 029-853-2899"#,
                title = news.title().clone().value(),
                body = news.body().clone().value(),
                url = app_url::news(ctx, news.id().clone()),
                email = ctx.config().email_reply_to_address.clone(),
            ),
        };
        self.adapters.email_sender().send_email(command).await?;

        self.adapters
            .notifier()
            .notify(format!(
                "お知らせ「{}」が公開されました。\n{}",
                news.title().clone().value(),
                app_url::committee_news(ctx, news_id.clone()),
            ))
            .await?;

        Ok(news_id.value().to_string())
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
        news::{interactor::create::CreateNewsCommand, NewsUseCase, NewsUseCaseError},
        project::dto::{ProjectAttributesDto, ProjectCategoriesDto},
        shared::{adapter::MockAdapters, context::TestContext},
    };

    #[tokio::test]
    async fn 実委人はお知らせを作成できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .news_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let adapters = MockAdapters::default();
        let use_case = NewsUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .create(
                &ctx,
                CreateNewsCommand {
                    title: fixture::news::title1().value(),
                    body: fixture::news::body1().value(),
                    attachments: fixture::news::attachments1()
                        .into_iter()
                        .map(|id| id.value().to_string())
                        .collect(),
                    categories: ProjectCategoriesDto::from(fixture::news::categories1()),
                    attributes: ProjectAttributesDto::from(fixture::news::attributes1()),
                },
            )
            .await;
        assert!(matches!(
            res,
            Err(NewsUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn 実委人管理者はお知らせを作成できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .news_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        repositories
            .project_repository_mut()
            .expect_list()
            .returning(|| Ok(vec![]));
        let mut adapters = MockAdapters::default();
        adapters
            .email_sender_mut()
            .expect_send_email()
            .returning(|_| Ok(()));
        adapters
            .notifier_mut()
            .expect_notify()
            .returning(|_| Ok(()));
        let use_case = NewsUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .create(
                &ctx,
                CreateNewsCommand {
                    title: fixture::news::title1().value(),
                    body: fixture::news::body1().value(),
                    attachments: fixture::news::attachments1()
                        .into_iter()
                        .map(|id| id.value().to_string())
                        .collect(),
                    categories: ProjectCategoriesDto::from(fixture::news::categories1()),
                    attributes: ProjectAttributesDto::from(fixture::news::attributes1()),
                },
            )
            .await;
        assert!(res.is_ok());
    }
}
