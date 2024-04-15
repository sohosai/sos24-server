use anyhow::{anyhow, Context};
use futures_util::{StreamExt, TryStreamExt};
use mongodb::{
    bson::{self, doc},
    Collection,
};
use serde::{Deserialize, Serialize};

use sos24_domain::entity::file_data::FileId;
use sos24_domain::entity::form::{FormItemExtension, FormItemId};
use sos24_domain::entity::project::{ProjectAttributes, ProjectCategories};
use sos24_domain::{
    entity::{
        common::{date::WithDate, datetime::DateTime},
        form::{
            Form, FormDescription, FormId, FormItem, FormItemAllowNewline, FormItemDescription,
            FormItemKind, FormItemLimit, FormItemMax, FormItemMaxLength, FormItemMaxSelection,
            FormItemMin, FormItemMinLength, FormItemMinSelection, FormItemName, FormItemOption,
            FormItemRequired, FormTitle,
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
    categories: i32,
    attributes: i32,
    items: Vec<FormItemDoc>,
    attachments: Vec<String>,
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
            categories: form.categories.bits() as i32,
            attributes: form.attributes.bits() as i32,
            items: form.items.into_iter().map(FormItemDoc::from).collect(),
            attachments: form
                .attachments
                .into_iter()
                .map(|it| it.value().to_string())
                .collect(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
        }
    }
}

impl TryFrom<FormDoc> for WithDate<Form> {
    type Error = anyhow::Error;
    fn try_from(value: FormDoc) -> Result<Self, Self::Error> {
        Ok(WithDate::new(
            Form::new(
                FormId::new(value._id),
                FormTitle::new(value.title),
                FormDescription::new(value.description),
                DateTime::new(value.starts_at),
                DateTime::new(value.ends_at),
                ProjectCategories::from_bits(value.categories as u32)
                    .ok_or(anyhow!("cannot convert project categories"))?,
                ProjectAttributes::from_bits(value.attributes as u32)
                    .ok_or(anyhow!("cannot convert project attributes"))?,
                value.items.into_iter().map(FormItemDoc::into).collect(),
                value
                    .attachments
                    .into_iter()
                    .map(FileId::try_from)
                    .collect::<Result<_, _>>()?,
            ),
            value.created_at,
            value.updated_at,
            value.deleted_at,
        ))
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

impl From<FormItemKindDoc> for FormItemKind {
    fn from(value: FormItemKindDoc) -> Self {
        match value {
            FormItemKindDoc::String {
                min_length,
                max_length,
                allow_newline,
            } => FormItemKind::new_string(
                min_length.map(FormItemMinLength::new),
                max_length.map(FormItemMaxLength::new),
                FormItemAllowNewline::new(allow_newline),
            ),
            FormItemKindDoc::Int { min, max } => {
                FormItemKind::new_int(min.map(FormItemMin::new), max.map(FormItemMax::new))
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
                min_selection.map(FormItemMinSelection::new),
                max_selection.map(FormItemMaxSelection::new),
            ),
            FormItemKindDoc::File { extensions, limit } => FormItemKind::new_file(
                extensions.map(|it| it.into_iter().map(FormItemExtension::new).collect()),
                limit.map(FormItemLimit::new),
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
        tracing::info!("申請一覧を取得します");

        let form_list = self
            .collection
            .aggregate(
                vec![
                    doc! { "$match": { "deleted_at": None::<String> } },
                    doc! { "$sort": { "ends_at": 1 } },
                ],
                None,
            )
            .await
            .context("Failed to list forms")?;
        let forms = form_list
            .map(|doc| WithDate::try_from(bson::from_document::<FormDoc>(doc?)?))
            .try_collect()
            .await?;

        tracing::info!("申請一覧を取得しました");
        Ok(forms)
    }

    async fn create(&self, form: Form) -> Result<(), FormRepositoryError> {
        tracing::info!("申請を作成します");

        let form_doc = FormDoc::from(form);
        self.collection
            .insert_one(form_doc, None)
            .await
            .context("Failed to create form")?;

        tracing::info!("申請を作成しました");
        Ok(())
    }

    async fn find_by_id(&self, id: FormId) -> Result<Option<WithDate<Form>>, FormRepositoryError> {
        tracing::info!("申請を取得します: {id:?}");

        let form_doc = self
            .collection
            .find_one(
                doc! { "_id": id.clone().value(),  "deleted_at": None::<String>  },
                None,
            )
            .await
            .context("Failed to find form")?;

        tracing::info!("申請を取得しました: {id:?}");
        Ok(form_doc.map(WithDate::try_from).transpose()?)
    }

    async fn update(&self, form: Form) -> Result<(), FormRepositoryError> {
        tracing::info!("申請を更新します");

        let form_doc = FormDoc::from(form);
        self.collection
            .update_one(
                doc! { "_id": form_doc._id,  "deleted_at": None::<String> },
                doc! { "$set":
                    doc! {
                        "title": form_doc.title,
                        "description": form_doc.description,
                        "starts_at": form_doc.starts_at,
                        "ends_at": form_doc.ends_at,
                        "categories": form_doc.categories,
                        "attributes": form_doc.attributes,
                        "items": bson::to_bson(&form_doc.items).unwrap(),
                        "updated_at": form_doc.updated_at,
                    }
                },
                None,
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
                doc! { "_id": id.clone().value(),  "deleted_at": None::<String> },
                doc! { "$set": { "deleted_at": chrono::Utc::now() } },
                None,
            )
            .await
            .context("Failed to delete form")?;

        tracing::info!("申請を削除しました: {id:?}");
        Ok(())
    }
}
