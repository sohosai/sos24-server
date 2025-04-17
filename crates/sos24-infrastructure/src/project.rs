use anyhow::{anyhow, Context};
use futures_util::{StreamExt, TryStreamExt};
use sqlx::prelude::{FromRow, Type};

use sos24_domain::{
    entity::{
        common::datetime::DateTime,
        project::{
            Project, ProjectAttributes, ProjectCategory, ProjectGroupName, ProjectId, ProjectIndex,
            ProjectKanaGroupName, ProjectKanaTitle, ProjectLocationId, ProjectRemarks,
            ProjectTitle,
        },
        user::{User, UserEmail, UserId, UserKanaName, UserName, UserPhoneNumber},
    },
    repository::project::{ProjectRepository, ProjectRepositoryError, ProjectWithOwners},
};

use crate::shared::postgresql::Postgresql;

use super::user::UserRoleRow;

#[derive(FromRow)]
pub struct ProjectWithOwnersRow {
    // project
    project_id: uuid::Uuid,
    project_index: i32,
    project_title: String,
    project_kana_title: String,
    project_group_name: String,
    project_kana_group_name: String,
    project_category: ProjectCategoryRow,
    project_attributes: i32,
    project_owner_id: String,
    project_sub_owner_id: Option<String>,
    project_remarks: Option<String>,
    project_location_id: Option<String>,
    project_created_at: chrono::DateTime<chrono::Utc>,
    project_updated_at: chrono::DateTime<chrono::Utc>,

    // owner
    owner_id: String,
    owner_name: String,
    owner_kana_name: String,
    owner_email: String,
    owner_phone_number: String,
    owner_role: UserRoleRow,
    owner_created_at: chrono::DateTime<chrono::Utc>,
    owner_updated_at: chrono::DateTime<chrono::Utc>,

    // sub_owner
    sub_owner_id: Option<String>,
    sub_owner_name: Option<String>,
    sub_owner_kana_name: Option<String>,
    sub_owner_email: Option<String>,
    sub_owner_phone_number: Option<String>,
    sub_owner_role: Option<UserRoleRow>,
    sub_owner_created_at: Option<chrono::DateTime<chrono::Utc>>,
    sub_owner_updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl TryFrom<ProjectWithOwnersRow> for ProjectWithOwners {
    type Error = anyhow::Error;
    fn try_from(value: ProjectWithOwnersRow) -> Result<Self, Self::Error> {
        let project = Project::new(
            ProjectId::new(value.project_id),
            ProjectIndex::new(value.project_index),
            ProjectTitle::try_from(value.project_title)?,
            ProjectKanaTitle::new(value.project_kana_title),
            ProjectGroupName::try_from(value.project_group_name)?,
            ProjectKanaGroupName::new(value.project_kana_group_name),
            value.project_category.into(),
            ProjectAttributes::from_bits(value.project_attributes as u32)
                .ok_or(anyhow!("cannot convert project attributes"))?,
            UserId::new(value.project_owner_id),
            value.project_sub_owner_id.map(UserId::new),
            value.project_remarks.map(ProjectRemarks::new),
            value.project_location_id.map(ProjectLocationId::new),
            DateTime::new(value.project_created_at),
            DateTime::new(value.project_updated_at),
        );

        let owner = User::new(
            UserId::new(value.owner_id),
            UserName::new(value.owner_name),
            UserKanaName::new(value.owner_kana_name),
            UserEmail::try_from(value.owner_email)?,
            UserPhoneNumber::new(value.owner_phone_number),
            value.owner_role.into(),
            DateTime::new(value.owner_created_at),
            DateTime::new(value.owner_updated_at),
        );

        let sub_owner = match (
            value.sub_owner_id,
            value.sub_owner_name,
            value.sub_owner_kana_name,
            value.sub_owner_email,
            value.sub_owner_phone_number,
            value.sub_owner_role,
            value.sub_owner_created_at,
            value.sub_owner_updated_at,
        ) {
            (
                Some(sub_owner_id),
                Some(sub_owner_name),
                Some(sub_owner_kana_name),
                Some(sub_owner_email),
                Some(sub_owner_phone_number),
                Some(sub_owner_role),
                Some(sub_owner_created_at),
                Some(sub_owner_updated_at),
            ) => Some(User::new(
                UserId::new(sub_owner_id),
                UserName::new(sub_owner_name),
                UserKanaName::new(sub_owner_kana_name),
                UserEmail::try_from(sub_owner_email)?,
                UserPhoneNumber::new(sub_owner_phone_number),
                sub_owner_role.into(),
                DateTime::new(sub_owner_created_at),
                DateTime::new(sub_owner_updated_at),
            )),
            _ => None,
        };

        Ok(ProjectWithOwners {
            project,
            owner,
            sub_owner,
        })
    }
}

#[derive(Type)]
#[sqlx(type_name = "project_category", rename_all = "snake_case")]
pub enum ProjectCategoryRow {
    General,
    FoodsWithKitchen,
    FoodsWithoutKitchen,
    FoodsWithoutCooking,
    #[sqlx(rename = "stage_1a")]
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
    async fn create(&self, project: Project) -> Result<(), ProjectRepositoryError> {
        tracing::info!("企画を作成します");

        let project = project.destruct();
        sqlx::query!(
        r#"INSERT INTO projects (id, title, kana_title, group_name, kana_group_name, category, attributes, owner_id, remarks, location_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
        project.id.value(),
        project.title.value(),
        project.kana_title.value(),
        project.group_name.value(),
        project.kana_group_name.value(),
        ProjectCategoryRow::from(project.category) as ProjectCategoryRow,
        project.attributes.bits() as i32,
        project.owner_id.value(),
        project.remarks.map(|it| it.value()),
        project.location_id.map(|it| it.value())
        )
            .execute(&*self.db)
            .await
            .context("Failed to create project")?;

        tracing::info!("企画を作成しました");
        Ok(())
    }

    async fn update(&self, project: Project) -> Result<(), ProjectRepositoryError> {
        tracing::info!("企画を更新します");

        let project = project.destruct();
        sqlx::query!(
            r#"UPDATE projects
            SET title = $2, kana_title = $3, group_name = $4, kana_group_name = $5, category = $6, attributes = $7, owner_id = $8, sub_owner_id = $9, remarks = $10, location_id = $11
            WHERE id = $1 AND deleted_at IS NULL"#,
            project.id.value(),
            project.title.value(),
            project.kana_title.value(),
            project.group_name.value(),
            project.kana_group_name.value(),
            ProjectCategoryRow::from(project.category) as ProjectCategoryRow,
            project.attributes.bits() as i32,
            project.owner_id.value(),
            project.sub_owner_id.map(|it| it.value()),
            project.remarks.map(|it| it.value()),
            project.location_id.map(|it| it.value())
        )
            .execute(&*self.db)
            .await
            .context("Failed to update project")?;

        tracing::info!("企画を更新しました");
        Ok(())
    }

    async fn delete_by_id(&self, id: ProjectId) -> Result<(), ProjectRepositoryError> {
        tracing::info!("企画を削除します: {id:?}");

        sqlx::query!(
            r#"UPDATE projects
            SET deleted_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL"#,
            id.clone().value()
        )
        .execute(&*self.db)
        .await
        .context("Failed to delete project")?;

        tracing::info!("企画を削除しました: {id:?}");
        Ok(())
    }

    async fn list(&self) -> Result<Vec<ProjectWithOwners>, ProjectRepositoryError> {
        tracing::info!("企画一覧を取得します");

        // 現在のsqlxはLEFT JOINで得られるnullableなフィールドの型をうまく推論できないので、明示的に指定する
        // ref: https://github.com/launchbadge/sqlx/issues/2127
        let project_list = sqlx::query_as!(
            ProjectWithOwnersRow,
            r#"SELECT
            projects.id AS "project_id",
            projects.index AS "project_index",
            projects.title AS "project_title",
            projects.kana_title AS "project_kana_title",
            projects.group_name AS "project_group_name",
            projects.kana_group_name AS "project_kana_group_name",
            projects.category AS "project_category: ProjectCategoryRow",
            projects.attributes AS "project_attributes",
            projects.owner_id AS "project_owner_id",
            projects.sub_owner_id AS "project_sub_owner_id",
            projects.remarks AS "project_remarks",
            projects.location_id AS "project_location_id",
            projects.created_at AS "project_created_at",
            projects.updated_at AS "project_updated_at",
            owners.id AS "owner_id",
            owners.name AS "owner_name",
            owners.kana_name AS "owner_kana_name",
            owners.email AS "owner_email",
            owners.phone_number AS "owner_phone_number",
            owners.role AS "owner_role: UserRoleRow",
            owners.created_at AS "owner_created_at",
            owners.updated_at AS "owner_updated_at",
            sub_owners.id AS "sub_owner_id?",
            sub_owners.name AS "sub_owner_name?",
            sub_owners.kana_name AS "sub_owner_kana_name?",
            sub_owners.email AS "sub_owner_email?",
            sub_owners.phone_number AS "sub_owner_phone_number?",
            sub_owners.role AS "sub_owner_role?: UserRoleRow",
            sub_owners.created_at AS "sub_owner_created_at?",
            sub_owners.updated_at AS "sub_owner_updated_at?"
            FROM projects
            INNER JOIN users AS owners ON projects.owner_id = owners.id AND owners.deleted_at IS NULL
            LEFT JOIN users AS sub_owners ON projects.sub_owner_id = sub_owners.id AND sub_owners.deleted_at IS NULL
            WHERE projects.deleted_at IS NULL
            ORDER BY projects.index ASC"#
        )
        .fetch(&*self.db)
        .map(|row| ProjectWithOwners::try_from(row?))
        .try_collect()
        .await
        .context("Failed to fetch project list")?;

        tracing::info!("企画一覧を取得しました");
        Ok(project_list)
    }

    async fn find_by_id(
        &self,
        id: ProjectId,
    ) -> Result<Option<ProjectWithOwners>, ProjectRepositoryError> {
        tracing::info!("企画を取得します: {id:?}");

        let project_row = sqlx::query_as!(
            ProjectWithOwnersRow,
            r#"SELECT
            projects.id AS "project_id",
            projects.index AS "project_index",
            projects.title AS "project_title",
            projects.kana_title AS "project_kana_title",
            projects.group_name AS "project_group_name",
            projects.kana_group_name AS "project_kana_group_name",
            projects.category AS "project_category: ProjectCategoryRow",
            projects.attributes AS "project_attributes",
            projects.owner_id AS "project_owner_id",
            projects.sub_owner_id AS "project_sub_owner_id",
            projects.remarks AS "project_remarks",
            projects.location_id AS "project_location_id",
            projects.created_at AS "project_created_at",
            projects.updated_at AS "project_updated_at",
            owners.id AS "owner_id",
            owners.name AS "owner_name",
            owners.kana_name AS "owner_kana_name",
            owners.email AS "owner_email",
            owners.phone_number AS "owner_phone_number",
            owners.role AS "owner_role: UserRoleRow",
            owners.created_at AS "owner_created_at",
            owners.updated_at AS "owner_updated_at",
            sub_owners.id AS "sub_owner_id?",
            sub_owners.name AS "sub_owner_name?",
            sub_owners.kana_name AS "sub_owner_kana_name?",
            sub_owners.email AS "sub_owner_email?",
            sub_owners.phone_number AS "sub_owner_phone_number?",
            sub_owners.role AS "sub_owner_role?: UserRoleRow",
            sub_owners.created_at AS "sub_owner_created_at?",
            sub_owners.updated_at AS "sub_owner_updated_at?"
            FROM projects
            INNER JOIN users AS owners ON projects.owner_id = owners.id AND owners.deleted_at IS NULL
            LEFT JOIN users AS sub_owners ON projects.sub_owner_id = sub_owners.id AND sub_owners.deleted_at IS NULL
            WHERE projects.id = $1 AND projects.deleted_at IS NULL"#,
            id.clone().value()
        )
            .fetch_optional(&*self.db)
            .await
            .context("Failed to fetch project")?;

        tracing::info!("企画を取得しました: {id:?}");
        Ok(project_row.map(ProjectWithOwners::try_from).transpose()?)
    }

    async fn find_by_owner_id(
        &self,
        owner_id: UserId,
    ) -> Result<Option<ProjectWithOwners>, ProjectRepositoryError> {
        tracing::info!("企画責任者に紐づく企画を取得します: {owner_id:?}");

        let project_row = sqlx::query_as!(
            ProjectWithOwnersRow,
            r#"SELECT
            projects.id AS "project_id",
            projects.index AS "project_index",
            projects.title AS "project_title",
            projects.kana_title AS "project_kana_title",
            projects.group_name AS "project_group_name",
            projects.kana_group_name AS "project_kana_group_name",
            projects.category AS "project_category: ProjectCategoryRow",
            projects.attributes AS "project_attributes",
            projects.owner_id AS "project_owner_id",
            projects.sub_owner_id AS "project_sub_owner_id",
            projects.remarks AS "project_remarks",
            projects.location_id AS "project_location_id",
            projects.created_at AS "project_created_at",
            projects.updated_at AS "project_updated_at",
            owners.id AS "owner_id",
            owners.name AS "owner_name",
            owners.kana_name AS "owner_kana_name",
            owners.email AS "owner_email",
            owners.phone_number AS "owner_phone_number",
            owners.role AS "owner_role: UserRoleRow",
            owners.created_at AS "owner_created_at",
            owners.updated_at AS "owner_updated_at",
            sub_owners.id AS "sub_owner_id?",
            sub_owners.name AS "sub_owner_name?",
            sub_owners.kana_name AS "sub_owner_kana_name?",
            sub_owners.email AS "sub_owner_email?",
            sub_owners.phone_number AS "sub_owner_phone_number?",
            sub_owners.role AS "sub_owner_role?: UserRoleRow",
            sub_owners.created_at AS "sub_owner_created_at?",
            sub_owners.updated_at AS "sub_owner_updated_at?"
            FROM projects
            INNER JOIN users AS owners ON projects.owner_id = owners.id AND owners.deleted_at IS NULL
            LEFT JOIN users AS sub_owners ON projects.sub_owner_id = sub_owners.id AND sub_owners.deleted_at IS NULL
            WHERE projects.owner_id = $1 AND projects.deleted_at IS NULL"#,
            owner_id.clone().value()
        )
            .fetch_optional(&*self.db)
            .await
            .context("Failed to fetch project")?;

        tracing::info!("企画責任者に紐づく企画を取得しました: {owner_id:?}");
        Ok(project_row.map(ProjectWithOwners::try_from).transpose()?)
    }

    async fn find_by_sub_owner_id(
        &self,
        sub_owner_id: UserId,
    ) -> Result<Option<ProjectWithOwners>, ProjectRepositoryError> {
        tracing::info!("副企画責任者に紐づく企画を取得します: {sub_owner_id:?}");

        let project_row = sqlx::query_as!(
            ProjectWithOwnersRow,
            r#"SELECT
            projects.id AS "project_id",
            projects.index AS "project_index",
            projects.title AS "project_title",
            projects.kana_title AS "project_kana_title",
            projects.group_name AS "project_group_name",
            projects.kana_group_name AS "project_kana_group_name",
            projects.category AS "project_category: ProjectCategoryRow",
            projects.attributes AS "project_attributes",
            projects.owner_id AS "project_owner_id",
            projects.sub_owner_id AS "project_sub_owner_id",
            projects.remarks AS "project_remarks",
            projects.location_id AS "project_location_id",
            projects.created_at AS "project_created_at",
            projects.updated_at AS "project_updated_at",
            owners.id AS "owner_id",
            owners.name AS "owner_name",
            owners.kana_name AS "owner_kana_name",
            owners.email AS "owner_email",
            owners.phone_number AS "owner_phone_number",
            owners.role AS "owner_role: UserRoleRow",
            owners.created_at AS "owner_created_at",
            owners.updated_at AS "owner_updated_at",
            sub_owners.id AS "sub_owner_id?",
            sub_owners.name AS "sub_owner_name?",
            sub_owners.kana_name AS "sub_owner_kana_name?",
            sub_owners.email AS "sub_owner_email?",
            sub_owners.phone_number AS "sub_owner_phone_number?",
            sub_owners.role AS "sub_owner_role?: UserRoleRow",
            sub_owners.created_at AS "sub_owner_created_at?",
            sub_owners.updated_at AS "sub_owner_updated_at?"
            FROM projects
            INNER JOIN users AS owners ON projects.owner_id = owners.id AND owners.deleted_at IS NULL
            LEFT JOIN users AS sub_owners ON projects.sub_owner_id = sub_owners.id AND sub_owners.deleted_at IS NULL
            WHERE projects.sub_owner_id = $1 AND projects.deleted_at IS NULL"#,
            sub_owner_id.clone().value()
        ).fetch_optional(&*self.db)
            .await
            .context("Failed to fetch project")?;

        tracing::info!("副企画責任者に紐づく企画を取得しました: {sub_owner_id:?}");
        Ok(project_row.map(ProjectWithOwners::try_from).transpose()?)
    }
}
