use chrono_tz::Asia::Tokyo;
use sos24_domain::repository::{
    form::FormRepository, project::ProjectRepository, user::UserRepository, Repositories,
};

use crate::adapter::{
    email::{Email, EmailSender, SendEmailCommand},
    Adapters,
};

use super::{FormUseCase, FormUseCaseError};

impl<R: Repositories, A: Adapters> FormUseCase<R, A> {
    pub async fn check_form_and_send_notify(
        &self,
        now: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), FormUseCaseError> {
        let form_list = self.repositories.form_repository().list().await?;
        let form_list_to_notify = form_list
            .into_iter()
            .filter(|form| !form.value.is_notified().clone().value())
            .filter(|form| form.value.starts_at().clone().value() <= now);

        let project_list = self.repositories.project_repository().list().await?;
        for form in form_list_to_notify {
            let form = form.value;
            let target_project_list = project_list
                .iter()
                .filter(|project| form.is_sent_to(&project.value));

            let mut emails = Vec::new();
            for project in target_project_list {
                let owner_id = project.value.owner_id().clone();
                let owner = self
                    .repositories
                    .user_repository()
                    .find_by_id(owner_id.clone())
                    .await?
                    .ok_or(FormUseCaseError::UserNotFound(owner_id))?;
                emails.push(owner.value.email().clone().value());

                if let Some(sub_owner_id) = project.value.sub_owner_id().clone() {
                    let sub_owner = self
                        .repositories
                        .user_repository()
                        .find_by_id(sub_owner_id.clone())
                        .await?
                        .ok_or(FormUseCaseError::UserNotFound(sub_owner_id))?;
                    emails.push(sub_owner.value.email().clone().value());
                }
            }

            let command = SendEmailCommand {
                from: Email {
                    address: String::from("system@sohosai.com"),
                    name: String::from("雙峰祭オンラインシステム"),
                },
                to: emails,
                reply_to: Some(String::from("project50th@sohosai.com")),
                subject: format!(
                    "申請「{title}」が公開されました - 雙峰祭オンラインシステム",
                    title = form.title().clone().value()
                ),
                body: format!(
                    r#"雙峰祭オンラインシステムで申請が公開されました。

タイトル: {title}
回答開始時刻: {starts_at}
回答終了時刻: {ends_at}

{url}

※このメールは雙峰祭オンラインシステムが自動送信しています。
※配信停止は以下のリンクからお手続きください。
{optout_url}"#,
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
                    url = format!(
                        "https://entry.sohosai.com/forms/{}",
                        form.id().clone().value()
                    ),
                    optout_url = self.adapters.email_sender().opt_out_url(),
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
