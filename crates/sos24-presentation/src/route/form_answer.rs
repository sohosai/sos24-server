use serde::{Deserialize, Serialize};
use sos24_use_case::form_answer::dto::{FormAnswerDto, FormAnswerItemDto, FormAnswerItemKindDto};

pub mod export;
pub mod get;
pub mod get_by_id;
pub mod post;
pub mod put_by_id;

#[derive(Debug, Serialize, Deserialize)]
pub struct FormAnswer {
    id: String,
    project_id: String,
    project_title: String,
    form_id: String,
    form_title: String,
    items: Vec<FormAnswerItem>,
    created_at: String,
    updated_at: String,
    deleted_at: Option<String>,
}

impl From<FormAnswerDto> for FormAnswer {
    fn from(form_answer_dto: FormAnswerDto) -> Self {
        FormAnswer {
            id: form_answer_dto.id,
            project_id: form_answer_dto.project_id,
            project_title: form_answer_dto.project_title,
            form_id: form_answer_dto.form_id,
            form_title: form_answer_dto.form_title,
            items: form_answer_dto
                .items
                .into_iter()
                .map(FormAnswerItem::from)
                .collect(),
            created_at: form_answer_dto.created_at.to_rfc3339(),
            updated_at: form_answer_dto.updated_at.to_rfc3339(),
            deleted_at: form_answer_dto.deleted_at.map(|it| it.to_rfc3339()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FormAnswerItem {
    String { item_id: String, value: String },
    Int { item_id: String, value: i32 },
    ChooseOne { item_id: String, value: String },
    ChooseMany { item_id: String, value: Vec<String> },
    File { item_id: String, value: Vec<String> },
}

impl From<FormAnswerItem> for FormAnswerItemDto {
    fn from(form_answer_item: FormAnswerItem) -> Self {
        match form_answer_item {
            FormAnswerItem::String { item_id, value } => FormAnswerItemDto {
                item_id,
                kind: FormAnswerItemKindDto::String(value),
            },
            FormAnswerItem::Int { item_id, value } => FormAnswerItemDto {
                item_id,
                kind: FormAnswerItemKindDto::Int(value),
            },
            FormAnswerItem::ChooseOne { item_id, value } => FormAnswerItemDto {
                item_id,
                kind: FormAnswerItemKindDto::ChooseOne(value),
            },
            FormAnswerItem::ChooseMany { item_id, value } => FormAnswerItemDto {
                item_id,
                kind: FormAnswerItemKindDto::ChooseMany(value),
            },
            FormAnswerItem::File { item_id, value } => FormAnswerItemDto {
                item_id,
                kind: FormAnswerItemKindDto::File(value),
            },
        }
    }
}

impl From<FormAnswerItemDto> for FormAnswerItem {
    fn from(form_answer_item_dto: FormAnswerItemDto) -> Self {
        let FormAnswerItemDto { item_id, kind } = form_answer_item_dto;
        match kind {
            FormAnswerItemKindDto::String(value) => FormAnswerItem::String { item_id, value },
            FormAnswerItemKindDto::Int(value) => FormAnswerItem::Int { item_id, value },
            FormAnswerItemKindDto::ChooseOne(value) => FormAnswerItem::ChooseOne { item_id, value },
            FormAnswerItemKindDto::ChooseMany(value) => {
                FormAnswerItem::ChooseMany { item_id, value }
            }
            FormAnswerItemKindDto::File(value) => FormAnswerItem::File { item_id, value },
        }
    }
}
