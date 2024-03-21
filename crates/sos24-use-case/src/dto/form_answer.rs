use sos24_domain::entity::{
    form::{FormId, FormItemId},
    form_answer::{
        FormAnswer, FormAnswerItem, FormAnswerItemChooseMany, FormAnswerItemChooseOne,
        FormAnswerItemFile, FormAnswerItemInt, FormAnswerItemKind, FormAnswerItemString,
    },
    project::ProjectId,
};

use crate::interactor::form::FormUseCaseError;

use super::ToEntity;

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

#[derive(Debug)]
pub struct FormAnswerItemDto {
    item_id: String,
    kind: FormAnswerItemKindDto,
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
