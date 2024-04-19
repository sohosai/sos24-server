use chrono_tz::Asia::Tokyo;

use sos24_domain::entity::form::FormId;
use sos24_domain::entity::form_answer::{FormAnswerItem, FormAnswerItemKind};
use sos24_domain::repository::form::FormRepository;
use sos24_domain::repository::project::ProjectRepository;
use sos24_domain::{
    ensure,
    entity::permission::Permissions,
    repository::{form_answer::FormAnswerRepository, Repositories},
};

use crate::context::ContextProvider;
use crate::dto::form_answer::{FormAnswerToBeExportedDto, FormAnswerToBeExportedListDto};

use super::{FormAnswerUseCase, FormAnswerUseCaseError};

impl<R: Repositories> FormAnswerUseCase<R> {
    pub async fn export_by_form_id(
        &self,
        ctx: &impl ContextProvider,
        form_id: String,
    ) -> Result<FormAnswerToBeExportedListDto, FormAnswerUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
        ensure!(actor.has_permission(Permissions::READ_FORM_ANSWER_ALL));

        let form_id = FormId::try_from(form_id.clone())?;
        let raw_form = self
            .repositories
            .form_repository()
            .find_by_id(form_id.clone())
            .await?
            .ok_or(FormAnswerUseCaseError::FormNotFound(form_id.clone()))?;

        let form = raw_form.value.destruct();
        let form_title = form.title.value();
        let (form_item_ids, form_item_names): (Vec<_>, Vec<_>) = form
            .items
            .into_iter()
            .map(|item| (item.id().clone(), item.name().clone().value()))
            .unzip();

        let raw_form_answer_list = self.repositories.form_answer_repository().list().await?;

        let mut form_answers = Vec::new();
        for raw_form_answer in raw_form_answer_list {
            let project_id = raw_form_answer.value.project_id();
            let raw_project = self
                .repositories
                .project_repository()
                .find_by_id(project_id.clone())
                .await?
                .ok_or(FormAnswerUseCaseError::ProjectNotFound(project_id.clone()))?;
            let project = raw_project.value.destruct();

            let form_answer_item_values = form_item_ids
                .iter()
                .map(|item_id| {
                    raw_form_answer
                        .value
                        .items()
                        .iter()
                        .find(|item| item.item_id() == item_id)
                        .map(convert_answer_item_to_string)
                })
                .collect();

            form_answers.push(FormAnswerToBeExportedDto {
                project_index: project.index.value(),
                project_title: project.title.value().to_string(),
                project_group_name: project.group_name.value().to_string(),
                form_answer_item_values,
                created_at: raw_form_answer
                    .created_at
                    .with_timezone(&Tokyo)
                    .to_rfc3339(),
            });
        }

        Ok(FormAnswerToBeExportedListDto {
            form_title,
            form_item_names,
            form_answers,
        })
    }
}

fn convert_answer_item_to_string(item: &FormAnswerItem) -> String {
    match item.kind() {
        FormAnswerItemKind::String(value) => value.clone().value().to_string(),
        FormAnswerItemKind::Int(value) => value.clone().value().to_string(),
        FormAnswerItemKind::ChooseOne(value) => value.clone().value().to_string(),
        FormAnswerItemKind::ChooseMany(value) => value
            .clone()
            .value()
            .iter()
            .map(|it| it.to_string())
            .collect::<Vec<_>>()
            .join(";"),
        // TODO: ファイルのリネームを実装した段階で書き換え
        FormAnswerItemKind::File(value) => value
            .clone()
            .value()
            .into_iter()
            .map(|it| it.value().to_string())
            .collect::<Vec<_>>()
            .join(";"),
    }
}

#[cfg(test)]
mod tests {}
