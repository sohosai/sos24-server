use std::ops::BitOr;

use sos24_domain::entity::{
    common::date::WithDate,
    project::{Project, ProjectAttributes, ProjectCategories, ProjectCategory},
    user::User,
};

use crate::user::dto::UserDto;

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
    pub sub_owner_id: Option<String>,
    pub remarks: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<WithDate<Project>> for ProjectDto {
    fn from(entity: WithDate<Project>) -> Self {
        let project = entity.value.destruct();
        ProjectDto {
            id: project.id.value().to_string(),
            index: project.index.value(),
            title: project.title.value(),
            kana_title: project.kana_title.value(),
            group_name: project.group_name.value(),
            kana_group_name: project.kana_group_name.value(),
            category: ProjectCategoryDto::from(project.category),
            attributes: ProjectAttributesDto::from(project.attributes),
            owner_id: project.owner_id.value().to_string(),
            sub_owner_id: project.sub_owner_id.map(|id| id.value().to_string()),
            remarks: project.remarks.map(|it| it.value()),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
        }
    }
}

pub struct ProjectWithOwnersDto {
    pub project: ProjectDto,
    pub owner: UserDto,
    pub sub_owner: Option<UserDto>,
}

impl From<(WithDate<Project>, WithDate<User>, Option<WithDate<User>>)> for ProjectWithOwnersDto {
    fn from(entity: (WithDate<Project>, WithDate<User>, Option<WithDate<User>>)) -> Self {
        let (project, owner, sub_owner) = entity;
        ProjectWithOwnersDto {
            project: ProjectDto::from(project),
            owner: UserDto::from(owner),
            sub_owner: sub_owner.map(UserDto::from),
        }
    }
}

pub enum ProjectCategoryDto {
    General,
    FoodsWithKitchen,
    FoodsWithoutKitchen,
    FoodsWithoutCooking,
    Stage1A,
    StageUniversityHall,
    StageUnited,
}

impl From<ProjectCategoryDto> for ProjectCategory {
    fn from(dto: ProjectCategoryDto) -> Self {
        match dto {
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

impl From<ProjectCategory> for ProjectCategoryDto {
    fn from(entity: ProjectCategory) -> Self {
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

pub struct ProjectCategoriesDto(pub Vec<ProjectCategoryDto>);

impl From<ProjectCategoriesDto> for ProjectCategories {
    fn from(dto: ProjectCategoriesDto) -> Self {
        dto.0
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
            .fold(ProjectCategories::empty(), ProjectCategories::bitor)
    }
}

impl From<ProjectCategories> for ProjectCategoriesDto {
    fn from(entity: ProjectCategories) -> Self {
        let inner = entity
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
            .collect();
        ProjectCategoriesDto(inner)
    }
}

pub enum ProjectAttributeDto {
    Academic,
    Art,
    Official,
    Inside,
    Outside,
}

pub struct ProjectAttributesDto(pub Vec<ProjectAttributeDto>);

impl From<ProjectAttributesDto> for ProjectAttributes {
    fn from(dto: ProjectAttributesDto) -> Self {
        dto.0
            .into_iter()
            .map(|attribute| match attribute {
                ProjectAttributeDto::Academic => ProjectAttributes::ACADEMIC,
                ProjectAttributeDto::Art => ProjectAttributes::ART,
                ProjectAttributeDto::Official => ProjectAttributes::OFFICIAL,
                ProjectAttributeDto::Inside => ProjectAttributes::INSIDE,
                ProjectAttributeDto::Outside => ProjectAttributes::OUTSIDE,
            })
            .fold(ProjectAttributes::empty(), ProjectAttributes::bitor)
    }
}

impl From<ProjectAttributes> for ProjectAttributesDto {
    fn from(entity: ProjectAttributes) -> Self {
        let inner = entity
            .into_iter()
            .map(|attribute| match attribute {
                ProjectAttributes::ACADEMIC => ProjectAttributeDto::Academic,
                ProjectAttributes::ART => ProjectAttributeDto::Art,
                ProjectAttributes::OFFICIAL => ProjectAttributeDto::Official,
                ProjectAttributes::INSIDE => ProjectAttributeDto::Inside,
                ProjectAttributes::OUTSIDE => ProjectAttributeDto::Outside,
                _ => panic!("unknown project attribute: {attribute:?}"),
            })
            .collect();
        ProjectAttributesDto(inner)
    }
}
