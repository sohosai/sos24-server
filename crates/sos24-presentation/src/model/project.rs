use chrono_tz::Asia::Tokyo;
use serde::{Deserialize, Serialize};

use sos24_use_case::project::{
    dto::{
        ProjectAttributeDto, ProjectAttributesDto, ProjectCategoriesDto, ProjectCategoryDto,
        ProjectDto,
    },
    interactor::{create::CreateProjectCommand, update::UpdateProjectCommand},
};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateProject {
    title: String,
    kana_title: String,
    group_name: String,
    kana_group_name: String,
    category: ProjectCategory,
    attributes: ProjectAttributes,
}

pub trait ConvertToCreateProjectDto {
    fn to_create_project_dto(self) -> CreateProjectCommand;
}

impl ConvertToCreateProjectDto for (CreateProject, String) {
    fn to_create_project_dto(self) -> CreateProjectCommand {
        let (project, owner_id) = self;
        CreateProjectCommand {
            title: project.title,
            kana_title: project.kana_title,
            group_name: project.group_name,
            kana_group_name: project.kana_group_name,
            category: ProjectCategoryDto::from(project.category),
            attributes: ProjectAttributesDto::from(project.attributes),
            owner_id,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedProject {
    #[schema(format = "uuid")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateProject {
    title: String,
    kana_title: String,
    group_name: String,
    kana_group_name: String,
    category: ProjectCategory,
    attributes: ProjectAttributes,
    remarks: Option<String>,
}

pub trait ConvertToUpdateProjectDto {
    fn to_update_project_dto(self) -> UpdateProjectCommand;
}

impl ConvertToUpdateProjectDto for (UpdateProject, String) {
    fn to_update_project_dto(self) -> UpdateProjectCommand {
        let (project, id) = self;
        UpdateProjectCommand {
            id,
            title: project.title,
            kana_title: project.kana_title,
            group_name: project.group_name,
            kana_group_name: project.kana_group_name,
            category: ProjectCategoryDto::from(project.category),
            attributes: ProjectAttributesDto::from(project.attributes),
            remarks: project.remarks,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Project {
    #[schema(format = "uuid")]
    id: String,
    index: i32,
    title: String,
    kana_title: String,
    group_name: String,
    kana_group_name: String,
    category: ProjectCategory,
    attributes: ProjectAttributes,
    owner_id: String,
    owner_name: String,
    owner_email: String,
    sub_owner_id: Option<String>,
    sub_owner_name: Option<String>,
    sub_owner_email: Option<String>,
    remarks: Option<String>,
    #[schema(format = "date-time")]
    created_at: String,
    #[schema(format = "date-time")]
    updated_at: String,
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
            attributes: ProjectAttributes::from(project.attributes),
            owner_id: project.owner_id,
            owner_name: project.owner_name,
            owner_email: project.owner_email,
            sub_owner_id: project.sub_owner_id,
            sub_owner_name: project.sub_owner_name,
            sub_owner_email: project.sub_owner_email,
            remarks: project.remarks,
            created_at: project.created_at.to_rfc3339(),
            updated_at: project.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ProjectToBeExported {
    #[serde(rename(serialize = "企画番号"))]
    id: i32,
    #[serde(rename(serialize = "企画名"))]
    title: String,
    #[serde(rename(serialize = "企画名（ふりがな）"))]
    kana_title: String,
    #[serde(rename(serialize = "企画団体名"))]
    group_name: String,
    #[serde(rename(serialize = "企画団体名（ふりがな）"))]
    kana_group_name: String,
    #[serde(rename(serialize = "企画責任者"))]
    owner_name: String,
    #[serde(rename(serialize = "企画責任者メールアドレス"))]
    owner_email: String,
    #[serde(rename(serialize = "企画責任者電話番号"))]
    owner_phone_number: String,
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
    remarks: Option<String>,
    #[serde(rename(serialize = "作成日時"))]
    created_at: String,
}

impl From<ProjectDto> for ProjectToBeExported {
    fn from(project: ProjectDto) -> Self {
        ProjectToBeExported {
            id: project.index,
            title: project.title,
            kana_title: project.kana_title,
            group_name: project.group_name,
            kana_group_name: project.kana_group_name,
            owner_name: project.owner_name,
            owner_email: project.owner_email,
            owner_phone_number: project.owner_phone_number,
            sub_owner_name: project.sub_owner_name,
            sub_owner_email: project.sub_owner_email,
            sub_owner_phone_number: project.sub_owner_phone_number,
            category: project.category.to_string(),
            attributes: project
                .attributes
                .0
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(";"),
            remarks: project.remarks,
            created_at: project.created_at.with_timezone(&Tokyo).to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProjectSummary {
    #[schema(format = "uuid")]
    id: String,
    index: i32,
    title: String,
    category: ProjectCategory,
    attributes: ProjectAttributes,
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
            attributes: ProjectAttributes::from(project.attributes),
            owner_id: project.owner_id,
            owner_name: project.owner_name,
            owner_email: project.owner_email,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProjectCategories(pub Vec<ProjectCategory>);

impl From<ProjectCategories> for ProjectCategoriesDto {
    fn from(value: ProjectCategories) -> Self {
        ProjectCategoriesDto(value.0.into_iter().map(ProjectCategoryDto::from).collect())
    }
}

impl From<ProjectCategoriesDto> for ProjectCategories {
    fn from(value: ProjectCategoriesDto) -> Self {
        ProjectCategories(value.0.into_iter().map(ProjectCategory::from).collect())
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProjectAttributes(pub Vec<ProjectAttribute>);

impl From<ProjectAttributes> for ProjectAttributesDto {
    fn from(value: ProjectAttributes) -> Self {
        ProjectAttributesDto(value.0.into_iter().map(ProjectAttributeDto::from).collect())
    }
}

impl From<ProjectAttributesDto> for ProjectAttributes {
    fn from(value: ProjectAttributesDto) -> Self {
        ProjectAttributes(value.0.into_iter().map(ProjectAttribute::from).collect())
    }
}
