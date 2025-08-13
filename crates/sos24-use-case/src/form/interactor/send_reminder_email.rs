use sos24_domain::{
    entity::form::FormId,
    repository::{
        form::FormRepository, form_answer::FormAnswerRepository, project::ProjectRepository,
        Repositories,
    },
};

use crate::{
    form::{FormUseCase, FormUseCaseError},
    shared::{
        adapter::{
            email::{Email, EmailSender, SendEmailCommand},
            Adapters,
        },
        context::ContextProvider,
    },
};

#[derive(Debug, Clone)]
pub struct SendReminderEmailCommand {
    pub form_id: String,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct SendReminderEmailResult {
    pub sent_count: u32,
    pub emails: Vec<String>,
}

impl<R: Repositories, A: Adapters> FormUseCase<R, A> {
    pub async fn send_reminder_email(
        &self,
        ctx: &impl ContextProvider,
        command: SendReminderEmailCommand,
    ) -> Result<SendReminderEmailResult, FormUseCaseError> {
        // フォームIDをFormId型に変換
        let form_id = FormId::try_from(command.form_id)?;

        // フォームの存在確認
        let form = self
            .repositories
            .form_repository()
            .find_by_id(form_id.clone())
            .await?
            .ok_or(FormUseCaseError::NotFound(form_id.clone()))?;

        // フォームが対象とするプロジェクト一覧を取得
        let project_list = self.repositories.project_repository().list().await?;
        let target_projects: Vec<_> = project_list
            .iter()
            .filter(|project_with_owners| form.is_sent_to(&project_with_owners.project))
            .collect();

        // 既に回答したプロジェクトを取得
        let answered_project_ids: std::collections::HashSet<_> = self
            .repositories
            .form_answer_repository()
            .find_by_form_id(form_id)
            .await?
            .into_iter()
            .map(|answer| answer.project_id().clone().value().to_string())
            .collect();

        // 未回答のプロジェクトを抽出
        let unanswered_projects: Vec<_> = target_projects
            .into_iter()
            .filter(|project_with_owners| {
                !answered_project_ids.contains(&project_with_owners.project.id().clone().value().to_string())
            })
            .collect();

        // 未回答プロジェクトの責任者・副責任者のメールアドレスを収集
        let emails: Vec<String> = unanswered_projects
            .iter()
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
            .collect();

        if !emails.is_empty() {
            let email_command = SendEmailCommand {
                from: Email {
                    address: ctx.config().email_sender_address.clone(),
                    name: String::from("雙峰祭オンラインシステム"),
                },
                to: emails.clone(),
                reply_to: Some(ctx.config().email_reply_to_address.clone()),
                subject: command.subject,
                body: command.body,
            };

            self.adapters.email_sender().send_email(email_command).await?;
        }

        Ok(SendReminderEmailResult {
            sent_count: emails.len() as u32,
            emails,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::{
        entity::user::UserRole,
        test::{fixture, repository::MockRepositories},
    };

    use crate::{
        form::{FormUseCase, FormUseCaseError},
        shared::{adapter::MockAdapters, context::TestContext},
    };

    use super::SendReminderEmailCommand;

    #[tokio::test]
    async fn リマインドメールが正常に送信される() {
        let mut repositories = MockRepositories::default();
        let mut adapters = MockAdapters::default();

        // フォーム取得をモック
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::form::form1_opened())));

        // プロジェクト一覧取得をモック  
        repositories
            .project_repository_mut()
            .expect_list()
            .returning(|| {
                // フォームの条件に合うプロジェクトを作成
                // form1_openedは GENERAL カテゴリ + ACADEMIC 属性
                let mut project = fixture::project::project1(fixture::user::user1(UserRole::General).id().clone());
                let actor = fixture::actor::actor1(UserRole::CommitteeOperator);
                project.set_category(&actor, sos24_domain::entity::project::ProjectCategory::General).unwrap();
                project.set_attributes(&actor, sos24_domain::entity::project::ProjectAttributes::ACADEMIC).unwrap();
                
                Ok(vec![sos24_domain::repository::project::ProjectWithOwners {
                    project,
                    owner: fixture::user::user1(UserRole::General),
                    sub_owner: None,
                }])
            });

        // 回答済みプロジェクト取得をモック（空を返す = 未回答）
        repositories
            .form_answer_repository_mut()
            .expect_find_by_form_id()
            .returning(|_| Ok(vec![]));

        // メール送信をモック
        adapters
            .email_sender_mut()
            .expect_send_email()
            .returning(|_| Ok(()));

        let use_case = FormUseCase::new(Arc::new(repositories), Arc::new(adapters));
        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));

        let command = SendReminderEmailCommand {
            form_id: fixture::form::id1().value().to_string(),
            subject: "リマインドメール".to_string(),
            body: "申請の提出をお忘れではありませんか？".to_string(),
        };

        let result = use_case.send_reminder_email(&ctx, command).await;
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert_eq!(result.sent_count, 1); // オーナーの1人分
        assert_eq!(result.emails.len(), 1);
    }

    #[tokio::test]
    async fn 存在しないフォームIDでエラーが返される() {
        let mut repositories = MockRepositories::default();
        let adapters = MockAdapters::default();

        // フォーム取得でNoneを返す
        repositories
            .form_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(None));

        let use_case = FormUseCase::new(Arc::new(repositories), Arc::new(adapters));
        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));

        let command = SendReminderEmailCommand {
            form_id: fixture::form::id1().value().to_string(),
            subject: "リマインドメール".to_string(),
            body: "申請の提出をお忘れではありませんか？".to_string(),
        };

        let result = use_case.send_reminder_email(&ctx, command).await;
        assert!(matches!(result, Err(FormUseCaseError::NotFound(_))));
    }
}