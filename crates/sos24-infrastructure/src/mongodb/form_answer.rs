use anyhow::Context;
use mongodb::{bson, Collection};
use serde::{Deserialize, Serialize};
use sos24_domain::{
    entity::{
        common::date::WithDate,
        form::FormId,
        form_answer::{FormAnswer, FormAnswerId, FormAnswerItem, FormAnswerItemKind},
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
        }
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
        todo!()
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
        todo!()
    }

    async fn find_by_project_id(
        &self,
        project_id: ProjectId,
    ) -> Result<Vec<WithDate<FormAnswer>>, FormAnswerRepositoryError> {
        todo!()
    }

    async fn find_by_form_id(
        &self,
        form_id: FormId,
    ) -> Result<Vec<WithDate<FormAnswer>>, FormAnswerRepositoryError> {
        todo!()
    }

    async fn update(&self, form_answer: FormAnswer) -> Result<(), FormAnswerRepositoryError> {
        todo!()
    }
}
