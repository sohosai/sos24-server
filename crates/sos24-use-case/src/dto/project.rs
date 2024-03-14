use sos24_domain::entity::{
    project::{
        Project, ProjectAttributes, ProjectCategory, ProjectGroupName, ProjectKanaGroupName,
        ProjectKanaTitle, ProjectTitle,
    },
    user::UserId,
};

use crate::interactor::project::ProjectUseCaseError;

use super::ToEntity;

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

#[derive(Debug)]
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
