use chrono_tz::Asia::Tokyo;

use sos24_domain::entity::form::{Form, FormId, FormItemKind};
use sos24_domain::entity::form_answer::{FormAnswer, FormAnswerItemKind};
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

        let header = export_header(&form);

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
                    let form_answer_item_values = export_record(&form, &form_answer)?;
                    let created_at = form_answer
                        .created_at()
                        .clone()
                        .value()
                        .with_timezone(&Tokyo)
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string();
                    (Some(form_answer_item_values), Some(created_at))
                }
                None => (None, None),
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

        Ok(dbg!(FormAnswerToBeExportedListDto {
            form_title: form.title().clone().value(),
            form_item_names: header,
            form_answers,
        }))
    }
}

fn export_header(form: &Form) -> Vec<String> {
    let mut record = vec![];
    for form_item in form.items() {
        let form_item_name = form_item.name().clone().value();
        match form_item.kind() {
            FormItemKind::String(_)
            | FormItemKind::Int(_)
            | FormItemKind::ChooseOne(_)
            | FormItemKind::File(_) => record.push(form_item_name),
            FormItemKind::ChooseMany(choose_many) => {
                for options in choose_many.options() {
                    record.push(format!("{} {}", form_item_name, options.clone().value()));
                }
            }
        }
    }
    record
}

fn export_record(
    form: &Form,
    form_answer: &FormAnswer,
) -> Result<Vec<String>, FormAnswerUseCaseError> {
    let mut record = vec![];
    for form_item in form.items() {
        let form_item_kind = form_item.kind();
        let form_answer_item_kind = form_answer
            .items()
            .iter()
            .find(|form_answer_item| form_answer_item.item_id() == form_item.id())
            .map(|it| it.kind());

        match (form_item_kind, form_answer_item_kind) {
            (FormItemKind::String(_), None) => record.push(String::new()),
            (FormItemKind::String(_), Some(FormAnswerItemKind::String(value))) => {
                record.push(value.clone().value().to_string());
            }
            (FormItemKind::Int(_), None) => record.push(String::new()),
            (FormItemKind::Int(_), Some(FormAnswerItemKind::Int(value))) => {
                record.push(value.clone().value().to_string());
            }
            (FormItemKind::ChooseOne(_), None) => record.push(String::new()),
            (FormItemKind::ChooseOne(_), Some(FormAnswerItemKind::ChooseOne(value))) => {
                record.push(value.clone().value().to_string());
            }
            (FormItemKind::ChooseMany(choose_many), None) => {
                for _ in choose_many.options() {
                    record.push(String::new());
                }
            }
            (
                FormItemKind::ChooseMany(choose_many),
                Some(FormAnswerItemKind::ChooseMany(value)),
            ) => {
                let chosen_options = value.clone().value();
                for option in choose_many.options() {
                    record.push(chosen_options.contains(&option.clone().value()).to_string());
                }
            }
            (FormItemKind::File(_), None) => record.push(String::new()),
            (FormItemKind::File(_), Some(FormAnswerItemKind::File(value))) => {
                let files = value
                    .clone()
                    .value()
                    .into_iter()
                    .map(|it| it.value().to_string())
                    .collect::<Vec<_>>()
                    .join(";");
                record.push(files);
            }
            _ => {
                tracing::error!(
                    "Export failed: form_item_kind: {:?}, form_answer_item_kind: {:?}",
                    form_item_kind,
                    form_answer_item_kind
                );
                return Err(FormAnswerUseCaseError::ExportFailed);
            }
        }
    }
    Ok(record)
}

#[cfg(test)]
mod tests {}
