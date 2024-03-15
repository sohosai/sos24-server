use serde::{Deserialize, Serialize};
use sos24_use_case::dto::form::{CreateFormDto, CreateFormItemDto, FormItemKindDto};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateForm {
    title: String,
    description: String,
    starts_at: String,
    ends_at: String,
    items: Vec<CreateFormItem>,
}

impl From<CreateForm> for CreateFormDto {
    fn from(create_form: CreateForm) -> Self {
        CreateFormDto::new(
            create_form.title,
            create_form.description,
            create_form.starts_at,
            create_form.ends_at,
            create_form
                .items
                .into_iter()
                .map(CreateFormItemDto::from)
                .collect(),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CreateFormItem {
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
        extentions: Vec<String>,
        limit: i32,
    },
}

impl From<CreateFormItem> for CreateFormItemDto {
    fn from(value: CreateFormItem) -> Self {
        match value {
            CreateFormItem::String {
                name,
                description,
                required,
                min_length,
                max_length,
                allow_newline,
            } => CreateFormItemDto::new(
                name,
                description,
                required,
                FormItemKindDto::String {
                    min_length,
                    max_length,
                    allow_newline,
                },
            ),
            CreateFormItem::Int {
                name,
                description,
                required,
                min,
                max,
            } => CreateFormItemDto::new(
                name,
                description,
                required,
                FormItemKindDto::Int { min, max },
            ),
            CreateFormItem::ChooseOne {
                name,
                description,
                required,
                options,
            } => CreateFormItemDto::new(
                name,
                description,
                required,
                FormItemKindDto::ChooseOne { options },
            ),
            CreateFormItem::ChooseMany {
                name,
                description,
                required,
                options,
                min_selection,
                max_selection,
            } => CreateFormItemDto::new(
                name,
                description,
                required,
                FormItemKindDto::ChooseMany {
                    options,
                    min_selection,
                    max_selection,
                },
            ),
            CreateFormItem::File {
                name,
                description,
                required,
                extentions,
                limit,
            } => CreateFormItemDto::new(
                name,
                description,
                required,
                FormItemKindDto::File { extentions, limit },
            ),
        }
    }
}
