use anyhow::anyhow;
use sos24_domain::{
    ensure,
    entity::{
        common::datetime::DateTime,
        file_data::FileId,
        news::{News, NewsBody, NewsState, NewsTitle},
        permission::Permissions,
        project::{ProjectAttributes, ProjectCategories},
    },
    repository::{
        file_data::FileDataRepository, news::NewsRepository, project::ProjectRepository,
        Repositories,
    },
};

use crate::{
    news::{dto::NewsStateDto, NewsUseCase, NewsUseCaseError},
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
    pub state: NewsStateDto,
    pub title: String,
    pub body: String,
    pub attachments: Vec<String>,
    pub categories: ProjectCategoriesDto,
    pub attributes: ProjectAttributesDto,
    pub scheduled_at: Option<String>,
}

impl CreateNewsCommand {
    pub fn get_news_state(&self) -> Result<NewsState, NewsUseCaseError> {
        match &self.state {
            NewsStateDto::Draft => Ok(NewsState::Draft),
            NewsStateDto::Scheduled => match &self.scheduled_at {
                Some(date) => Ok(NewsState::Scheduled(DateTime::try_from(date.clone())?)),
                None => Err(NewsUseCaseError::InternalError(anyhow!(
                    "Invalid newsstate format"
                ))),
            },
            NewsStateDto::Published => Ok(NewsState::Published),
        }
    }
}

impl<R: Repositories, A: Adapters> NewsUseCase<R, A> {
    pub async fn create(
        &self,
        ctx: &impl ContextProvider,
        raw_news: CreateNewsCommand,
    ) -> Result<String, NewsUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
        match raw_news.state {
            NewsStateDto::Draft => ensure!(actor.has_permission(Permissions::CREATE_DRAFT_NEWS)),
            NewsStateDto::Scheduled => {
                ensure!(actor.has_permission(Permissions::CREATE_SCHEDULED_NEWS))
            }
            NewsStateDto::Published => ensure!(actor.has_permission(Permissions::CREATE_NEWS)),
        }

        let news = News::create(
            raw_news.get_news_state()?,
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
        news::{
            dto::NewsStateDto, interactor::create::CreateNewsCommand, NewsUseCase, NewsUseCaseError,
        },
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

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeViewer));
        let (state, scheduled_at) = NewsStateDto::from_news_state(fixture::news::state1());
        let res = use_case
            .create(
                &ctx,
                CreateNewsCommand {
                    state,
                    title: fixture::news::title1().value(),
                    body: fixture::news::body1().value(),
                    attachments: fixture::news::attachments1()
                        .into_iter()
                        .map(|id| id.value().to_string())
                        .collect(),
                    categories: ProjectCategoriesDto::from(fixture::news::categories1()),
                    attributes: ProjectAttributesDto::from(fixture::news::attributes1()),
                    scheduled_at,
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
    async fn 実委人編集者は予約投稿のお知らせを作成できる() {
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

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeEditor));
        let (state, scheduled_at) = NewsStateDto::from_news_state(fixture::news::state2());
        let res = use_case
            .create(
                &ctx,
                CreateNewsCommand {
                    state,
                    title: fixture::news::title1().value(),
                    body: fixture::news::body1().value(),
                    attachments: fixture::news::attachments1()
                        .into_iter()
                        .map(|id| id.value().to_string())
                        .collect(),
                    categories: ProjectCategoriesDto::from(fixture::news::categories1()),
                    attributes: ProjectAttributesDto::from(fixture::news::attributes1()),
                    scheduled_at,
                },
            )
            .await;
        assert!(res.is_ok());
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
        let (state, scheduled_at) = NewsStateDto::from_news_state(fixture::news::state1());
        let res = use_case
            .create(
                &ctx,
                CreateNewsCommand {
                    state,
                    title: fixture::news::title1().value(),
                    body: fixture::news::body1().value(),
                    attachments: fixture::news::attachments1()
                        .into_iter()
                        .map(|id| id.value().to_string())
                        .collect(),
                    categories: ProjectCategoriesDto::from(fixture::news::categories1()),
                    attributes: ProjectAttributesDto::from(fixture::news::attributes1()),
                    scheduled_at,
                },
            )
            .await;
        assert!(res.is_ok());
    }
}
