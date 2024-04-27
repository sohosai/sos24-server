use std::fmt;
use std::fmt::Formatter;

use sos24_domain::entity::user::User;
use sos24_domain::entity::{
    common::date::WithDate,
    project::{Project, ProjectAttributes, ProjectCategory},
};

use crate::project::ProjectUseCaseError;
use crate::{FromEntity, ToEntity};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ProjectDto {
    pub id: String,
    pub index: i32,
    pub title: String,
    pub kana_title: String,
    pub group_name: String,
    pub kana_group_name: String,
    pub category: ProjectCategoryDto,
    pub attributes: Vec<ProjectAttributeDto>,
    pub owner_id: String,
    pub owner_name: String,
    pub owner_email: String,
    pub sub_owner_id: Option<String>,
    pub sub_owner_name: Option<String>,
    pub sub_owner_email: Option<String>,
    pub remarks: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl FromEntity for ProjectDto {
    type Entity = (WithDate<Project>, WithDate<User>, Option<WithDate<User>>);
    fn from_entity(entity: Self::Entity) -> Self {
        let (project_entity, owner_entity, sub_owner_entity) = entity;
        let project = project_entity.value.destruct();
        let owner = owner_entity.value.destruct();
        let sub_owner = sub_owner_entity.map(|it| it.value.destruct());
        let (sub_owner_name, sub_owner_email) = match sub_owner {
            Some(user) => (Some(user.name.value()), Some(user.email.value())),
            None => (None, None),
        };

        Self {
            id: project.id.value().to_string(),
            index: project.index.value(),
            title: project.title.value(),
            kana_title: project.kana_title.value(),
            group_name: project.group_name.value(),
            kana_group_name: project.kana_group_name.value(),
            category: ProjectCategoryDto::from_entity(project.category),
            attributes: Vec::from_entity(project.attributes),
            owner_id: project.owner_id.value(),
            owner_name: owner.name.value(),
            owner_email: owner.email.value(),
            sub_owner_id: project.sub_owner_id.map(|id| id.value()),
            sub_owner_name,
            sub_owner_email,
            remarks: project.remarks.map(|it| it.value()),
            created_at: project_entity.created_at,
            updated_at: project_entity.updated_at,
            deleted_at: project_entity.deleted_at,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ProjectCategoryDto {
    General,
    FoodsWithKitchen,
    FoodsWithoutKitchen,
    FoodsWithoutCooking,
    Stage1A,
    StageUniversityHall,
    StageUnited,
}

impl fmt::Display for ProjectCategoryDto {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ProjectCategoryDto::General => write!(f, "普通企画"),
            ProjectCategoryDto::FoodsWithKitchen => write!(f, "調理企画（仕込み場が必要）"),
            ProjectCategoryDto::FoodsWithoutKitchen => write!(f, "調理企画（仕込み場が不要）"),
            ProjectCategoryDto::FoodsWithoutCooking => write!(f, "既成食品販売企画"),
            ProjectCategoryDto::Stage1A => write!(f, "ステージ企画(1Aステージ)"),
            ProjectCategoryDto::StageUniversityHall => write!(f, "ステージ企画(大学会館ステージ)"),
            ProjectCategoryDto::StageUnited => write!(f, "ステージ企画(UNITEDステージ)"),
        }
    }
}

impl ToEntity for ProjectCategoryDto {
    type Entity = ProjectCategory;
    type Error = ProjectUseCaseError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        match self {
            ProjectCategoryDto::General => Ok(ProjectCategory::General),
            ProjectCategoryDto::FoodsWithKitchen => Ok(ProjectCategory::FoodsWithKitchen),
            ProjectCategoryDto::FoodsWithoutKitchen => Ok(ProjectCategory::FoodsWithoutKitchen),
            ProjectCategoryDto::FoodsWithoutCooking => Ok(ProjectCategory::FoodsWithoutCooking),
            ProjectCategoryDto::Stage1A => Ok(ProjectCategory::Stage1A),
            ProjectCategoryDto::StageUniversityHall => Ok(ProjectCategory::StageUniversityHall),
            ProjectCategoryDto::StageUnited => Ok(ProjectCategory::StageUnited),
        }
    }
}

impl FromEntity for ProjectCategoryDto {
    type Entity = ProjectCategory;
    fn from_entity(entity: Self::Entity) -> Self {
        match entity {
            ProjectCategory::General => ProjectCategoryDto::General,
            ProjectCategory::FoodsWithKitchen => ProjectCategoryDto::FoodsWithKitchen,
            ProjectCategory::FoodsWithoutKitchen => ProjectCategoryDto::FoodsWithoutKitchen,
            ProjectCategory::FoodsWithoutCooking => ProjectCategoryDto::FoodsWithoutCooking,
            ProjectCategory::Stage1A => ProjectCategoryDto::Stage1A,
            ProjectCategory::StageUniversityHall => ProjectCategoryDto::StageUniversityHall,
            ProjectCategory::StageUnited => ProjectCategoryDto::StageUnited,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectAttributeDto {
    Academic,
    Art,
    Official,
    Inside,
    Outside,
}

impl fmt::Display for ProjectAttributeDto {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ProjectAttributeDto::Academic => write!(f, "学術企画"),
            ProjectAttributeDto::Art => write!(f, "芸術際企画"),
            ProjectAttributeDto::Official => write!(f, "委員会開催企画"),
            ProjectAttributeDto::Inside => write!(f, "屋内企画"),
            ProjectAttributeDto::Outside => write!(f, "屋外企画"),
        }
    }
}

impl ToEntity for Vec<ProjectAttributeDto> {
    type Entity = ProjectAttributes;
    type Error = ProjectUseCaseError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        let res = self
            .into_iter()
            .map(|attribute| match attribute {
                ProjectAttributeDto::Academic => ProjectAttributes::ACADEMIC,
                ProjectAttributeDto::Art => ProjectAttributes::ART,
                ProjectAttributeDto::Official => ProjectAttributes::OFFICIAL,
                ProjectAttributeDto::Inside => ProjectAttributes::INSIDE,
                ProjectAttributeDto::Outside => ProjectAttributes::OUTSIDE,
            })
            .fold(ProjectAttributes::empty(), |acc, attribute| acc | attribute);
        Ok(res)
    }
}

impl FromEntity for Vec<ProjectAttributeDto> {
    type Entity = ProjectAttributes;

    fn from_entity(entity: Self::Entity) -> Self {
        entity
            .into_iter()
            .map(|attribute| match attribute {
                ProjectAttributes::ACADEMIC => ProjectAttributeDto::Academic,
                ProjectAttributes::ART => ProjectAttributeDto::Art,
                ProjectAttributes::OFFICIAL => ProjectAttributeDto::Official,
                ProjectAttributes::INSIDE => ProjectAttributeDto::Inside,
                ProjectAttributes::OUTSIDE => ProjectAttributeDto::Outside,
                _ => panic!("unknown project attribute: {attribute:?}"),
            })
            .collect()
    }
}
