use anyhow::Context;
use futures_util::{StreamExt, TryStreamExt};
use mongodb::{
    bson::{self, doc},
    Collection,
};
use serde::{Deserialize, Serialize};
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
                    .map(FormAnswerItem::from)
                    .collect(),
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

impl From<FormAnswerItemDoc> for FormAnswerItem {
    fn from(form_answer_item_doc: FormAnswerItemDoc) -> Self {
        FormAnswerItem::new(
            FormItemId::new(form_answer_item_doc.item_id),
            form_answer_item_doc.kind.into(),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FormAnswerItemKindDoc {
    String(String),
    Int(i32),
    ChooseOne(String),
    ChooseMany(Vec<String>),
    File(String),
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
            FormAnswerItemKind::File(value) => FormAnswerItemKindDoc::File(value.value()),
        }
    }
}

impl From<FormAnswerItemKindDoc> for FormAnswerItemKind {
    fn from(form_answer_item_kind_doc: FormAnswerItemKindDoc) -> Self {
        match form_answer_item_kind_doc {
            FormAnswerItemKindDoc::String(value) => {
                FormAnswerItemKind::String(FormAnswerItemString::new(value))
            }
            FormAnswerItemKindDoc::Int(value) => {
                FormAnswerItemKind::Int(FormAnswerItemInt::new(value))
            }
            FormAnswerItemKindDoc::ChooseOne(value) => {
                FormAnswerItemKind::ChooseOne(FormAnswerItemChooseOne::new(value))
            }
            FormAnswerItemKindDoc::ChooseMany(value) => {
                FormAnswerItemKind::ChooseMany(FormAnswerItemChooseMany::new(value))
            }
            FormAnswerItemKindDoc::File(value) => {
                FormAnswerItemKind::File(FormAnswerItemFile::new(value))
            }
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
            .find(doc! { "deleted_at": None::<String> }, None)
            .await
            .context("Failed to list form answers")?;
        let form_answers = form_answer_list
            .map(|doc| WithDate::try_from(doc.context("Failed to fetch form answer list")?))
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
            .find(
                doc! { "project_id": project_id.value(), "deleted_at": None::<String> },
                None,
            )
            .await
            .context("Failed to find form answers")?;
        let form_answers = form_answer_list
            .map(|doc| WithDate::try_from(doc.context("Failed to fetch form answer")?))
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
            .find(
                doc! { "form_id": form_id.value(), "deleted_at": None::<String> },
                None,
            )
            .await
            .context("Failed to find form answers")?;
        let form_answers = form_answer_list
            .map(|doc| WithDate::try_from(doc.context("Failed to fetch form answer")?))
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
