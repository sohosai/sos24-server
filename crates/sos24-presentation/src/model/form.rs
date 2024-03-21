use serde::{Deserialize, Serialize};

use sos24_use_case::dto::form::{CreateFormDto, FormDto, FormItemDto, FormItemKindDto};
use sos24_use_case::dto::project::{ProjectAttributeDto, ProjectCategoryDto};

use crate::model::project::{ProjectAttribute, ProjectCategory};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateForm {
    title: String,
    description: String,
    starts_at: String,
    ends_at: String,
    categories: Vec<ProjectCategory>,
    attributes: Vec<ProjectAttribute>,
    items: Vec<FormItem>,
}

impl From<CreateForm> for CreateFormDto {
    fn from(create_form: CreateForm) -> Self {
        CreateFormDto::new(
            create_form.title,
            create_form.description,
            create_form.starts_at,
            create_form.ends_at,
            create_form
                .categories
                .into_iter()
                .map(ProjectCategoryDto::from)
                .collect(),
            create_form
                .attributes
                .into_iter()
                .map(ProjectAttributeDto::from)
                .collect(),
            create_form
                .items
                .into_iter()
                .map(FormItemDto::from)
                .collect(),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Form {
    pub id: String,
    pub title: String,
    pub description: String,
    pub starts_at: String,
    pub ends_at: String,
    pub categories: Vec<ProjectCategory>,
    pub attributes: Vec<ProjectAttribute>,
    pub items: Vec<FormItem>,
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
            categories: form
                .categories
                .into_iter()
                .map(ProjectCategory::from)
                .collect(),
            attributes: form
                .attributes
                .into_iter()
                .map(ProjectAttribute::from)
                .collect(),
            items: form.items.into_iter().map(FormItem::from).collect(),
            created_at: form.created_at.to_rfc3339(),
            updated_at: form.updated_at.to_rfc3339(),
            deleted_at: form.deleted_at.map(|it| it.to_rfc3339()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FormItem {
    String {
        name: String,
        description: String,
        required: bool,
        min_length: i32,
        max_length: i32,
        allow_newline: bool,
    },
    Int {
        name: String,
        description: String,
        required: bool,
        min: i32,
        max: i32,
    },
    ChooseOne {
        name: String,
        description: String,
        required: bool,
        options: Vec<String>,
    },
    ChooseMany {
        name: String,
        description: String,
        required: bool,
        options: Vec<String>,
        min_selection: i32,
        max_selection: i32,
    },
    File {
        name: String,
        description: String,
        required: bool,
        extensions: Vec<String>,
        limit: i32,
    },
}

impl From<FormItem> for FormItemDto {
    fn from(value: FormItem) -> Self {
        match value {
            FormItem::String {
                name,
                description,
                required,
                min_length,
                max_length,
                allow_newline,
            } => FormItemDto::new(
                name,
                description,
                required,
                FormItemKindDto::String {
                    min_length,
                    max_length,
                    allow_newline,
                },
            ),
            FormItem::Int {
                name,
                description,
                required,
                min,
                max,
            } => FormItemDto::new(
                name,
                description,
                required,
                FormItemKindDto::Int { min, max },
            ),
            FormItem::ChooseOne {
                name,
                description,
                required,
                options,
            } => FormItemDto::new(
                name,
                description,
                required,
                FormItemKindDto::ChooseOne { options },
            ),
            FormItem::ChooseMany {
                name,
                description,
                required,
                options,
                min_selection,
                max_selection,
            } => FormItemDto::new(
                name,
                description,
                required,
                FormItemKindDto::ChooseMany {
                    options,
                    min_selection,
                    max_selection,
                },
            ),
            FormItem::File {
                name,
                description,
                required,
                extensions,
                limit,
            } => FormItemDto::new(
                name,
                description,
                required,
                FormItemKindDto::File { extensions, limit },
            ),
        }
    }
}

impl From<FormItemDto> for FormItem {
    fn from(value: FormItemDto) -> Self {
        match value.kind {
            FormItemKindDto::String {
                min_length,
                max_length,
                allow_newline,
            } => FormItem::String {
                name: value.name,
                description: value.description,
                required: value.required,
                min_length,
                max_length,
                allow_newline,
            },
            FormItemKindDto::Int { min, max } => FormItem::Int {
                name: value.name,
                description: value.description,
                required: value.required,
                min,
                max,
            },
            FormItemKindDto::ChooseOne { options } => FormItem::ChooseOne {
                name: value.name,
                description: value.description,
                required: value.required,
                options,
            },
            FormItemKindDto::ChooseMany {
                options,
                min_selection,
                max_selection,
            } => FormItem::ChooseMany {
                name: value.name,
                description: value.description,
                required: value.required,
                options,
                min_selection,
                max_selection,
            },
            FormItemKindDto::File { extensions, limit } => FormItem::File {
                name: value.name,
                description: value.description,
                required: value.required,
                extensions,
                limit,
            },
        }
    }
}
