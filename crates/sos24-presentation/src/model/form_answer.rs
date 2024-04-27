use serde::{Deserialize, Serialize};

use sos24_use_case::form_answer::{
    dto::{FormAnswerDto, FormAnswerItemDto, FormAnswerItemKindDto},
    interactor::{create::CreateFormAnswerCommand, update::UpdateFormAnswerCommand},
};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateFormAnswer {
    #[schema(format = "uuid")]
    form_id: String,
    items: Vec<FormAnswerItem>,
}

impl From<CreateFormAnswer> for CreateFormAnswerCommand {
    fn from(create_form_answer: CreateFormAnswer) -> Self {
        CreateFormAnswerCommand::new(
            create_form_answer.form_id,
            create_form_answer
                .items
                .into_iter()
                .map(FormAnswerItemDto::from)
                .collect(),
        )
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedFormAnswer {
    #[schema(format = "uuid")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateFormAnswer {
    items: Vec<FormAnswerItem>,
}

pub trait ConvertToUpdateFormAnswerDto {
    fn to_update_form_answer_dto(self) -> UpdateFormAnswerCommand;
}

impl ConvertToUpdateFormAnswerDto for (UpdateFormAnswer, String) {
    fn to_update_form_answer_dto(self) -> UpdateFormAnswerCommand {
        let (update_form_answer, id) = self;
        UpdateFormAnswerCommand::new(
            id,
            update_form_answer
                .items
                .into_iter()
                .map(FormAnswerItemDto::from)
                .collect(),
        )
    }
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct FormAnswerQuery {
    #[param(format = "uuid")]
    pub project_id: Option<String>,
    #[param(format = "uuid")]
    pub form_id: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ExportFormAnswerQuery {
    #[param(format = "uuid")]
    pub form_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FormAnswer {
    #[schema(format = "uuid")]
    id: String,
    #[schema(format = "uuid")]
    project_id: String,
    project_title: String,
    #[schema(format = "uuid")]
    form_id: String,
    form_title: String,
    items: Vec<FormAnswerItem>,
    #[schema(format = "date-time")]
    created_at: String,
    #[schema(format = "date-time")]
    updated_at: String,
    #[schema(format = "date-time")]
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

#[derive(Debug, Serialize, ToSchema)]
pub struct FormAnswerSummary {
    #[schema(format = "uuid")]
    id: String,
    #[schema(format = "uuid")]
    project_id: String,
    project_title: String,
    #[schema(format = "uuid")]
    form_id: String,
    form_title: String,
    #[schema(format = "date-time")]
    updated_at: String,
}

impl From<FormAnswerDto> for FormAnswerSummary {
    fn from(form_answer_dto: FormAnswerDto) -> Self {
        FormAnswerSummary {
            id: form_answer_dto.id,
            project_id: form_answer_dto.project_id,
            project_title: form_answer_dto.project_title,
            form_id: form_answer_dto.form_id,
            form_title: form_answer_dto.form_title,
            updated_at: form_answer_dto.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FormAnswerItem {
    String {
        #[schema(format = "uuid")]
        item_id: String,
        value: String,
    },
    Int {
        #[schema(format = "uuid")]
        item_id: String,
        value: i32,
    },
    ChooseOne {
        #[schema(format = "uuid")]
        item_id: String,
        value: String,
    },
    ChooseMany {
        #[schema(format = "uuid")]
        item_id: String,
        value: Vec<String>,
    },
    File {
        #[schema(format = "uuid")]
        item_id: String,
        #[schema(format = "uuid")]
        value: Vec<String>,
    },
}

impl From<FormAnswerItem> for FormAnswerItemDto {
    fn from(form_answer_item: FormAnswerItem) -> Self {
        match form_answer_item {
            FormAnswerItem::String { item_id, value } => {
                FormAnswerItemDto::new(item_id, FormAnswerItemKindDto::String(value))
            }
            FormAnswerItem::Int { item_id, value } => {
                FormAnswerItemDto::new(item_id, FormAnswerItemKindDto::Int(value))
            }
            FormAnswerItem::ChooseOne { item_id, value } => {
                FormAnswerItemDto::new(item_id, FormAnswerItemKindDto::ChooseOne(value))
            }
            FormAnswerItem::ChooseMany { item_id, value } => {
                FormAnswerItemDto::new(item_id, FormAnswerItemKindDto::ChooseMany(value))
            }
            FormAnswerItem::File { item_id, value } => {
                FormAnswerItemDto::new(item_id, FormAnswerItemKindDto::File(value))
            }
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
