use anyhow::Context;
use futures_util::{StreamExt, TryStreamExt};
use mongodb::{
    bson::{self, doc},
    Collection,
};
use serde::{Deserialize, Serialize};

use sos24_domain::entity::{common::datetime::DateTime, file_data::FileId};
use sos24_domain::{
    entity::{
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

use crate::shared::mongodb::MongoDb;

#[derive(Debug, Serialize, Deserialize)]
pub struct FormAnswerDoc {
    _id: String,
    project_id: String,
    form_id: String,
    items: Vec<FormAnswerItemDoc>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<FormAnswer> for FormAnswerDoc {
    fn from(form_answer: FormAnswer) -> Self {
        let form_answer = form_answer.destruct();
        Self {
            _id: form_answer.id.value().to_string(),
            project_id: form_answer.project_id.value().to_string(),
            form_id: form_answer.form_id.value().to_string(),
            items: form_answer
                .items
                .into_iter()
                .map(FormAnswerItemDoc::from)
                .collect(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}

impl TryFrom<FormAnswerDoc> for FormAnswer {
    type Error = anyhow::Error;
    fn try_from(form_answer_doc: FormAnswerDoc) -> Result<Self, Self::Error> {
        Ok(FormAnswer::new(
            FormAnswerId::try_from(form_answer_doc._id)?,
            ProjectId::try_from(form_answer_doc.project_id)?,
            FormId::try_from(form_answer_doc.form_id)?,
            form_answer_doc
                .items
                .into_iter()
                .map(FormAnswerItem::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            DateTime::new(form_answer_doc.created_at),
            DateTime::new(form_answer_doc.updated_at),
        ))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormAnswerItemDoc {
    item_id: String,
    kind: FormAnswerItemKindDoc,
}

impl From<FormAnswerItem> for FormAnswerItemDoc {
    fn from(form_answer_item: FormAnswerItem) -> Self {
        let form_answer_item = form_answer_item.destruct();
        Self {
            item_id: form_answer_item.item_id.value().to_string(),
            kind: form_answer_item.kind.into(),
        }
    }
}

impl TryFrom<FormAnswerItemDoc> for FormAnswerItem {
    type Error = anyhow::Error;
    fn try_from(form_answer_item_doc: FormAnswerItemDoc) -> Result<Self, Self::Error> {
        Ok(FormAnswerItem::new(
            FormItemId::try_from(form_answer_item_doc.item_id)?,
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
    async fn list(&self) -> Result<Vec<FormAnswer>, FormAnswerRepositoryError> {
        tracing::info!("申請回答一覧を取得します");

        let form_answer_list = self
            .collection
            .aggregate(vec![
                doc! { "$match": { "deleted_at": None::<String> } },
                doc! { "$sort": { "updated_at": 1 } },
            ])
            .await
            .context("Failed to list form answers")?;
        let form_answers = form_answer_list
            .map(|doc| FormAnswer::try_from(bson::from_document::<FormAnswerDoc>(doc?)?))
            .try_collect()
            .await?;

        tracing::info!("申請回答一覧を取得しました");
        Ok(form_answers)
    }

    async fn create(&self, form_answer: FormAnswer) -> Result<(), FormAnswerRepositoryError> {
        tracing::info!("申請回答を作成します");

        let form_answer_doc = FormAnswerDoc::from(form_answer);
        self.collection
            .insert_one(form_answer_doc)
            .await
            .context("Failed to insert form answer")?;

        tracing::info!("申請回答を作成しました");
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: FormAnswerId,
    ) -> Result<Option<FormAnswer>, FormAnswerRepositoryError> {
        tracing::info!("申請回答を取得します: {id:?}");

        let form_answer_doc = self
            .collection
            .find_one(doc! { "_id": id.clone().value().to_string() })
            .await
            .context("Failed to find form answer")?;

        tracing::info!("申請回答を取得しました: {id:?}");
        Ok(form_answer_doc.map(FormAnswer::try_from).transpose()?)
    }

    async fn find_by_project_id(
        &self,
        project_id: ProjectId,
    ) -> Result<Vec<FormAnswer>, FormAnswerRepositoryError> {
        tracing::info!("企画の申請回答を取得します: {project_id:?}");

        let form_answer_list = self
            .collection
            .aggregate(vec![
                doc! { "$match": { "project_id": project_id.clone().value().to_string(),  "deleted_at": None::<String> } },
                doc! { "$sort": { "updated_at": 1 } },
            ])
            .await
            .context("Failed to find form answers")?;
        let form_answers = form_answer_list
            .map(|doc| FormAnswer::try_from(bson::from_document::<FormAnswerDoc>(doc?)?))
            .try_collect()
            .await?;

        tracing::info!("企画の申請回答を取得しました: {project_id:?}");
        Ok(form_answers)
    }

    async fn find_by_form_id(
        &self,
        form_id: FormId,
    ) -> Result<Vec<FormAnswer>, FormAnswerRepositoryError> {
        tracing::info!("申請の回答を取得します: {form_id:?}");

        let form_answer_list = self
            .collection
            .aggregate(
                vec![
                    doc! { "$match": { "form_id": form_id.clone().value().to_string(), "deleted_at": None::<String> } },
                    doc! { "$sort": { "updated_at": 1 } },
                ],
            )
            .await
            .context("Failed to find form answers")?;
        let form_answers = form_answer_list
            .map(|doc| FormAnswer::try_from(bson::from_document::<FormAnswerDoc>(doc?)?))
            .try_collect()
            .await?;

        tracing::info!("申請の回答を取得しました: {form_id:?}");
        Ok(form_answers)
    }

    async fn find_by_project_id_and_form_id(
        &self,
        project_id: ProjectId,
        form_id: FormId,
    ) -> Result<Option<FormAnswer>, FormAnswerRepositoryError> {
        tracing::info!("企画の申請の回答を取得します: {project_id:?}, {form_id:?}");

        let form_answer_doc = self
            .collection
            .find_one(
                doc! { "project_id": project_id.clone().value().to_string(), "form_id": form_id.clone().value().to_string() },
            )
            .await
            .context("Failed to find form answer")?;

        tracing::info!("企画の申請の回答を取得しました: {project_id:?}, {form_id:?}");
        Ok(form_answer_doc.map(FormAnswer::try_from).transpose()?)
    }

    async fn update(&self, form_answer: FormAnswer) -> Result<(), FormAnswerRepositoryError> {
        tracing::info!("申請回答を更新します");

        let form_answer_doc = FormAnswerDoc::from(form_answer);
        self.collection
            .update_one(
                doc! { "_id": form_answer_doc._id,  "deleted_at": None::<String> },
                doc! { "$set":
                    doc! {
                        "project_id": bson::to_bson(&form_answer_doc.project_id).unwrap(),
                        "form_id": bson::to_bson(&form_answer_doc.form_id).unwrap(),
                        "items": bson::to_bson(&form_answer_doc.items).unwrap(),
                        "updated_at": bson::to_bson(&form_answer_doc.updated_at).unwrap(),
                    }
                },
            )
            .await
            .context("Failed to update form")?;

        tracing::info!("申請回答を更新しました");
        Ok(())
    }

    async fn delete_by_project_id(&self, id: ProjectId) -> Result<(), FormAnswerRepositoryError> {
        self.collection
            .update_many(
                doc! { "project_id": id.value().to_string(),  "deleted_at": None::<String> },
                doc! { "$set": { "deleted_at": bson::to_bson(&chrono::Utc::now()).unwrap() } },
            )
            .await
            .context("Failed to delete form by project id")?;
        Ok(())
    }
}
