use chrono_tz::Asia::Tokyo;
use sos24_domain::repository::{form::FormRepository, project::ProjectRepository, Repositories};

use crate::{
    form::{FormUseCase, FormUseCaseError},
    shared::{
        adapter::{
            email::{Email, EmailSender, SendEmailCommand},
            Adapters,
        },
        app_url,
        context::ContextProvider,
    },
};

impl<R: Repositories, A: Adapters> FormUseCase<R, A> {
    pub async fn check_form_and_send_notify(
        &self,
        ctx: &impl ContextProvider,
    ) -> Result<(), FormUseCaseError> {
        let form_list = self.repositories.form_repository().list().await?;
        let form_list_to_notify = form_list
            .into_iter()
            .filter(|form| !form.is_draft().clone().value())
            .filter(|form| !form.is_notified().clone().value())
            .filter(|form| form.is_started(ctx.requested_at()));

        let project_list = self.repositories.project_repository().list().await?;
        for form in form_list_to_notify {
            let target_project_list = project_list
                .iter()
                .filter(|project_with_owners| form.is_sent_to(&project_with_owners.project));

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
                    "申請「{title}」が公開されました",
                    title = form.title().clone().value()
                ),
                body: format!(
                    r#"雙峰祭オンラインシステムで申請が公開されました。

タイトル: {title}
回答開始時刻: {starts_at}
回答終了時刻: {ends_at}

詳細は以下のリンクから確認できます。
{url}

※このメールは雙峰祭オンラインシステムが自動送信しています。
＿＿＿
筑波大学学園祭実行委員会
Email : {email}
電話 : 029-853-2899"#,
                    title = form.title().clone().value(),
                    starts_at = form
                        .starts_at()
                        .clone()
                        .value()
                        .with_timezone(&Tokyo)
                        .format("%Y年%m月%d日 %H:%M"),
                    ends_at = form
                        .ends_at()
                        .clone()
                        .value()
                        .with_timezone(&Tokyo)
                        .format("%Y年%m月%d日 %H:%M"),
                    url = app_url::form(ctx, form.id().clone()),
                    email = ctx.config().email_reply_to_address.clone(),
                ),
            };
            self.adapters.email_sender().send_email(command).await?;

            let mut new_form = form;
            new_form.set_notified()?;
            self.repositories.form_repository().update(new_form).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
