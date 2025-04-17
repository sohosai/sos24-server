use sos24_domain::{
    entity::{
        common::datetime::DateTime,
        news::{News, NewsState},
    },
    repository::{news::NewsRepository, project::ProjectRepository, Repositories},
};

use crate::{
    news::{NewsUseCase, NewsUseCaseError},
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

impl<R: Repositories, A: Adapters> NewsUseCase<R, A> {
    pub async fn check_news_and_send_notify(
        &self,
        ctx: &impl ContextProvider,
    ) -> Result<(), NewsUseCaseError> {
        let news_list = self.repositories.news_repository().list().await?;
        let news_list_to_notify = news_list.into_iter().filter(|news| match news.state() {
            NewsState::Scheduled(date) => &date.clone().value() <= ctx.requested_at(),
            _ => false,
        });

        let project_list = self.repositories.project_repository().list().await?;
        for news in news_list_to_notify {
            let target_project_list = project_list
                .iter()
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
                    app_url::committee_news(ctx, news.id().clone()),
                ))
                .await?;

            // 公開済みに更新（公開時刻も更新）
            let new_news = News::new(
                news.id().clone(),
                NewsState::Published,
                news.title().clone(),
                news.body().clone(),
                news.attachments().clone(),
                news.categories().clone(),
                news.attributes().clone(),
                DateTime::new(ctx.requested_at().clone()),
                DateTime::new(ctx.requested_at().clone()),
            );
            self.repositories.news_repository().update(new_news).await?;
        }

        Ok(())
    }
}
