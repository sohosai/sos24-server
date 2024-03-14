use sos24_domain::entity::{
    common::date::WithDate,
    project::{
        Project, ProjectAttributes, ProjectCategory, ProjectGroupName, ProjectKanaGroupName,
        ProjectKanaTitle, ProjectTitle,
    },
    user::UserId,
};

use crate::interactor::project::ProjectUseCaseError;

use super::{FromEntity, ToEntity};

#[derive(Debug)]
pub struct CreateProjectDto {
    title: String,
    kana_title: String,
    group_name: String,
    kana_group_name: String,
    category: ProjectCategoryDto,
    attributes: i32,
    owner_id: String,
}

impl CreateProjectDto {
    pub fn new(
        title: String,
        kana_title: String,
        group_name: String,
        kana_group_name: String,
        category: ProjectCategoryDto,
        attributes: i32,
        owner_id: String,
    ) -> Self {
        Self {
            title,
            kana_title,
            group_name,
            kana_group_name,
            category,
            attributes,
            owner_id,
        }
    }
}

impl ToEntity for CreateProjectDto {
    type Entity = Project;
    type Error = ProjectUseCaseError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        Ok(Project::create(
            ProjectTitle::new(self.title),
            ProjectKanaTitle::new(self.kana_title),
            ProjectGroupName::new(self.group_name),
            ProjectKanaGroupName::new(self.kana_group_name),
            self.category.into_entity()?,
            ProjectAttributes::new(self.attributes),
            UserId::new(self.owner_id),
        ))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ProjectDto {
    pub id: String,
    pub index: i32,
    pub title: String,
    pub kana_title: String,
    pub group_name: String,
    pub kana_group_name: String,
    pub category: ProjectCategoryDto,
    pub attributes: i32,
    pub owner_id: String,
    pub sub_owner_id: Option<String>,
    pub remarks: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl FromEntity for ProjectDto {
    type Entity = WithDate<Project>;
    fn from_entity(entity: Self::Entity) -> Self {
        let project = entity.value.destruct();
        Self {
            id: project.id.value().to_string(),
            index: project.index.value(),
            title: project.title.value(),
            kana_title: project.kana_title.value(),
            group_name: project.group_name.value(),
            kana_group_name: project.kana_group_name.value(),
            category: ProjectCategoryDto::from_entity(project.category),
            attributes: project.attributes.value(),
            owner_id: project.owner_id.value(),
            sub_owner_id: project.sub_owner_id.map(|id| id.value()),
            remarks: project.remarks.map(|it| it.value()),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProjectCategoryDto {
    General,
    FoodsWithKitchen,
    FoodsWithoutKitchen,
    FoodsWithoutCooking,
    Stage1A,
    StageUniversityHall,
    StageUnited,
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
