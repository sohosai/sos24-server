use anyhow::{anyhow, Context};
use futures_util::{StreamExt, TryStreamExt};
use mongodb::{
    bson::{self, doc},
    Collection,
};
use serde::{Deserialize, Serialize};

use sos24_domain::entity::form::{FormItemExtension, FormItemId};
use sos24_domain::entity::project::{ProjectAttributes, ProjectCategories};
use sos24_domain::entity::{file_data::FileId, form::FormIsNotified};
use sos24_domain::{
    entity::{
        common::datetime::DateTime,
        form::{
            Form, FormDescription, FormId, FormItem, FormItemAllowNewline, FormItemDescription,
            FormItemKind, FormItemLimit, FormItemMax, FormItemMaxLength, FormItemMaxSelection,
            FormItemMin, FormItemMinLength, FormItemMinSelection, FormItemName, FormItemOption,
            FormItemRequired, FormTitle,
        },
    },
    repository::form::{FormRepository, FormRepositoryError},
};

use crate::shared::mongodb::MongoDb;

#[derive(Debug, Serialize, Deserialize)]
pub struct FormDoc {
    _id: String,
    title: String,
    description: String,
    starts_at: chrono::DateTime<chrono::Utc>,
    ends_at: chrono::DateTime<chrono::Utc>,
    categories: i32,
    attributes: i32,
    is_notified: bool,
    items: Vec<FormItemDoc>,
    attachments: Vec<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Form> for FormDoc {
    fn from(form: Form) -> Self {
        let form = form.destruct();
        Self {
            _id: form.id.value().to_string(),
            title: form.title.value(),
            description: form.description.value(),
            starts_at: form.starts_at.value(),
            ends_at: form.ends_at.value(),
            categories: form.categories.bits() as i32,
            attributes: form.attributes.bits() as i32,
            items: form.items.into_iter().map(FormItemDoc::from).collect(),
            attachments: form
                .attachments
                .into_iter()
                .map(|it| it.value().to_string())
                .collect(),
            is_notified: form.is_notified.value(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}

impl TryFrom<FormDoc> for Form {
    type Error = anyhow::Error;
    fn try_from(value: FormDoc) -> Result<Self, Self::Error> {
        Ok(Form::new(
            FormId::try_from(value._id)?,
            FormTitle::new(value.title),
            FormDescription::new(value.description),
            DateTime::new(value.starts_at),
            DateTime::new(value.ends_at),
            ProjectCategories::from_bits(value.categories as u32)
                .ok_or(anyhow!("cannot convert project categories"))?,
            ProjectAttributes::from_bits(value.attributes as u32)
                .ok_or(anyhow!("cannot convert project attributes"))?,
            FormIsNotified::new(value.is_notified),
            value
                .items
                .into_iter()
                .map(FormItem::try_from)
                .collect::<Result<_, _>>()?,
            value
                .attachments
                .into_iter()
                .map(FileId::try_from)
                .collect::<Result<_, _>>()?,
            DateTime::new(value.created_at),
            DateTime::new(value.updated_at),
        ))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormItemDoc {
    _id: String,
    name: String,
    description: Option<String>,
    required: bool,
    kind: FormItemKindDoc,
}

impl From<FormItem> for FormItemDoc {
    fn from(value: FormItem) -> Self {
        let value = value.destruct();
        Self {
            _id: value.id.value().to_string(),
            name: value.name.value(),
            description: value.description.map(|it| it.value()),
            required: value.required.value(),
            kind: FormItemKindDoc::from(value.kind),
        }
    }
}

impl TryFrom<FormItemDoc> for FormItem {
    type Error = anyhow::Error;
    fn try_from(value: FormItemDoc) -> Result<Self, Self::Error> {
        Ok(FormItem::new(
            FormItemId::try_from(value._id)?,
            FormItemName::new(value.name),
            value.description.map(FormItemDescription::new),
            FormItemRequired::new(value.required),
            FormItemKind::try_from(value.kind)?,
        ))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FormItemKindDoc {
    String {
        min_length: Option<u32>,
        max_length: Option<u32>,
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
        min_selection: Option<u32>,
        max_selection: Option<u32>,
    },
    File {
        extensions: Option<Vec<String>>,
        limit: Option<u32>,
    },
}

impl From<FormItemKind> for FormItemKindDoc {
    fn from(value: FormItemKind) -> Self {
        match value {
            FormItemKind::String(item) => {
                let item = item.destruct();
                Self::String {
                    min_length: item.min_length.map(|it| it.value()),
                    max_length: item.max_length.map(|it| it.value()),
                    allow_newline: item.allow_newline.value(),
                }
            }
            FormItemKind::Int(item) => {
                let item = item.destruct();
                Self::Int {
                    min: item.min.map(|it| it.value()),
                    max: item.max.map(|it| it.value()),
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
                    min_selection: item.min_selection.map(|it| it.value()),
                    max_selection: item.max_selection.map(|it| it.value()),
                }
            }
            FormItemKind::File(item) => {
                let item = item.destruct();
                Self::File {
                    extensions: item
                        .extensions
                        .map(|it| it.into_iter().map(|it| it.value()).collect()),
                    limit: item.limit.map(|it| it.value()),
                }
            }
        }
    }
}

impl TryFrom<FormItemKindDoc> for FormItemKind {
    type Error = anyhow::Error;
    fn try_from(value: FormItemKindDoc) -> Result<Self, Self::Error> {
        match value {
            FormItemKindDoc::String {
                min_length,
                max_length,
                allow_newline,
            } => Ok(FormItemKind::new_string(
                min_length.map(FormItemMinLength::new),
                max_length.map(FormItemMaxLength::new),
                FormItemAllowNewline::new(allow_newline),
            )?),
            FormItemKindDoc::Int { min, max } => Ok(FormItemKind::new_int(
                min.map(FormItemMin::new),
                max.map(FormItemMax::new),
            )?),
            FormItemKindDoc::ChooseOne { options } => Ok(FormItemKind::new_choose_one(
                options.into_iter().map(FormItemOption::new).collect(),
            )?),
            FormItemKindDoc::ChooseMany {
                options,
                min_selection,
                max_selection,
            } => Ok(FormItemKind::new_choose_many(
                options.into_iter().map(FormItemOption::new).collect(),
                min_selection.map(FormItemMinSelection::new),
                max_selection.map(FormItemMaxSelection::new),
            )?),
            FormItemKindDoc::File { extensions, limit } => Ok(FormItemKind::new_file(
                extensions.map(|it| it.into_iter().map(FormItemExtension::new).collect()),
                limit.map(FormItemLimit::new),
            )),
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
    async fn list(&self) -> Result<Vec<Form>, FormRepositoryError> {
        tracing::info!("申請一覧を取得します");

        let form_list = self
            .collection
            .aggregate(vec![
                doc! { "$match": { "deleted_at": None::<String> } },
                doc! { "$sort": { "ends_at": 1 } },
            ])
            .await
            .context("Failed to list forms")?;
        let forms = form_list
            .map(|doc| Form::try_from(bson::from_document::<FormDoc>(doc?)?))
            .try_collect()
            .await?;

        tracing::info!("申請一覧を取得しました");
        Ok(forms)
    }

    async fn create(&self, form: Form) -> Result<(), FormRepositoryError> {
        tracing::info!("申請を作成します");

        let form_doc = FormDoc::from(form);
        self.collection
            .insert_one(form_doc)
            .await
            .context("Failed to create form")?;

        tracing::info!("申請を作成しました");
        Ok(())
    }

    async fn find_by_id(&self, id: FormId) -> Result<Option<Form>, FormRepositoryError> {
        tracing::info!("申請を取得します: {id:?}");

        let form_doc = self
            .collection
            .find_one(
                doc! { "_id": id.clone().value().to_string(),  "deleted_at": None::<String>  },
            )
            .await
            .context("Failed to find form")?;

        tracing::info!("申請を取得しました: {id:?}");
        Ok(form_doc.map(Form::try_from).transpose()?)
    }

    async fn update(&self, form: Form) -> Result<(), FormRepositoryError> {
        tracing::info!("申請を更新します");

        let form_doc = FormDoc::from(form);
        self.collection
            .update_one(
                doc! { "_id": form_doc._id,  "deleted_at": None::<String> },
                doc! { "$set":
                    doc! {
                        "title": bson::to_bson(&form_doc.title).unwrap(),
                        "description": bson::to_bson(&form_doc.description).unwrap(),
                        "starts_at": bson::to_bson(&form_doc.starts_at).unwrap(),
                        "ends_at":bson::to_bson(&form_doc.ends_at).unwrap(),
                        "categories": bson::to_bson(&form_doc.categories).unwrap(),
                        "attributes": bson::to_bson(&form_doc.attributes).unwrap(),
                        "attachments": bson::to_bson(&form_doc.attachments).unwrap(),
                        "is_notified": bson::to_bson(&form_doc.is_notified).unwrap(),
                        "items": bson::to_bson(&form_doc.items).unwrap(),
                        "updated_at": bson::to_bson(&form_doc.updated_at).unwrap(),
                    }
                },
            )
            .await
            .context("Failed to update form")?;

        tracing::info!("申請を更新しました");
        Ok(())
    }

    async fn delete_by_id(&self, id: FormId) -> Result<(), FormRepositoryError> {
        tracing::info!("申請を削除します: {id:?}");

        self.collection
            .update_one(
                doc! { "_id": id.clone().value().to_string(),  "deleted_at": None::<String> },
                doc! { "$set": { "deleted_at": bson::to_bson(&chrono::Utc::now()).unwrap() } },
            )
            .await
            .context("Failed to delete form")?;

        tracing::info!("申請を削除しました: {id:?}");
        Ok(())
    }
}
