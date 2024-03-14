use serde::{Deserialize, Serialize};
use sos24_use_case::dto::project::{CreateProjectDto, ProjectCategoryDto, ProjectDto};

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
pub struct Project {
    id: String,
    index: i32,
    title: String,
    kana_title: String,
    group_name: String,
    kana_group_name: String,
    category: ProjectCategory,
    attributes: i32,
    owner_id: String,
    sub_owner_id: Option<String>,
    remarks: Option<String>,
    created_at: String,
    updated_at: String,
    deleted_at: Option<String>,
}

impl From<ProjectDto> for Project {
    fn from(project: ProjectDto) -> Self {
        Project {
            id: project.id,
            index: project.index,
            title: project.title,
            kana_title: project.kana_title,
            group_name: project.group_name,
            kana_group_name: project.kana_group_name,
            category: ProjectCategory::from(project.category),
            attributes: project.attributes,
            owner_id: project.owner_id,
            sub_owner_id: project.sub_owner_id,
            remarks: project.remarks,
            created_at: project.created_at.to_rfc3339(),
            updated_at: project.updated_at.to_rfc3339(),
            deleted_at: project.deleted_at.map(|it| it.to_rfc3339()),
        }
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

impl From<ProjectCategoryDto> for ProjectCategory {
    fn from(value: ProjectCategoryDto) -> Self {
        match value {
            ProjectCategoryDto::General => ProjectCategory::General,
            ProjectCategoryDto::FoodsWithKitchen => ProjectCategory::FoodsWithKitchen,
            ProjectCategoryDto::FoodsWithoutKitchen => ProjectCategory::FoodsWithoutKitchen,
            ProjectCategoryDto::FoodsWithoutCooking => ProjectCategory::FoodsWithoutCooking,
            ProjectCategoryDto::Stage1A => ProjectCategory::Stage1A,
            ProjectCategoryDto::StageUniversityHall => ProjectCategory::StageUniversityHall,
            ProjectCategoryDto::StageUnited => ProjectCategory::StageUnited,
        }
    }
}
