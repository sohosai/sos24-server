use chrono_tz::Asia::Tokyo;
use serde::{Deserialize, Serialize};

use sos24_use_case::dto::project::ProjectAttributeDto;
use sos24_use_case::dto::{
    project::{CreateProjectDto, ProjectCategoryDto, ProjectDto, UpdateProjectDto},
    user::UserDto,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProject {
    title: String,
    kana_title: String,
    group_name: String,
    kana_group_name: String,
    category: ProjectCategory,
    attributes: Vec<ProjectAttribute>,
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
            project
                .attributes
                .into_iter()
                .map(ProjectAttributeDto::from)
                .collect(),
            owner_id,
        )
    }
}

#[derive(Debug, Serialize)]
pub struct CreatedProject {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProject {
    title: String,
    kana_title: String,
    group_name: String,
    kana_group_name: String,
    category: ProjectCategory,
    attributes: Vec<ProjectAttribute>,
    remarks: Option<String>,
}

pub trait ConvertToUpdateProjectDto {
    fn to_update_project_dto(self) -> UpdateProjectDto;
}

impl ConvertToUpdateProjectDto for (UpdateProject, String) {
    fn to_update_project_dto(self) -> UpdateProjectDto {
        let (project, id) = self;
        UpdateProjectDto::new(
            id,
            project.title,
            project.kana_title,
            project.group_name,
            project.kana_group_name,
            ProjectCategoryDto::from(project.category),
            project
                .attributes
                .into_iter()
                .map(ProjectAttributeDto::from)
                .collect(),
            project.remarks,
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
    attributes: Vec<ProjectAttribute>,
    owner_id: String,
    owner_name: String,
    owner_email: String,
    sub_owner_id: Option<String>,
    sub_owner_name: Option<String>,
    sub_owner_email: Option<String>,
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
            attributes: project
                .attributes
                .into_iter()
                .map(ProjectAttribute::from)
                .collect(),
            owner_id: project.owner_id,
            owner_name: project.owner_name,
            owner_email: project.owner_email,
            sub_owner_id: project.sub_owner_id,
            sub_owner_name: project.sub_owner_name,
            sub_owner_email: project.sub_owner_email,
            remarks: project.remarks,
            created_at: project.created_at.to_rfc3339(),
            updated_at: project.updated_at.to_rfc3339(),
            deleted_at: project.deleted_at.map(|it| it.to_rfc3339()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ProjectToBeExported {
    #[serde(rename(serialize = "企画番号"))]
    id: i32,
    #[serde(rename(serialize = "企画名"))]
    title: String,
    #[serde(rename(serialize = "きかくめい"))]
    kana_title: String,
    #[serde(rename(serialize = "企画団体名"))]
    group_name: String,
    #[serde(rename(serialize = "企画責任者"))]
    owner_name: String,
    #[serde(rename(serialize = "企画責任者電話番号"))]
    owner_phone_number: String,
    #[serde(rename(serialize = "企画責任者メールアドレス"))]
    owner_email: String,
    #[serde(rename(serialize = "副企画責任者"))]
    sub_owner_name: Option<String>,
    #[serde(rename(serialize = "副企画責任者メールアドレス"))]
    sub_owner_email: Option<String>,
    #[serde(rename(serialize = "副企画責任者電話番号"))]
    sub_owner_phone_number: Option<String>,
    #[serde(rename(serialize = "企画区分"))]
    category: String,
    #[serde(rename(serialize = "企画属性"))]
    attributes: String,
    #[serde(rename(serialize = "備考"))]
    remark: Option<String>,
    #[serde(rename(serialize = "作成日時"))]
    created_at: String,
}

impl From<(ProjectDto, UserDto, Option<UserDto>)> for ProjectToBeExported {
    fn from((project, owner, sub_owner): (ProjectDto, UserDto, Option<UserDto>)) -> Self {
        ProjectToBeExported {
            id: project.index,
            owner_name: owner.name,
            sub_owner_name: sub_owner.as_ref().map(|it| it.name.clone()),
            owner_email: owner.email,
            sub_owner_email: sub_owner.as_ref().map(|it| it.email.clone()),
            owner_phone_number: owner.phone_number,
            sub_owner_phone_number: sub_owner.map(|it| it.phone_number.clone()),
            title: project.title,
            kana_title: project.kana_title,
            group_name: project.group_name,
            category: project.category.to_string(),
            attributes: project
                .attributes
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(";"),
            remark: project.remarks,
            created_at: project.created_at.with_timezone(&Tokyo).to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectSummary {
    id: String,
    index: i32,
    title: String,
    category: ProjectCategory,
    attributes: Vec<ProjectAttribute>,
    owner_id: String,
    owner_name: String,
    owner_email: String,
}

impl From<ProjectDto> for ProjectSummary {
    fn from(project: ProjectDto) -> Self {
        ProjectSummary {
            id: project.id,
            index: project.index,
            title: project.title,
            category: ProjectCategory::from(project.category),
            attributes: project
                .attributes
                .into_iter()
                .map(ProjectAttribute::from)
                .collect(),
            owner_id: project.owner_id,
            owner_name: project.owner_name,
            owner_email: project.owner_email,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectCategory {
    General,
    FoodsWithKitchen,
    FoodsWithoutKitchen,
    FoodsWithoutCooking,
    #[serde(rename = "stage_1a")]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectAttribute {
    Academic,
    Art,
    Official,
    Inside,
    Outside,
}

impl From<ProjectAttribute> for ProjectAttributeDto {
    fn from(value: ProjectAttribute) -> Self {
        match value {
            ProjectAttribute::Academic => ProjectAttributeDto::Academic,
            ProjectAttribute::Art => ProjectAttributeDto::Art,
            ProjectAttribute::Official => ProjectAttributeDto::Official,
            ProjectAttribute::Inside => ProjectAttributeDto::Inside,
            ProjectAttribute::Outside => ProjectAttributeDto::Outside,
        }
    }
}

impl From<ProjectAttributeDto> for ProjectAttribute {
    fn from(value: ProjectAttributeDto) -> Self {
        match value {
            ProjectAttributeDto::Academic => ProjectAttribute::Academic,
            ProjectAttributeDto::Art => ProjectAttribute::Art,
            ProjectAttributeDto::Official => ProjectAttribute::Official,
            ProjectAttributeDto::Inside => ProjectAttribute::Inside,
            ProjectAttributeDto::Outside => ProjectAttribute::Outside,
        }
    }
}
