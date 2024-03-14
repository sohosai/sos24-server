use serde::{Deserialize, Serialize};
use sos24_use_case::dto::project::{CreateProjectDto, ProjectCategoryDto};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProject {
    title: String,
    kana_title: String,
    group_name: String,
    kana_group_name: String,
    category: ProjectCategory,
    attributes: i32,
}

pub trait ConvertToCreateProjectDto {
    fn to_create_project_dto(self) -> CreateProjectDto;
}

impl ConvertToCreateProjectDto for (CreateProject, String) {
    fn to_create_project_dto(self) -> CreateProjectDto {
        let (project, owner_id) = self;
        CreateProjectDto::new(
            project.title,
            project.kana_title,
            project.group_name,
            project.kana_group_name,
            ProjectCategoryDto::from(project.category),
            project.attributes,
            owner_id,
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProjectCategory {
    General,
    FoodsWithKitchen,
    FoodsWithoutKitchen,
    FoodsWithoutCooking,
    Stage1A,
    StageUniversityHall,
    StageUnited,
}

impl From<ProjectCategory> for ProjectCategoryDto {
    fn from(value: ProjectCategory) -> Self {
        match value {
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
