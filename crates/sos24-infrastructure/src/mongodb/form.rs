use anyhow::Context;
use futures_util::{StreamExt, TryStreamExt};
use mongodb::{
    bson::{self, doc},
    Collection,
};
use serde::{Deserialize, Serialize};

use sos24_domain::entity::form::FormItemId;
use sos24_domain::{
    entity::{
        common::{date::WithDate, datetime::DateTime},
        form::{
            Form, FormDescription, FormId, FormItem, FormItemAllowNewline, FormItemDescription,
            FormItemExtention, FormItemKind, FormItemLimit, FormItemMax, FormItemMaxLength,
            FormItemMaxSelection, FormItemMin, FormItemMinLength, FormItemMinSelection,
            FormItemName, FormItemOption, FormItemRequired, FormTitle,
        },
    },
    repository::form::{FormRepository, FormRepositoryError},
};

use super::MongoDb;

#[derive(Debug, Serialize, Deserialize)]
pub struct FormDoc {
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    _id: uuid::Uuid,
    title: String,
    description: String,
    starts_at: chrono::DateTime<chrono::Utc>,
    ends_at: chrono::DateTime<chrono::Utc>,
    items: Vec<FormItemDoc>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
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
            items: form.items.into_iter().map(FormItemDoc::from).collect(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
        }
    }
}

impl From<FormDoc> for WithDate<Form> {
    fn from(value: FormDoc) -> Self {
        WithDate::new(
            Form::new(
                FormId::new(value._id),
                FormTitle::new(value.title),
                FormDescription::new(value.description),
                DateTime::new(value.starts_at),
                DateTime::new(value.ends_at),
                value.items.into_iter().map(FormItemDoc::into).collect(),
            ),
            value.created_at,
            value.updated_at,
            value.deleted_at,
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormItemDoc {
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    _id: uuid::Uuid,
    name: String,
    description: String,
    required: bool,
    kind: FormItemKindDoc,
}

impl From<FormItem> for FormItemDoc {
    fn from(value: FormItem) -> Self {
        let value = value.destruct();
        Self {
            _id: value.id.value(),
            name: value.name.value(),
            description: value.description.value(),
            required: value.required.value(),
            kind: FormItemKindDoc::from(value.kind),
        }
    }
}

impl From<FormItemDoc> for FormItem {
    fn from(value: FormItemDoc) -> Self {
        FormItem::new(
            FormItemId::new(value._id),
            FormItemName::new(value.name),
            FormItemDescription::new(value.description),
            FormItemRequired::new(value.required),
            FormItemKind::from(value.kind),
        )
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
            FormItemKind::String(item) => {
                let item = item.destruct();
                Self::String {
                    min_length: item.min_length.value(),
                    max_length: item.max_length.value(),
                    allow_newline: item.allow_newline.value(),
                }
            }
            FormItemKind::Int(item) => {
                let item = item.destruct();
                Self::Int {
                    min: item.min.value(),
                    max: item.max.value(),
                }
            }
            FormItemKind::ChooseOne(item) => {
                let item = item.destruct();
                Self::ChooseOne {
                    options: item.options.into_iter().map(|it| it.value()).collect(),
                }
            }
            FormItemKind::ChooseMany(item) => {
                let item = item.destruct();
                Self::ChooseMany {
                    options: item.options.into_iter().map(|it| it.value()).collect(),
                    min_selection: item.min_selection.value(),
                    max_selection: item.max_selection.value(),
                }
            }
            FormItemKind::File(item) => {
                let item = item.destruct();
                Self::File {
                    extentions: item.extentions.into_iter().map(|it| it.value()).collect(),
                    limit: item.limit.value(),
                }
            }
        }
    }
}

impl From<FormItemKindDoc> for FormItemKind {
    fn from(value: FormItemKindDoc) -> Self {
        match value {
            FormItemKindDoc::String {
                min_length,
                max_length,
                allow_newline,
            } => FormItemKind::new_string(
                FormItemMinLength::new(min_length),
                FormItemMaxLength::new(max_length),
                FormItemAllowNewline::new(allow_newline),
            ),
            FormItemKindDoc::Int { min, max } => {
                FormItemKind::new_int(FormItemMin::new(min), FormItemMax::new(max))
            }
            FormItemKindDoc::ChooseOne { options } => {
                FormItemKind::new_choose_one(options.into_iter().map(FormItemOption::new).collect())
            }
            FormItemKindDoc::ChooseMany {
                options,
                min_selection,
                max_selection,
            } => FormItemKind::new_choose_many(
                options.into_iter().map(FormItemOption::new).collect(),
                FormItemMinSelection::new(min_selection),
                FormItemMaxSelection::new(max_selection),
            ),
            FormItemKindDoc::File { extentions, limit } => FormItemKind::new_file(
                extentions.into_iter().map(FormItemExtention::new).collect(),
                FormItemLimit::new(limit),
            ),
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
    async fn list(&self) -> Result<Vec<WithDate<Form>>, FormRepositoryError> {
        let form_list = self
            .collection
            .find(doc! { "deleted_at": None::<String> }, None)
            .await
            .context("Failed to list forms")?;
        let forms = form_list
            .map(|doc| {
                Ok::<_, anyhow::Error>(WithDate::from(doc.context("Failed to fetch form list")?))
            })
            .try_collect()
            .await?;
        Ok(forms)
    }

    async fn create(&self, form: Form) -> Result<(), FormRepositoryError> {
        let form_doc = FormDoc::from(form);
        self.collection
            .insert_one(form_doc, None)
            .await
            .context("Failed to create form")?;
        Ok(())
    }

    async fn find_by_id(&self, id: FormId) -> Result<Option<WithDate<Form>>, FormRepositoryError> {
        let form_doc = self
            .collection
            .find_one(doc! { "_id": id.value() }, None)
            .await
            .context("Failed to find form")?;
        Ok(form_doc.map(WithDate::from))
    }

    async fn delete_by_id(&self, id: FormId) -> Result<(), FormRepositoryError> {
        self.collection
            .update_one(
                doc! { "_id": id.value() },
                doc! { "$set": { "deleted_at": chrono::Utc::now() } },
                None,
            )
            .await
            .context("Failed to delete form")?;
        Ok(())
    }
}
