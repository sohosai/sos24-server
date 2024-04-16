use serde::{Deserialize, Serialize};
use sos24_use_case::project::dto::{
    ProjectAttributeDto, ProjectAttributesDto, ProjectCategoriesDto, ProjectCategoryDto,
    ProjectWithOwnersDto,
};

pub mod delete_by_id;
pub mod export;
pub mod get;
pub mod get_by_id;
pub mod get_me;
pub mod post;
pub mod put_by_id;

#[derive(Debug, Serialize)]
pub struct ProjectWithOwners {
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
    created_at: String,
    updated_at: String,
    deleted_at: Option<String>,
}

impl From<ProjectWithOwnersDto> for ProjectWithOwners {
    fn from(dto: ProjectWithOwnersDto) -> Self {
        let (sub_owner_id, sub_owner_name, sub_owner_email) =
            dto.sub_owner.map_or((None, None, None), |it| {
                (Some(it.id), Some(it.name), Some(it.email))
            });
        ProjectWithOwners {
            id: dto.project.id,
            index: dto.project.index,
            title: dto.project.title,
            kana_title: dto.project.kana_title,
            group_name: dto.project.group_name,
            kana_group_name: dto.project.kana_group_name,
            category: ProjectCategory::from(dto.project.category),
            attributes: ProjectAttributes::from(dto.project.attributes),
            owner_id: dto.owner.id,
            owner_name: dto.owner.name,
            owner_email: dto.owner.email,
            sub_owner_id,
            sub_owner_name,
            sub_owner_email,
            remarks: dto.project.remarks,
            created_at: dto.project.created_at.to_rfc3339(),
            updated_at: dto.project.updated_at.to_rfc3339(),
            deleted_at: dto.project.deleted_at.map(|it| it.to_rfc3339()),
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

impl std::fmt::Display for ProjectCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectCategory::General => write!(f, "普通企画"),
            ProjectCategory::FoodsWithKitchen => write!(f, "調理企画（仕込み場が必要）"),
            ProjectCategory::FoodsWithoutKitchen => write!(f, "調理企画（仕込み場が不要）"),
            ProjectCategory::FoodsWithoutCooking => write!(f, "既成食品販売企画"),
            ProjectCategory::Stage1A => write!(f, "ステージ企画(1Aステージ)"),
            ProjectCategory::StageUniversityHall => write!(f, "ステージ企画(大学会館ステージ)"),
            ProjectCategory::StageUnited => write!(f, "ステージ企画(UNITEDステージ)"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectCategories(Vec<ProjectCategory>);

impl From<ProjectCategoriesDto> for ProjectCategories {
    fn from(categories: ProjectCategoriesDto) -> Self {
        ProjectCategories(
            categories
                .0
                .into_iter()
                .map(ProjectCategory::from)
                .collect(),
        )
    }
}

impl From<ProjectCategories> for ProjectCategoriesDto {
    fn from(categories: ProjectCategories) -> Self {
        ProjectCategoriesDto(
            categories
                .0
                .into_iter()
                .map(ProjectCategoryDto::from)
                .collect(),
        )
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

impl std::fmt::Display for ProjectAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectAttribute::Academic => write!(f, "学術企画"),
            ProjectAttribute::Art => write!(f, "芸術際企画"),
            ProjectAttribute::Official => write!(f, "委員会開催企画"),
            ProjectAttribute::Inside => write!(f, "屋内企画"),
            ProjectAttribute::Outside => write!(f, "屋外企画"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectAttributes(Vec<ProjectAttribute>);

impl From<ProjectAttributesDto> for ProjectAttributes {
    fn from(attributes: ProjectAttributesDto) -> Self {
        ProjectAttributes(
            attributes
                .0
                .into_iter()
                .map(ProjectAttribute::from)
                .collect(),
        )
    }
}

impl From<ProjectAttributes> for ProjectAttributesDto {
    fn from(attributes: ProjectAttributes) -> Self {
        ProjectAttributesDto(
            attributes
                .0
                .into_iter()
                .map(ProjectAttributeDto::from)
                .collect(),
        )
    }
}

impl std::fmt::Display for ProjectAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .0
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(";");
        write!(f, "{s}")
    }
}
