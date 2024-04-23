use serde::{Deserialize, Serialize};

use sos24_use_case::dto::form::{
    CreateFormDto, FormDto, FormItemDto, FormItemKindDto, FormSummaryDto, NewFormItemDto,
    UpdateFormDto,
};
use sos24_use_case::dto::project::{ProjectAttributeDto, ProjectCategoryDto};
use utoipa::{IntoParams, ToSchema};

use crate::model::project::{ProjectAttribute, ProjectCategory};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateForm {
    title: String,
    description: String,
    #[schema(format = "date-time")]
    starts_at: String,
    #[schema(format = "date-time")]
    ends_at: String,
    categories: Vec<ProjectCategory>,
    attributes: Vec<ProjectAttribute>,
    items: Vec<NewFormItem>,
    #[schema(format = "uuid")]
    attachments: Vec<String>,
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
                .map(NewFormItemDto::from)
                .collect(),
            create_form.attachments,
        )
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NewFormItem {
    name: String,
    description: Option<String>,
    required: bool,
    #[serde(flatten)]
    kind: FormItemKind,
}

impl From<NewFormItem> for NewFormItemDto {
    fn from(create_form_item: NewFormItem) -> Self {
        NewFormItemDto::new(
            create_form_item.name,
            create_form_item.description,
            create_form_item.required,
            FormItemKindDto::from(create_form_item.kind),
        )
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedForm {
    #[schema(format = "uuid")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateForm {
    pub title: String,
    pub description: String,
    #[schema(format = "date-time")]
    pub starts_at: String,
    #[schema(format = "date-time")]
    pub ends_at: String,
    pub categories: Vec<ProjectCategory>,
    pub attributes: Vec<ProjectAttribute>,
    pub items: Vec<NewFormItem>,
    #[schema(format = "uuid")]
    pub attachments: Vec<String>,
}

pub trait ConvertToUpdateFormDto {
    fn to_update_form_dto(self) -> UpdateFormDto;
}

impl ConvertToUpdateFormDto for (String, UpdateForm) {
    fn to_update_form_dto(self) -> UpdateFormDto {
        let (id, form) = self;
        UpdateFormDto::new(
            id,
            form.title,
            form.description,
            form.starts_at,
            form.ends_at,
            form.categories
                .into_iter()
                .map(ProjectCategoryDto::from)
                .collect(),
            form.attributes
                .into_iter()
                .map(ProjectAttributeDto::from)
                .collect(),
            form.items.into_iter().map(NewFormItemDto::from).collect(),
            form.attachments,
        )
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Form {
    #[schema(format = "uuid")]
    pub id: String,
    pub title: String,
    pub description: String,
    #[schema(format = "date-time")]
    pub starts_at: String,
    #[schema(format = "date-time")]
    pub ends_at: String,
    pub categories: Vec<ProjectCategory>,
    pub attributes: Vec<ProjectAttribute>,
    pub items: Vec<FormItem>,
    #[schema(format = "uuid")]
    pub attachments: Vec<String>,
    #[schema(format = "uuid")]
    pub answer_id: Option<String>,
    #[schema(format = "date-time")]
    pub answered_at: Option<String>,
    #[schema(format = "date-time")]
    pub created_at: String,
    #[schema(format = "date-time")]
    pub updated_at: String,
    #[schema(format = "date-time")]
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
            attachments: form.attachments,
            answer_id: form.answer_id.map(|it| it.to_string()),
            answered_at: form.answered_at.map(|it| it.to_rfc3339()),
            created_at: form.created_at.to_rfc3339(),
            updated_at: form.updated_at.to_rfc3339(),
            deleted_at: form.deleted_at.map(|it| it.to_rfc3339()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FormSummary {
    #[schema(format = "uuid")]
    pub id: String,
    pub title: String,
    pub description: String,
    #[schema(format = "date-time")]
    pub starts_at: String,
    #[schema(format = "date-time")]
    pub ends_at: String,
    pub categories: Vec<ProjectCategory>,
    pub attributes: Vec<ProjectAttribute>,
    #[schema(format = "uuid")]
    pub answer_id: Option<String>,
    #[schema(format = "date-time")]
    pub answered_at: Option<String>,
    #[schema(format = "date-time")]
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
            answer_id: form.answer_id.map(|it| it.to_string()),
            answered_at: form.answered_at.map(|it| it.to_rfc3339()),
            updated_at: form.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct FormQuery {
    #[param(format = "uuid")]
    pub project_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FormItem {
    #[schema(format = "uuid")]
    pub id: String,
    pub name: String,
    pub description: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
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
