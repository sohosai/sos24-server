use sos24_domain::entity::{
    common::date::WithDate,
    form::{FormId, FormItemId},
    form_answer::{
        FormAnswer, FormAnswerItem, FormAnswerItemChooseMany, FormAnswerItemChooseOne,
        FormAnswerItemFile, FormAnswerItemInt, FormAnswerItemKind, FormAnswerItemString,
    },
    project::ProjectId,
};

use crate::interactor::form::FormUseCaseError;

use super::{FromEntity, ToEntity};

#[derive(Debug)]
pub struct CreateFormAnswerDto {
    project_id: String,
    form_id: String,
    items: Vec<FormAnswerItemDto>,
}

impl CreateFormAnswerDto {
    pub fn new(project_id: String, form_id: String, items: Vec<FormAnswerItemDto>) -> Self {
        Self {
            project_id,
            form_id,
            items,
        }
    }
}

impl ToEntity for CreateFormAnswerDto {
    type Entity = FormAnswer;
    type Error = FormUseCaseError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        Ok(FormAnswer::create(
            ProjectId::try_from(self.project_id)?,
            FormId::try_from(self.form_id)?,
            self.items
                .into_iter()
                .map(FormAnswerItemDto::into_entity)
                .collect::<Result<_, _>>()?,
        ))
    }
}

pub struct UpdateFormAnswerDto {
    pub id: String,
    pub project_id: String,
    pub form_id: String,
    pub items: Vec<FormAnswerItemDto>,
}

impl UpdateFormAnswerDto {
    pub fn new(
        id: String,
        project_id: String,
        form_id: String,
        items: Vec<FormAnswerItemDto>,
    ) -> Self {
        Self {
            id,
            project_id,
            form_id,
            items,
        }
    }
}

#[derive(Debug)]
pub struct FormAnswerDto {
    pub id: String,
    pub project_id: String,
    pub form_id: String,
    pub items: Vec<FormAnswerItemDto>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl FromEntity for FormAnswerDto {
    type Entity = WithDate<FormAnswer>;
    fn from_entity(entity: Self::Entity) -> Self {
        let form_answer = entity.value.destruct();
        Self {
            id: form_answer.id.value().to_string(),
            project_id: form_answer.project_id.value().to_string(),
            form_id: form_answer.form_id.value().to_string(),
            items: form_answer
                .items
                .into_iter()
                .map(FormAnswerItemDto::from_entity)
                .collect(),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
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

impl ToEntity for FormAnswerItemDto {
    type Entity = FormAnswerItem;
    type Error = FormUseCaseError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        Ok(FormAnswerItem::new(
            FormItemId::try_from(self.item_id)?,
            self.kind.into_entity()?,
        ))
    }
}

impl FromEntity for FormAnswerItemDto {
    type Entity = FormAnswerItem;
    fn from_entity(entity: Self::Entity) -> Self {
        let entity = entity.destruct();
        Self::new(
            entity.item_id.value().to_string(),
            FormAnswerItemKindDto::from_entity(entity.kind),
        )
    }
}

#[derive(Debug)]
pub enum FormAnswerItemKindDto {
    String(String),
    Int(i32),
    ChooseOne(String),
    ChooseMany(Vec<String>),
    File(String),
}

impl ToEntity for FormAnswerItemKindDto {
    type Entity = FormAnswerItemKind;
    type Error = FormUseCaseError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        match self {
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
                Ok(FormAnswerItemKind::File(FormAnswerItemFile::new(value)))
            }
        }
    }
}

impl FromEntity for FormAnswerItemKindDto {
    type Entity = FormAnswerItemKind;
    fn from_entity(entity: Self::Entity) -> Self {
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
            FormAnswerItemKind::File(value) => {
                FormAnswerItemKindDto::File(value.value().to_string())
            }
        }
    }
}
