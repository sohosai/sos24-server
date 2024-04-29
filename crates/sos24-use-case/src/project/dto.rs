use std::fmt;
use std::fmt::Formatter;

use sos24_domain::entity::project::ProjectCategories;
use sos24_domain::entity::project_application_period::ProjectApplicationPeriod;
use sos24_domain::entity::user::User;
use sos24_domain::entity::{
    common::date::WithDate,
    project::{Project, ProjectAttributes, ProjectCategory},
};

#[derive(Debug)]
pub struct ProjectDto {
    pub id: String,
    pub index: i32,
    pub title: String,
    pub kana_title: String,
    pub group_name: String,
    pub kana_group_name: String,
    pub category: ProjectCategoryDto,
    pub attributes: ProjectAttributesDto,
    pub owner_id: String,
    pub owner_name: String,
    pub owner_email: String,
    pub sub_owner_id: Option<String>,
    pub sub_owner_name: Option<String>,
    pub sub_owner_email: Option<String>,
    pub remarks: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<(WithDate<Project>, WithDate<User>, Option<WithDate<User>>)> for ProjectDto {
    fn from(entity: (WithDate<Project>, WithDate<User>, Option<WithDate<User>>)) -> Self {
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
            category: ProjectCategoryDto::from(project.category),
            attributes: ProjectAttributesDto::from(project.attributes),
            owner_id: project.owner_id.value(),
            owner_name: owner.name.value(),
            owner_email: owner.email.value(),
            sub_owner_id: project.sub_owner_id.map(|id| id.value()),
            sub_owner_name,
            sub_owner_email,
            remarks: project.remarks.map(|it| it.value()),
            created_at: project_entity.created_at,
            updated_at: project_entity.updated_at,
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

impl From<ProjectCategoryDto> for ProjectCategory {
    fn from(value: ProjectCategoryDto) -> Self {
        match value {
            ProjectCategoryDto::General => Self::General,
            ProjectCategoryDto::FoodsWithKitchen => Self::FoodsWithKitchen,
            ProjectCategoryDto::FoodsWithoutKitchen => Self::FoodsWithoutKitchen,
            ProjectCategoryDto::FoodsWithoutCooking => Self::FoodsWithoutCooking,
            ProjectCategoryDto::Stage1A => Self::Stage1A,
            ProjectCategoryDto::StageUniversityHall => Self::StageUniversityHall,
            ProjectCategoryDto::StageUnited => Self::StageUnited,
        }
    }
}

impl From<ProjectCategory> for ProjectCategoryDto {
    fn from(value: ProjectCategory) -> Self {
        match value {
            ProjectCategory::General => Self::General,
            ProjectCategory::FoodsWithKitchen => Self::FoodsWithKitchen,
            ProjectCategory::FoodsWithoutKitchen => Self::FoodsWithoutKitchen,
            ProjectCategory::FoodsWithoutCooking => Self::FoodsWithoutCooking,
            ProjectCategory::Stage1A => Self::Stage1A,
            ProjectCategory::StageUniversityHall => Self::StageUniversityHall,
            ProjectCategory::StageUnited => Self::StageUnited,
        }
    }
}

#[derive(Debug)]
pub struct ProjectCategoriesDto(pub Vec<ProjectCategoryDto>);

impl From<ProjectCategoriesDto> for ProjectCategories {
    fn from(value: ProjectCategoriesDto) -> Self {
        let res = value
            .0
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
        res
    }
}

impl From<ProjectCategories> for ProjectCategoriesDto {
    fn from(value: ProjectCategories) -> Self {
        Self(
            value
                .into_iter()
                .map(|category| match category {
                    ProjectCategories::GENERAL => ProjectCategoryDto::General,
                    ProjectCategories::FOODS_WITH_KITCHEN => ProjectCategoryDto::FoodsWithKitchen,
                    ProjectCategories::FOODS_WITHOUT_KITCHEN => {
                        ProjectCategoryDto::FoodsWithoutKitchen
                    }
                    ProjectCategories::FOODS_WITHOUT_COOKING => {
                        ProjectCategoryDto::FoodsWithoutCooking
                    }
                    ProjectCategories::STAGE_1A => ProjectCategoryDto::Stage1A,
                    ProjectCategories::STAGE_UNIVERSITY_HALL => {
                        ProjectCategoryDto::StageUniversityHall
                    }
                    ProjectCategories::STAGE_UNITED => ProjectCategoryDto::StageUnited,
                    _ => panic!("unknown project category: {category:?}"),
                })
                .collect(),
        )
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

#[derive(Debug)]
pub struct ProjectAttributesDto(pub Vec<ProjectAttributeDto>);

impl From<ProjectAttributesDto> for ProjectAttributes {
    fn from(value: ProjectAttributesDto) -> Self {
        let res = value
            .0
            .into_iter()
            .map(|attribute| match attribute {
                ProjectAttributeDto::Academic => ProjectAttributes::ACADEMIC,
                ProjectAttributeDto::Art => ProjectAttributes::ART,
                ProjectAttributeDto::Official => ProjectAttributes::OFFICIAL,
                ProjectAttributeDto::Inside => ProjectAttributes::INSIDE,
                ProjectAttributeDto::Outside => ProjectAttributes::OUTSIDE,
            })
            .fold(ProjectAttributes::empty(), |acc, attribute| acc | attribute);
        res
    }
}

impl From<ProjectAttributes> for ProjectAttributesDto {
    fn from(value: ProjectAttributes) -> Self {
        Self(
            value
                .into_iter()
                .map(|attribute| match attribute {
                    ProjectAttributes::ACADEMIC => ProjectAttributeDto::Academic,
                    ProjectAttributes::ART => ProjectAttributeDto::Art,
                    ProjectAttributes::OFFICIAL => ProjectAttributeDto::Official,
                    ProjectAttributes::INSIDE => ProjectAttributeDto::Inside,
                    ProjectAttributes::OUTSIDE => ProjectAttributeDto::Outside,
                    _ => panic!("unknown project attribute: {attribute:?}"),
                })
                .collect(),
        )
    }
}

#[derive(Debug)]
pub struct ProjectApplicationPeriodDto {
    pub start_at: String,
    pub end_at: String,
}

impl From<ProjectApplicationPeriod> for ProjectApplicationPeriodDto {
    fn from(entity: ProjectApplicationPeriod) -> Self {
        Self {
            start_at: entity.start_at().to_rfc3339(),
            end_at: entity.end_at().to_rfc3339(),
        }
    }
}
