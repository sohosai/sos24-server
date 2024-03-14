use anyhow::Context;
use futures_util::{StreamExt, TryStreamExt};
use sos24_domain::{
    entity::{
        common::date::WithDate,
        project::{
            Project, ProjectAttributes, ProjectCategory, ProjectGroupName, ProjectId, ProjectIndex,
            ProjectKanaGroupName, ProjectKanaTitle, ProjectRemarks, ProjectTitle,
        },
        user::UserId,
    },
    repository::project::{ProjectRepository, ProjectRepositoryError},
};
use sqlx::prelude::{FromRow, Type};

use super::Postgresql;

#[derive(FromRow)]
pub struct ProjectRow {
    id: uuid::Uuid,
    index: i32,
    title: String,
    kana_title: String,
    group_name: String,
    kana_group_name: String,
    category: ProjectCategoryRow,
    attributes: i32,
    owner_id: String,
    sub_owner_id: Option<String>,
    remarks: Option<String>,

    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<ProjectRow> for WithDate<Project> {
    fn from(value: ProjectRow) -> Self {
        WithDate::new(
            Project::new(
                ProjectId::new(value.id),
                ProjectIndex::new(value.index),
                ProjectTitle::new(value.title),
                ProjectKanaTitle::new(value.kana_title),
                ProjectGroupName::new(value.group_name),
                ProjectKanaGroupName::new(value.kana_group_name),
                value.category.into(),
                ProjectAttributes::new(value.attributes),
                UserId::new(value.owner_id),
                value.sub_owner_id.map(UserId::new),
                value.remarks.map(ProjectRemarks::new),
            ),
            value.created_at,
            value.updated_at,
            value.deleted_at,
        )
    }
}

#[derive(Type)]
#[sqlx(type_name = "project_category", rename_all = "snake_case")]
pub enum ProjectCategoryRow {
    General,
    FoodsWithKitchen,
    FoodsWithoutKitchen,
    FoodsWithoutCooking,
    Stage1A,
    StageUniversityHall,
    StageUnited,
}

impl From<ProjectCategoryRow> for ProjectCategory {
    fn from(value: ProjectCategoryRow) -> Self {
        match value {
            ProjectCategoryRow::General => ProjectCategory::General,
            ProjectCategoryRow::FoodsWithKitchen => ProjectCategory::FoodsWithKitchen,
            ProjectCategoryRow::FoodsWithoutKitchen => ProjectCategory::FoodsWithoutKitchen,
            ProjectCategoryRow::FoodsWithoutCooking => ProjectCategory::FoodsWithoutCooking,
            ProjectCategoryRow::Stage1A => ProjectCategory::Stage1A,
            ProjectCategoryRow::StageUniversityHall => ProjectCategory::StageUniversityHall,
            ProjectCategoryRow::StageUnited => ProjectCategory::StageUnited,
        }
    }
}

impl From<ProjectCategory> for ProjectCategoryRow {
    fn from(value: ProjectCategory) -> Self {
        match value {
            ProjectCategory::General => ProjectCategoryRow::General,
            ProjectCategory::FoodsWithKitchen => ProjectCategoryRow::FoodsWithKitchen,
            ProjectCategory::FoodsWithoutKitchen => ProjectCategoryRow::FoodsWithoutKitchen,
            ProjectCategory::FoodsWithoutCooking => ProjectCategoryRow::FoodsWithoutCooking,
            ProjectCategory::Stage1A => ProjectCategoryRow::Stage1A,
            ProjectCategory::StageUniversityHall => ProjectCategoryRow::StageUniversityHall,
            ProjectCategory::StageUnited => ProjectCategoryRow::StageUnited,
        }
    }
}

#[derive(Clone)]
pub struct PgProjectRepository {
    db: Postgresql,
}

impl PgProjectRepository {
    pub fn new(db: Postgresql) -> Self {
        Self { db }
    }
}

impl ProjectRepository for PgProjectRepository {
    async fn list(&self) -> Result<Vec<WithDate<Project>>, ProjectRepositoryError> {
        let project_list = sqlx::query_as!(
            ProjectRow,
            r#"SELECT id, index, title, kana_title, group_name, kana_group_name, category AS "category: ProjectCategoryRow", attributes, owner_id, sub_owner_id, remarks, created_at, updated_at, deleted_at FROM projects WHERE deleted_at IS NULL"#
        )
        .fetch(&*self.db)
        .map(|row| Ok::<_, anyhow::Error>(WithDate::from(row?)))
        .try_collect()
        .await
        .context("Failed to fetch project list")?;
        Ok(project_list)
    }

    async fn create(&self, project: Project) -> Result<(), ProjectRepositoryError> {
        let project = project.destruct();
        sqlx::query!(
        r#"INSERT INTO projects (id, title, kana_title, group_name, kana_group_name, category, attributes, owner_id, remarks) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
        project.id.value(),
        project.title.value(),
        project.kana_title.value(),
        project.group_name.value(),
        project.kana_group_name.value(),
        ProjectCategoryRow::from(project.category) as ProjectCategoryRow,
        project.attributes.value(),
        project.owner_id.value(),
        project.remarks.map(|it| it.value()),
        )
        .execute(&*self.db)
        .await
        .context("Failed to create project")?;
        Ok(())
    }
}
