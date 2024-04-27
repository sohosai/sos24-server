use sos24_domain::entity::project::ProjectCategories;
use sos24_domain::entity::{common::date::WithDate, news::News};

use crate::project::dto::{ProjectAttributeDto, ProjectCategoryDto};
use crate::project::ProjectUseCaseError;
use crate::{FromEntity, ToEntity};

#[derive(Debug, PartialEq, Eq)]
pub struct NewsDto {
    pub id: String,
    pub title: String,
    pub body: String,
    pub attachments: Vec<String>,
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
            attachments: news
                .attachments
                .into_iter()
                .map(|file_id| file_id.value().to_string())
                .collect(),
            categories: Vec::from_entity(news.categories),
            attributes: Vec::from_entity(news.attributes),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
        }
    }
}

impl ToEntity for Vec<ProjectCategoryDto> {
    type Entity = ProjectCategories;
    type Error = ProjectUseCaseError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        let res = self
            .into_iter()
            .map(|category| match category {
                ProjectCategoryDto::General => ProjectCategories::GENERAL,
                ProjectCategoryDto::FoodsWithKitchen => ProjectCategories::FOODS_WITH_KITCHEN,
                ProjectCategoryDto::FoodsWithoutKitchen => ProjectCategories::FOODS_WITHOUT_KITCHEN,
                ProjectCategoryDto::FoodsWithoutCooking => ProjectCategories::FOODS_WITHOUT_COOKING,
                ProjectCategoryDto::Stage1A => ProjectCategories::STAGE_1A,
                ProjectCategoryDto::StageUniversityHall => ProjectCategories::STAGE_UNIVERSITY_HALL,
                ProjectCategoryDto::StageUnited => ProjectCategories::STAGE_UNITED,
            })
            .fold(ProjectCategories::empty(), |acc, category| acc | category);
        Ok(res)
    }
}

impl FromEntity for Vec<ProjectCategoryDto> {
    type Entity = ProjectCategories;
    fn from_entity(entity: Self::Entity) -> Self {
        entity
            .into_iter()
            .map(|category| match category {
                ProjectCategories::GENERAL => ProjectCategoryDto::General,
                ProjectCategories::FOODS_WITH_KITCHEN => ProjectCategoryDto::FoodsWithKitchen,
                ProjectCategories::FOODS_WITHOUT_KITCHEN => ProjectCategoryDto::FoodsWithoutKitchen,
                ProjectCategories::FOODS_WITHOUT_COOKING => ProjectCategoryDto::FoodsWithoutCooking,
                ProjectCategories::STAGE_1A => ProjectCategoryDto::Stage1A,
                ProjectCategories::STAGE_UNIVERSITY_HALL => ProjectCategoryDto::StageUniversityHall,
                ProjectCategories::STAGE_UNITED => ProjectCategoryDto::StageUnited,
                _ => panic!("unknown project category: {category:?}"),
            })
            .collect()
    }
}
