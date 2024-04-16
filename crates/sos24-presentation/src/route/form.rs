use serde::{Deserialize, Serialize};
use sos24_use_case::form::dto::{
    FormDto, FormItemDto, FormItemKindDto, FormSummaryDto, NewFormItemDto,
};

use super::project::{ProjectAttributes, ProjectCategories};

pub mod delete_by_id;
pub mod get;
pub mod get_by_id;
pub mod post;
pub mod put_by_id;

#[derive(Debug, Deserialize)]
pub struct NewFormItem {
    pub name: String,
    pub description: String,
    pub required: bool,
    #[serde(flatten)]
    pub kind: FormItemKind,
}

impl From<NewFormItem> for NewFormItemDto {
    fn from(create_form_item: NewFormItem) -> Self {
        NewFormItemDto {
            name: create_form_item.name,
            description: create_form_item.description,
            required: create_form_item.required,
            kind: FormItemKindDto::from(create_form_item.kind),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Form {
    pub id: String,
    pub title: String,
    pub description: String,
    pub starts_at: String,
    pub ends_at: String,
    pub categories: ProjectCategories,
    pub attributes: ProjectAttributes,
    pub items: Vec<FormItem>,
    pub attachments: Vec<String>,
    pub answer_id: Option<String>,
    pub answered_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl From<FormDto> for Form {
    fn from(form: FormDto) -> Self {
        Form {
            id: form.id.to_string(),
            title: form.title,
            description: form.description,
            starts_at: form.starts_at.to_rfc3339(),
            ends_at: form.ends_at.to_rfc3339(),
            categories: ProjectCategories::from(form.categories),
            attributes: ProjectAttributes::from(form.attributes),
            items: form.items.into_iter().map(FormItem::from).collect(),
            attachments: form.attachments,
            answer_id: form.answer_id.map(|it| it.to_string()),
            answered_at: form.answered_at.map(|it| it.to_rfc3339()),
            created_at: form.created_at.to_rfc3339(),
            updated_at: form.updated_at.to_rfc3339(),
            deleted_at: form.deleted_at.map(|it| it.to_rfc3339()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormSummary {
    pub id: String,
    pub title: String,
    pub description: String,
    pub starts_at: String,
    pub ends_at: String,
    pub categories: ProjectCategories,
    pub attributes: ProjectAttributes,
    pub answer_id: Option<String>,
    pub answered_at: Option<String>,
    pub updated_at: String,
}

impl From<FormSummaryDto> for FormSummary {
    fn from(form: FormSummaryDto) -> Self {
        FormSummary {
            id: form.id.to_string(),
            title: form.title,
            description: form.description,
            starts_at: form.starts_at.to_rfc3339(),
            ends_at: form.ends_at.to_rfc3339(),
            categories: ProjectCategories::from(form.categories),
            attributes: ProjectAttributes::from(form.attributes),
            answer_id: form.answer_id.map(|it| it.to_string()),
            answered_at: form.answered_at.map(|it| it.to_rfc3339()),
            updated_at: form.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub required: bool,
    #[serde(flatten)]
    pub kind: FormItemKind,
}

impl From<FormItemDto> for FormItem {
    fn from(item: FormItemDto) -> Self {
        FormItem {
            id: item.id.to_string(),
            name: item.name,
            description: item.description,
            required: item.required,
            kind: item.kind.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FormItemKind {
    String {
        min_length: Option<i32>,
        max_length: Option<i32>,
        allow_newline: bool,
    },
    Int {
        min: Option<i32>,
        max: Option<i32>,
    },
    ChooseOne {
        options: Vec<String>,
    },
    ChooseMany {
        options: Vec<String>,
        min_selection: Option<i32>,
        max_selection: Option<i32>,
    },
    File {
        extensions: Option<Vec<String>>,
        limit: Option<i32>,
    },
}

impl From<FormItemKind> for FormItemKindDto {
    fn from(kind: FormItemKind) -> Self {
        match kind {
            FormItemKind::String {
                min_length,
                max_length,
                allow_newline,
            } => FormItemKindDto::String {
                min_length,
                max_length,
                allow_newline,
            },
            FormItemKind::Int { min, max } => FormItemKindDto::Int { min, max },
            FormItemKind::ChooseOne { options } => FormItemKindDto::ChooseOne { options },
            FormItemKind::ChooseMany {
                options,
                min_selection,
                max_selection,
            } => FormItemKindDto::ChooseMany {
                options,
                min_selection,
                max_selection,
            },
            FormItemKind::File { extensions, limit } => FormItemKindDto::File { extensions, limit },
        }
    }
}

impl From<FormItemKindDto> for FormItemKind {
    fn from(kind: FormItemKindDto) -> Self {
        match kind {
            FormItemKindDto::String {
                min_length,
                max_length,
                allow_newline,
            } => FormItemKind::String {
                min_length,
                max_length,
                allow_newline,
            },
            FormItemKindDto::Int { min, max } => FormItemKind::Int { min, max },
            FormItemKindDto::ChooseOne { options } => FormItemKind::ChooseOne { options },
            FormItemKindDto::ChooseMany {
                options,
                min_selection,
                max_selection,
            } => FormItemKind::ChooseMany {
                options,
                min_selection,
                max_selection,
            },
            FormItemKindDto::File { extensions, limit } => FormItemKind::File { extensions, limit },
        }
    }
}
