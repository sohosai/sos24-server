use sos24_domain::entity::file_data::FileId;
use sos24_domain::entity::form::Form;
use sos24_domain::entity::form::FormItemId;
use sos24_domain::entity::form_answer::{
    FormAnswer, FormAnswerItem, FormAnswerItemChooseMany, FormAnswerItemChooseOne,
    FormAnswerItemFile, FormAnswerItemInt, FormAnswerItemKind, FormAnswerItemString,
};
use sos24_domain::entity::project::Project;

use super::FormAnswerUseCaseError;

#[derive(Debug)]
pub struct FormAnswerDto {
    pub id: String,
    pub project_id: String,
    pub project_title: String,
    pub form_id: String,
    pub form_title: String,
    pub items: Vec<FormAnswerItemDto>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<(FormAnswer, Project, Form)> for FormAnswerDto {
    fn from((form_answer, project, form): (FormAnswer, Project, Form)) -> Self {
        let form_answer = form_answer.destruct();
        let project = project.destruct();
        let form = form.destruct();

        Self {
            id: form_answer.id.value().to_string(),
            project_id: form_answer.project_id.value().to_string(),
            project_title: project.title.value().to_string(),
            form_id: form_answer.form_id.value().to_string(),
            form_title: form.title.value().to_string(),
            items: form_answer
                .items
                .into_iter()
                .map(FormAnswerItemDto::from)
                .collect(),
            created_at: form_answer.created_at.value(),
            updated_at: form_answer.updated_at.value(),
        }
    }
}

#[derive(Debug)]
pub struct FormAnswerItemDto {
    pub item_id: String,
    pub kind: FormAnswerItemKindDto,
}

impl FormAnswerItemDto {
    pub fn new(item_id: String, kind: FormAnswerItemKindDto) -> Self {
        Self { item_id, kind }
    }
}

impl TryFrom<FormAnswerItemDto> for FormAnswerItem {
    type Error = FormAnswerUseCaseError;
    fn try_from(value: FormAnswerItemDto) -> Result<Self, Self::Error> {
        Ok(FormAnswerItem::new(
            FormItemId::try_from(value.item_id)?,
            FormAnswerItemKind::try_from(value.kind)?,
        ))
    }
}

impl From<FormAnswerItem> for FormAnswerItemDto {
    fn from(entity: FormAnswerItem) -> Self {
        let entity = entity.destruct();
        Self::new(
            entity.item_id.value().to_string(),
            FormAnswerItemKindDto::from(entity.kind),
        )
    }
}

#[derive(Debug)]
pub enum FormAnswerItemKindDto {
    String(String),
    Int(i32),
    ChooseOne(String),
    ChooseMany(Vec<String>),
    File(Vec<String>),
}

impl TryFrom<FormAnswerItemKindDto> for FormAnswerItemKind {
    type Error = FormAnswerUseCaseError;
    fn try_from(value: FormAnswerItemKindDto) -> Result<Self, Self::Error> {
        match value {
            FormAnswerItemKindDto::String(value) => {
                Ok(FormAnswerItemKind::String(FormAnswerItemString::new(value)))
            }
            FormAnswerItemKindDto::Int(value) => {
                Ok(FormAnswerItemKind::Int(FormAnswerItemInt::new(value)))
            }
            FormAnswerItemKindDto::ChooseOne(value) => Ok(FormAnswerItemKind::ChooseOne(
                FormAnswerItemChooseOne::new(value),
            )),
            FormAnswerItemKindDto::ChooseMany(value) => Ok(FormAnswerItemKind::ChooseMany(
                FormAnswerItemChooseMany::new(value),
            )),
            FormAnswerItemKindDto::File(value) => {
                Ok(FormAnswerItemKind::File(FormAnswerItemFile::new(
                    value
                        .into_iter()
                        .map(FileId::try_from)
                        .collect::<Result<_, _>>()?,
                )))
            }
        }
    }
}

impl From<FormAnswerItemKind> for FormAnswerItemKindDto {
    fn from(entity: FormAnswerItemKind) -> Self {
        match entity {
            FormAnswerItemKind::String(value) => {
                FormAnswerItemKindDto::String(value.value().to_string())
            }
            FormAnswerItemKind::Int(value) => FormAnswerItemKindDto::Int(value.value()),
            FormAnswerItemKind::ChooseOne(value) => {
                FormAnswerItemKindDto::ChooseOne(value.value().to_string())
            }
            FormAnswerItemKind::ChooseMany(value) => {
                FormAnswerItemKindDto::ChooseMany(value.value().to_vec())
            }
            FormAnswerItemKind::File(value) => FormAnswerItemKindDto::File(
                value
                    .value()
                    .into_iter()
                    .map(|id| id.value().to_string())
                    .collect(),
            ),
        }
    }
}

#[derive(Debug)]
pub struct FormAnswerToBeExportedListDto {
    pub form_title: String,
    pub form_item_names: Vec<String>,
    pub form_answers: Vec<FormAnswerToBeExportedDto>,
}

#[derive(Debug)]
pub struct FormAnswerToBeExportedDto {
    pub project_index: i32,
    pub project_title: String,
    pub project_group_name: String,
    pub form_answer_item_values: Option<Vec<String>>,
    pub created_at: Option<String>,
}
