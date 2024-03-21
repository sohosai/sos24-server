use serde::{Deserialize, Serialize};
use sos24_use_case::dto::form_answer::{
    CreateFormAnswerDto, FormAnswerItemDto, FormAnswerItemKindDto,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFormAnswer {
    project_id: String,
    form_id: String,
    items: Vec<FormAnswerItem>,
}

impl From<CreateFormAnswer> for CreateFormAnswerDto {
    fn from(create_form_answer: CreateFormAnswer) -> Self {
        CreateFormAnswerDto::new(
            create_form_answer.project_id,
            create_form_answer.form_id,
            create_form_answer
                .items
                .into_iter()
                .map(FormAnswerItemDto::from)
                .collect(),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FormAnswerItem {
    String { form_id: String, value: String },
    Int { form_id: String, value: i32 },
    ChooseOne { form_id: String, value: String },
    ChooseMany { form_id: String, value: Vec<String> },
    File { form_id: String, value: String },
}

impl From<FormAnswerItem> for FormAnswerItemDto {
    fn from(form_answer_item: FormAnswerItem) -> Self {
        match form_answer_item {
            FormAnswerItem::String { form_id, value } => {
                FormAnswerItemDto::new(form_id, FormAnswerItemKindDto::String(value))
            }
            FormAnswerItem::Int { form_id, value } => {
                FormAnswerItemDto::new(form_id, FormAnswerItemKindDto::Int(value))
            }
            FormAnswerItem::ChooseOne { form_id, value } => {
                FormAnswerItemDto::new(form_id, FormAnswerItemKindDto::ChooseOne(value))
            }
            FormAnswerItem::ChooseMany { form_id, value } => {
                FormAnswerItemDto::new(form_id, FormAnswerItemKindDto::ChooseMany(value))
            }
            FormAnswerItem::File { form_id, value } => {
                FormAnswerItemDto::new(form_id, FormAnswerItemKindDto::File(value))
            }
        }
    }
}
