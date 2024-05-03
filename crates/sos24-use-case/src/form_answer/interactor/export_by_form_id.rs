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

use crate::form_answer::dto::{FormAnswerToBeExportedDto, FormAnswerToBeExportedListDto};
use crate::form_answer::{FormAnswerUseCase, FormAnswerUseCaseError};
use crate::shared::context::ContextProvider;

impl<R: Repositories> FormAnswerUseCase<R> {
    pub async fn export_by_form_id(
        &self,
        ctx: &impl ContextProvider,
        form_id: String,
    ) -> Result<FormAnswerToBeExportedListDto, FormAnswerUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
        ensure!(actor.has_permission(Permissions::READ_FORM_ANSWER_ALL));

        let form_id = FormId::try_from(form_id.clone())?;
        let form = self
            .repositories
            .form_repository()
            .find_by_id(form_id.clone())
            .await?
            .ok_or(FormAnswerUseCaseError::FormNotFound(form_id.clone()))?;

        let project_list = self.repositories.project_repository().list().await?;
        let target_project_list: Vec<_> = project_list
            .into_iter()
            .filter(|project_with_owners| form.is_sent_to(&project_with_owners.project))
            .collect();

        let form = form.destruct();
        let form_title = form.title.value();
        let (form_item_ids, form_item_names): (Vec<_>, Vec<_>) = form
            .items
            .into_iter()
            .map(|item| (item.id().clone(), item.name().clone().value()))
            .unzip();

        let mut form_answers = Vec::new();
        for project_with_owner in target_project_list {
            let project_id = project_with_owner.project.id().clone();
            let form_answer = self
                .repositories
                .form_answer_repository()
                .find_by_project_id_and_form_id(project_id, form_id.clone())
                .await?;

            let (form_answer_item_values, created_at) = match form_answer {
                Some(form_answer) => {
                    let form_answer = form_answer.destruct();
                    let values = form_item_ids
                        .iter()
                        .map(|item_id| {
                            form_answer
                                .items
                                .iter()
                                .find(|item| item.item_id() == item_id)
                                .map(convert_answer_item_to_string)
                        })
                        .collect();
                    let created_at = form_answer
                        .created_at
                        .value()
                        .with_timezone(&Tokyo)
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string();
                    (values, Some(created_at))
                }
                None => (form_item_ids.iter().map(|_| None).collect(), None),
            };

            let project = project_with_owner.project.destruct();
            form_answers.push(FormAnswerToBeExportedDto {
                project_index: project.index.value(),
                project_title: project.title.value().to_string(),
                project_group_name: project.group_name.value().to_string(),
                form_answer_item_values,
                created_at,
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
