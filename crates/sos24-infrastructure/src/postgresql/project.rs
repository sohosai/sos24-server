use anyhow::Context;
use sos24_domain::{
    entity::project::{Project, ProjectCategory},
    repository::project::{ProjectRepository, ProjectRepositoryError},
};
use sqlx::prelude::Type;

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
    remarks: String,
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
        project.remarks.value(),
        )
        .execute(&*self.db)
        .await
        .context("Failed to create project")?;
        Ok(())
    }
}
