use anyhow::Context;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use sos24_domain::{
    entity::form::{Form, FormItem, FormItemKind},
    repository::form::{FormRepository, FormRepositoryError},
};

use super::MongoDb;

#[derive(Debug, Serialize, Deserialize)]
pub struct FormDoc {
    _id: uuid::Uuid,
    title: String,
    description: String,
    starts_at: chrono::DateTime<chrono::Utc>,
    ends_at: chrono::DateTime<chrono::Utc>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    items: Vec<FormItemDoc>,
}

impl From<Form> for FormDoc {
    fn from(form: Form) -> Self {
        let form = form.destruct();
        Self {
            _id: form.id.value(),
            title: form.title.value(),
            description: form.description.value(),
            starts_at: form.starts_at.value(),
            ends_at: form.ends_at.value(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
            items: form.items.into_iter().map(FormItemDoc::from).collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormItemDoc {
    name: String,
    description: String,
    required: bool,
    kind: FormItemKindDoc,
}

impl From<FormItem> for FormItemDoc {
    fn from(value: FormItem) -> Self {
        let value = value.destruct();
        Self {
            name: value.name.value(),
            description: value.description.value(),
            required: value.required.value(),
            kind: FormItemKindDoc::from(value.kind),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FormItemKindDoc {
    String {
        min_length: i32,
        max_length: i32,
        allow_newline: bool,
    },
    Int {
        min: i32,
        max: i32,
    },
    ChooseOne {
        options: Vec<String>,
    },
    ChooseMany {
        options: Vec<String>,
        min_selection: i32,
        max_selection: i32,
    },
    File {
        extentions: Vec<String>,
        limit: i32,
    },
}

impl From<FormItemKind> for FormItemKindDoc {
    fn from(value: FormItemKind) -> Self {
        match value {
            FormItemKind::String {
                min_length,
                max_length,
                allow_newline,
            } => Self::String {
                min_length: min_length.value(),
                max_length: max_length.value(),
                allow_newline: allow_newline.value(),
            },
            FormItemKind::Int { min, max } => Self::Int {
                min: min.value(),
                max: max.value(),
            },
            FormItemKind::ChooseOne { options } => Self::ChooseOne {
                options: options.into_iter().map(|it| it.value()).collect(),
            },
            FormItemKind::ChooseMany {
                options,
                min_selection,
                max_selection,
            } => Self::ChooseMany {
                options: options.into_iter().map(|it| it.value()).collect(),
                min_selection: min_selection.value(),
                max_selection: max_selection.value(),
            },
            FormItemKind::File { extentions, limit } => Self::File {
                extentions: extentions.into_iter().map(|it| it.value()).collect(),
                limit: limit.value(),
            },
        }
    }
}

pub struct MongoFormRepository {
    collection: Collection<FormDoc>,
}

impl MongoFormRepository {
    pub fn new(mongodb: MongoDb) -> Self {
        Self {
            collection: mongodb.collection("forms"),
        }
    }
}

impl FormRepository for MongoFormRepository {
    async fn create(&self, form: Form) -> Result<(), FormRepositoryError> {
        let form_doc = FormDoc::from(form);
        self.collection
            .insert_one(form_doc, None)
            .await
            .context("Failed to create form")?;
        Ok(())
    }
}
