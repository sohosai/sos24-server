use sos24_domain::entity::news::NewsCategories;
use sos24_domain::entity::{
    common::date::WithDate,
    news::{News, NewsBody, NewsId, NewsTitle},
};

use crate::dto::project::{ProjectAttributeDto, ProjectCategoryDto};
use crate::interactor::news::NewsUseCaseError;

use super::{FromEntity, ToEntity};

#[derive(Debug)]
pub struct CreateNewsDto {
    pub title: String,
    pub body: String,
    pub categories: Vec<ProjectCategoryDto>,
    pub attributes: Vec<ProjectAttributeDto>,
}

impl CreateNewsDto {
    pub fn new(
        title: String,
        body: String,
        categories: Vec<ProjectCategoryDto>,
        attributes: Vec<ProjectAttributeDto>,
    ) -> Self {
        Self {
            title,
            body,
            categories,
            attributes,
        }
    }
}

impl ToEntity for CreateNewsDto {
    type Entity = News;
    type Error = NewsUseCaseError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        Ok(News::create(
            NewsTitle::new(self.title),
            NewsBody::new(self.body),
            self.categories.into_entity()?,
            self.attributes.into_entity()?,
        ))
    }
}

#[derive(Debug)]
pub struct UpdateNewsDto {
    pub id: String,
    pub title: String,
    pub body: String,
    pub categories: Vec<ProjectCategoryDto>,
    pub attributes: Vec<ProjectAttributeDto>,
}

impl UpdateNewsDto {
    pub fn new(
        id: String,
        title: String,
        body: String,
        categories: Vec<ProjectCategoryDto>,
        attributes: Vec<ProjectAttributeDto>,
    ) -> Self {
        Self {
            id,
            title,
            body,
            categories,
            attributes,
        }
    }
}

impl ToEntity for UpdateNewsDto {
    type Entity = News;
    type Error = NewsUseCaseError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        Ok(News::new(
            NewsId::try_from(self.id)?,
            NewsTitle::new(self.title),
            NewsBody::new(self.body),
            self.categories.into_entity()?,
            self.attributes.into_entity()?,
        ))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NewsDto {
    pub id: String,
    pub title: String,
    pub body: String,
    pub categories: Vec<ProjectCategoryDto>,
    pub attributes: Vec<ProjectAttributeDto>,
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
            categories: Vec::from_entity(news.categories),
            attributes: Vec::from_entity(news.attributes),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
        }
    }
}

impl ToEntity for Vec<ProjectCategoryDto> {
    type Entity = NewsCategories;
    type Error = NewsUseCaseError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        let res = self
            .into_iter()
            .map(|category| match category {
                ProjectCategoryDto::General => NewsCategories::GENERAL,
                ProjectCategoryDto::FoodsWithKitchen => NewsCategories::FOODS_WITH_KITCHEN,
                ProjectCategoryDto::FoodsWithoutKitchen => NewsCategories::FOODS_WITHOUT_KITCHEN,
                ProjectCategoryDto::FoodsWithoutCooking => NewsCategories::FOODS_WITHOUT_COOKING,
                ProjectCategoryDto::Stage1A => NewsCategories::STAGE_1A,
                ProjectCategoryDto::StageUniversityHall => NewsCategories::STAGE_UNIVERSITY_HALL,
                ProjectCategoryDto::StageUnited => NewsCategories::STAGE_UNITED,
            })
            .fold(NewsCategories::empty(), |acc, category| acc | category);
        Ok(res)
    }
}

impl FromEntity for Vec<ProjectCategoryDto> {
    type Entity = NewsCategories;
    fn from_entity(entity: Self::Entity) -> Self {
        entity
            .into_iter()
            .map(|category| match category {
                NewsCategories::GENERAL => ProjectCategoryDto::General,
                NewsCategories::FOODS_WITH_KITCHEN => ProjectCategoryDto::FoodsWithKitchen,
                NewsCategories::FOODS_WITHOUT_KITCHEN => ProjectCategoryDto::FoodsWithoutKitchen,
                NewsCategories::FOODS_WITHOUT_COOKING => ProjectCategoryDto::FoodsWithoutCooking,
                NewsCategories::STAGE_1A => ProjectCategoryDto::Stage1A,
                NewsCategories::STAGE_UNIVERSITY_HALL => ProjectCategoryDto::StageUniversityHall,
                NewsCategories::STAGE_UNITED => ProjectCategoryDto::StageUnited,
                _ => panic!("unknown project category: {category:?}"),
            })
            .collect()
    }
}
