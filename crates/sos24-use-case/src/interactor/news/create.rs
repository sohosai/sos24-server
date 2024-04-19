use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::permission::Permissions,
    repository::{
        file_data::FileDataRepository, news::NewsRepository, project::ProjectRepository,
        user::UserRepository, Repositories,
    },
};

use crate::{
    adapter::{
        email::{Email, EmailSender, SendEmailCommand},
        Adapters,
    },
    context::ContextProvider,
    dto::{news::CreateNewsDto, ToEntity},
};

use super::{NewsUseCase, NewsUseCaseError};

impl<R: Repositories, A: Adapters> NewsUseCase<R, A> {
    pub async fn create(
        &self,
        ctx: &impl ContextProvider,
        raw_news: CreateNewsDto,
    ) -> Result<String, NewsUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::CREATE_NEWS));

        let news = raw_news.into_entity()?;
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
            .filter(|project| news.is_sent_to(&project.value));

        let mut emails = Vec::new();
        for project in target_project_list {
            let owner_id = project.value.owner_id().clone();
            let owner = self
                .repositories
                .user_repository()
                .find_by_id(owner_id.clone())
                .await?
                .ok_or(NewsUseCaseError::UserNotFound(owner_id))?;
            emails.push(owner.value.email().clone().value());

            if let Some(sub_owner_id) = project.value.sub_owner_id().clone() {
                let sub_owner = self
                    .repositories
                    .user_repository()
                    .find_by_id(sub_owner_id.clone())
                    .await?
                    .ok_or(NewsUseCaseError::UserNotFound(sub_owner_id))?;
                emails.push(sub_owner.value.email().clone().value());
            }
        }

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
※配信停止は以下のリンクからお手続きください。
{optout_url}"#,
                title = news.title().clone().value(),
                body = news.body().clone().value(),
                url = format!(
                    "{}/news/{}",
                    ctx.config().app_url,
                    news.id().clone().value()
                ),
                optout_url = self.adapters.email_sender().opt_out_url(),
            ),
        };
        self.adapters.email_sender().send_email(command).await?;

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
        adapter::MockAdapters,
        context::TestContext,
        dto::{news::CreateNewsDto, FromEntity},
        interactor::news::{NewsUseCase, NewsUseCaseError},
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
                CreateNewsDto::new(
                    fixture::news::title1().value(),
                    fixture::news::body1().value(),
                    fixture::news::attachments1()
                        .into_iter()
                        .map(|id| id.value().to_string())
                        .collect(),
                    Vec::from_entity(fixture::news::categories1()),
                    Vec::from_entity(fixture::news::attributes1()),
                ),
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
            .expect_opt_out_url()
            .returning(|| String::new());
        adapters
            .email_sender_mut()
            .expect_send_email()
            .returning(|_| Ok(()));
        let use_case = NewsUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .create(
                &ctx,
                CreateNewsDto::new(
                    fixture::news::title1().value(),
                    fixture::news::body1().value(),
                    fixture::news::attachments1()
                        .into_iter()
                        .map(|id| id.value().to_string())
                        .collect(),
                    Vec::from_entity(fixture::news::categories1()),
                    Vec::from_entity(fixture::news::attributes1()),
                ),
            )
            .await;
        assert!(res.is_ok());
    }
}
