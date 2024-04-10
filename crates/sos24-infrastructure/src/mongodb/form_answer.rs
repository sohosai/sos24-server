use anyhow::Context;
use futures_util::{StreamExt, TryStreamExt};
use mongodb::{
    bson::{self, doc},
    Collection,
};
use serde::{Deserialize, Serialize};

use sos24_domain::entity::file_data::FileId;
use sos24_domain::{
    entity::{
        common::date::WithDate,
        form::{FormId, FormItemId},
        form_answer::{
            FormAnswer, FormAnswerId, FormAnswerItem, FormAnswerItemChooseMany,
            FormAnswerItemChooseOne, FormAnswerItemFile, FormAnswerItemInt, FormAnswerItemKind,
            FormAnswerItemString,
        },
        project::ProjectId,
    },
    repository::form_answer::{FormAnswerRepository, FormAnswerRepositoryError},
};

use super::MongoDb;

#[derive(Debug, Serialize, Deserialize)]
pub struct FormAnswerDoc {
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    _id: uuid::Uuid,
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    project_id: uuid::Uuid,
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    form_id: uuid::Uuid,
    items: Vec<FormAnswerItemDoc>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<FormAnswer> for FormAnswerDoc {
    fn from(form_answer: FormAnswer) -> Self {
        let form_answer = form_answer.destruct();
        Self {
            _id: form_answer.id.value(),
            project_id: form_answer.project_id.value(),
            form_id: form_answer.form_id.value(),
            items: form_answer
                .items
                .into_iter()
                .map(FormAnswerItemDoc::from)
                .collect(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
        }
    }
}

impl TryFrom<FormAnswerDoc> for WithDate<FormAnswer> {
    type Error = anyhow::Error;
    fn try_from(form_answer_doc: FormAnswerDoc) -> Result<Self, Self::Error> {
        Ok(WithDate::new(
            FormAnswer::new(
                FormAnswerId::new(form_answer_doc._id),
                ProjectId::new(form_answer_doc.project_id),
                FormId::new(form_answer_doc.form_id),
                form_answer_doc
                    .items
                    .into_iter()
                    .map(FormAnswerItem::try_from)
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            form_answer_doc.created_at,
            form_answer_doc.updated_at,
            form_answer_doc.deleted_at,
        ))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormAnswerItemDoc {
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    item_id: uuid::Uuid,
    kind: FormAnswerItemKindDoc,
}

impl From<FormAnswerItem> for FormAnswerItemDoc {
    fn from(form_answer_item: FormAnswerItem) -> Self {
        let form_answer_item = form_answer_item.destruct();
        Self {
            item_id: form_answer_item.item_id.value(),
            kind: form_answer_item.kind.into(),
        }
    }
}

impl TryFrom<FormAnswerItemDoc> for FormAnswerItem {
    type Error = anyhow::Error;
    fn try_from(form_answer_item_doc: FormAnswerItemDoc) -> Result<Self, Self::Error> {
        Ok(FormAnswerItem::new(
            FormItemId::new(form_answer_item_doc.item_id),
            FormAnswerItemKind::try_from(form_answer_item_doc.kind)?,
        ))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FormAnswerItemKindDoc {
    String(String),
    Int(i32),
    ChooseOne(String),
    ChooseMany(Vec<String>),
    File(Vec<String>),
}

impl From<FormAnswerItemKind> for FormAnswerItemKindDoc {
    fn from(form_answer_item_kind: FormAnswerItemKind) -> Self {
        match form_answer_item_kind {
            FormAnswerItemKind::String(value) => FormAnswerItemKindDoc::String(value.value()),
            FormAnswerItemKind::Int(value) => FormAnswerItemKindDoc::Int(value.value()),
            FormAnswerItemKind::ChooseOne(value) => FormAnswerItemKindDoc::ChooseOne(value.value()),
            FormAnswerItemKind::ChooseMany(value) => {
                FormAnswerItemKindDoc::ChooseMany(value.value())
            }
            FormAnswerItemKind::File(value) => FormAnswerItemKindDoc::File(
                value
                    .value()
                    .into_iter()
                    .map(|id| id.value().to_string())
                    .collect(),
            ),
        }
    }
}

impl TryFrom<FormAnswerItemKindDoc> for FormAnswerItemKind {
    type Error = anyhow::Error;
    fn try_from(form_answer_item_kind_doc: FormAnswerItemKindDoc) -> Result<Self, Self::Error> {
        match form_answer_item_kind_doc {
            FormAnswerItemKindDoc::String(value) => {
                Ok(FormAnswerItemKind::String(FormAnswerItemString::new(value)))
            }
            FormAnswerItemKindDoc::Int(value) => {
                Ok(FormAnswerItemKind::Int(FormAnswerItemInt::new(value)))
            }
            FormAnswerItemKindDoc::ChooseOne(value) => Ok(FormAnswerItemKind::ChooseOne(
                FormAnswerItemChooseOne::new(value),
            )),
            FormAnswerItemKindDoc::ChooseMany(value) => Ok(FormAnswerItemKind::ChooseMany(
                FormAnswerItemChooseMany::new(value),
            )),
            FormAnswerItemKindDoc::File(value) => Ok(FormAnswerItemKind::File(
                FormAnswerItemFile::new(value.into_iter().map(FileId::try_from).collect::<Result<
                    Vec<_>,
                    _,
                >>(
                )?),
            )),
        }
    }
}

pub struct MongoFormAnswerRepository {
    collection: Collection<FormAnswerDoc>,
}

impl MongoFormAnswerRepository {
    pub fn new(mongodb: MongoDb) -> Self {
        Self {
            collection: mongodb.collection("form_answers"),
        }
    }
}

impl FormAnswerRepository for MongoFormAnswerRepository {
    async fn list(&self) -> Result<Vec<WithDate<FormAnswer>>, FormAnswerRepositoryError> {
        let form_answer_list = self
            .collection
            .aggregate(
                vec![
                    doc! { "$match": { "deleted_at": None::<String> } },
                    doc! { "$sort": { "created_at": 1 } },
                ],
                None,
            )
            .await
            .context("Failed to list form answers")?;
        let form_answers = form_answer_list
            .map(|doc| WithDate::try_from(bson::from_document::<FormAnswerDoc>(doc?)?))
            .try_collect()
            .await?;
        Ok(form_answers)
    }

    async fn create(&self, form_answer: FormAnswer) -> Result<(), FormAnswerRepositoryError> {
        let form_answer_doc = FormAnswerDoc::from(form_answer);
        self.collection
            .insert_one(form_answer_doc, None)
            .await
            .context("Failed to insert form answer")?;
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: FormAnswerId,
    ) -> Result<Option<WithDate<FormAnswer>>, FormAnswerRepositoryError> {
        let form_answer_doc = self
            .collection
            .find_one(doc! { "_id": id.value() }, None)
            .await
            .context("Failed to find form answer")?;
        Ok(form_answer_doc.map(WithDate::try_from).transpose()?)
    }

    async fn find_by_project_id(
        &self,
        project_id: ProjectId,
    ) -> Result<Vec<WithDate<FormAnswer>>, FormAnswerRepositoryError> {
        let form_answer_list = self
            .collection
            .aggregate(vec![
                doc! { "$match": { "project_id": project_id.value(),  "deleted_at": None::<String> } },
                doc! { "$sort": { "created_at": 1 } },
            ], None)
            .await
            .context("Failed to find form answers")?;
        let form_answers = form_answer_list
            .map(|doc| WithDate::try_from(bson::from_document::<FormAnswerDoc>(doc?)?))
            .try_collect()
            .await?;
        Ok(form_answers)
    }

    async fn find_by_form_id(
        &self,
        form_id: FormId,
    ) -> Result<Vec<WithDate<FormAnswer>>, FormAnswerRepositoryError> {
        let form_answer_list = self
            .collection
            .aggregate(
                vec![
                    doc! { "$match": { "form_id": form_id.value(), "deleted_at": None::<String> } },
                    doc! { "$sort": { "created_at": 1 } },
                ],
                None,
            )
            .await
            .context("Failed to find form answers")?;
        let form_answers = form_answer_list
            .map(|doc| WithDate::try_from(bson::from_document::<FormAnswerDoc>(doc?)?))
            .try_collect()
            .await?;
        Ok(form_answers)
    }

    async fn find_by_project_id_and_form_id(
        &self,
        project_id: ProjectId,
        form_id: FormId,
    ) -> Result<Option<WithDate<FormAnswer>>, FormAnswerRepositoryError> {
        let form_answer_doc = self
            .collection
            .find_one(
                doc! { "project_id": project_id.value(), "form_id": form_id.value() },
                None,
            )
            .await
            .context("Failed to find form answer")?;
        Ok(form_answer_doc.map(WithDate::try_from).transpose()?)
    }

    async fn update(&self, form_answer: FormAnswer) -> Result<(), FormAnswerRepositoryError> {
        let form_answer_doc = FormAnswerDoc::from(form_answer);
        self.collection
            .update_one(
                doc! { "_id": form_answer_doc._id,  "deleted_at": None::<String> },
                doc! { "$set":
                    doc! {
                        "project_id": form_answer_doc.project_id,
                        "form_d": form_answer_doc.form_id,
                        "items": bson::to_bson(&form_answer_doc.items).unwrap(),
                        "updated_at": form_answer_doc.updated_at,
                    }
                },
                None,
            )
            .await
            .context("Failed to update form")?;
        Ok(())
    }
}
