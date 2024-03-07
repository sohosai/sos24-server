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
        Ok(News::new(
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
        Ok(News {
            id: NewsId::try_from(self.id)?,
            title: NewsTitle::new(self.title),
            body: NewsBody::new(self.body),
            categories: NewsCategories::new(self.categories),
        })
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
        Self {
            id: entity.value.id.value().to_string(),
            title: entity.value.title.value(),
            body: entity.value.body.value(),
            categories: entity.value.categories.value(),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
        }
    }
}
