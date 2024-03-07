use std::convert::Infallible;

use sos24_domain::entity::{
    common::date::WithDate,
    news::{News, NewsBody, NewsCategories, NewsId, NewsTitle},
};

use crate::error::{news::NewsError, Result};

use super::{FromEntity, ToEntity};

#[derive(Debug)]
pub struct CreateNewsDto {
    pub title: String,
    pub body: String,
    pub categories: i32,
}

impl CreateNewsDto {
    pub fn new(title: String, body: String, categories: i32) -> Self {
        Self {
            title,
            body,
            categories,
        }
    }
}

impl ToEntity for CreateNewsDto {
    type Entity = News;
    type Error = Infallible;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        Ok(News::create(
            NewsTitle::new(self.title),
            NewsBody::new(self.body),
            NewsCategories::new(self.categories),
        ))
    }
}

#[derive(Debug)]
pub struct UpdateNewsDto {
    pub id: String,
    pub title: String,
    pub body: String,
    pub categories: i32,
}

impl UpdateNewsDto {
    pub fn new(id: String, title: String, body: String, categories: i32) -> Self {
        Self {
            id,
            title,
            body,
            categories,
        }
    }
}

impl ToEntity for UpdateNewsDto {
    type Entity = News;
    type Error = NewsError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        Ok(News::new(
            NewsId::try_from(self.id)?,
            NewsTitle::new(self.title),
            NewsBody::new(self.body),
            NewsCategories::new(self.categories),
        ))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NewsDto {
    pub id: String,
    pub title: String,
    pub body: String,
    pub categories: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl FromEntity for NewsDto {
    type Entity = WithDate<News>;
    fn from_entity(entity: Self::Entity) -> Self {
        let news = entity.value.destruct();
        Self {
            id: news.id.value().to_string(),
            title: news.title.value(),
            body: news.body.value(),
            categories: news.categories.value(),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
        }
    }
}
