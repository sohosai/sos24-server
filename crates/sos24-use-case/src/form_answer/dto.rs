use sos24_domain::entity::file_data::FileId;
use sos24_domain::entity::form::{Form, FormItemIdError};
use sos24_domain::entity::project::Project;
use sos24_domain::entity::{
    common::date::WithDate,
    form::FormItemId,
    form_answer::{
        FormAnswer, FormAnswerItem, FormAnswerItemChooseMany, FormAnswerItemChooseOne,
        FormAnswerItemFile, FormAnswerItemInt, FormAnswerItemKind, FormAnswerItemString,
    },
};

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
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<(WithDate<FormAnswer>, WithDate<Project>, WithDate<Form>)> for FormAnswerDto {
    fn from(
        (form_answer_entity, project_entity, form_entity): (
            WithDate<FormAnswer>,
            WithDate<Project>,
            WithDate<Form>,
        ),
    ) -> Self {
        let form_answer = form_answer_entity.value.destruct();
        let project = project_entity.value.destruct();
        let form = form_entity.value.destruct();

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
            created_at: form_answer_entity.created_at,
            updated_at: form_answer_entity.updated_at,
            deleted_at: form_answer_entity.deleted_at,
        }
    }
}

#[derive(Debug)]
pub struct FormAnswerItemDto {
    pub item_id: String,
    pub kind: FormAnswerItemKindDto,
}

impl TryFrom<FormAnswerItemDto> for FormAnswerItem {
    type Error = FormItemIdError;
    fn try_from(dto: FormAnswerItemDto) -> Result<Self, Self::Error> {
        Ok(FormAnswerItem::new(
            FormItemId::try_from(dto.item_id)?,
            dto.kind.into(),
        ))
    }
}

impl From<FormAnswerItem> for FormAnswerItemDto {
    fn from(entity: FormAnswerItem) -> Self {
        let entity = entity.destruct();
        FormAnswerItemDto {
            item_id: entity.item_id.value().to_string(),
            kind: FormAnswerItemKindDto::from(entity.kind),
        }
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

impl From<FormAnswerItemKindDto> for FormAnswerItemKind {
    fn from(dto: FormAnswerItemKindDto) -> Self {
        match dto {
            FormAnswerItemKindDto::String(value) => {
                FormAnswerItemKind::String(FormAnswerItemString::new(value))
            }
            FormAnswerItemKindDto::Int(value) => {
                FormAnswerItemKind::Int(FormAnswerItemInt::new(value))
            }
            FormAnswerItemKindDto::ChooseOne(value) => {
                FormAnswerItemKind::ChooseOne(FormAnswerItemChooseOne::new(value))
            }
            FormAnswerItemKindDto::ChooseMany(value) => {
                FormAnswerItemKind::ChooseMany(FormAnswerItemChooseMany::new(value))
            }
            FormAnswerItemKindDto::File(value) => {
                FormAnswerItemKind::File(FormAnswerItemFile::new(
                    value
                        .into_iter()
                        .map(FileId::try_from)
                        .collect::<Result<_, _>>()
                        .unwrap(),
                ))
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
