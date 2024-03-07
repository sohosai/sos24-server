use anyhow::Context;
use sos24_domain::{
    entity::{
        common::date::WithDate,
        user::{
            User, UserCategory, UserEmail, UserId, UserKanaName, UserName, UserPhoneNumber,
            UserRole,
        },
    },
    repository::user::UserRepository,
};
use sqlx::prelude::*;

use super::Postgresql;

#[derive(FromRow)]
pub struct UserRow {
    id: String,

    name: String,
    kana_name: String,

    email: String,
    phone_number: String,
    role: UserRoleRow,
    category: UserCategoryRow,

    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl TryFrom<UserRow> for WithDate<User> {
    type Error = anyhow::Error;
    fn try_from(value: UserRow) -> Result<Self, Self::Error> {
        Ok(WithDate::new(
            User {
                id: UserId::new(value.id),
                name: UserName::new(value.name),
                kana_name: UserKanaName::new(value.kana_name),
                email: UserEmail::try_from(value.email)?,
                phone_number: UserPhoneNumber::new(value.phone_number),
                role: UserRole::from(value.role),
                category: UserCategory::from(value.category),
            },
            value.created_at,
            value.updated_at,
            value.deleted_at,
        ))
    }
}

#[derive(Type)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub enum UserRoleRow {
    General,
}

impl From<UserRoleRow> for UserRole {
    fn from(value: UserRoleRow) -> Self {
        match value {
            UserRoleRow::General => UserRole::General,
        }
    }
}

impl From<UserRole> for UserRoleRow {
    fn from(value: UserRole) -> Self {
        match value {
            UserRole::General => UserRoleRow::General,
        }
    }
}

#[derive(Type)]
#[sqlx(type_name = "user_category", rename_all = "snake_case")]
pub enum UserCategoryRow {
    UndergraduateStudent,
    GraduateStudent,
    AcademicStaff,
}

impl From<UserCategoryRow> for UserCategory {
    fn from(value: UserCategoryRow) -> Self {
        match value {
            UserCategoryRow::UndergraduateStudent => UserCategory::UndergraduateStudent,
            UserCategoryRow::GraduateStudent => UserCategory::GraduateStudent,
            UserCategoryRow::AcademicStaff => UserCategory::AcademicStaff,
        }
    }
}

impl From<UserCategory> for UserCategoryRow {
    fn from(value: UserCategory) -> Self {
        match value {
            UserCategory::UndergraduateStudent => UserCategoryRow::UndergraduateStudent,
            UserCategory::GraduateStudent => UserCategoryRow::GraduateStudent,
            UserCategory::AcademicStaff => UserCategoryRow::AcademicStaff,
        }
    }
}

#[derive(Clone)]
pub struct PgUserRepository {
    db: Postgresql,
}

impl PgUserRepository {
    pub fn new(db: Postgresql) -> Self {
        Self { db }
    }
}

impl UserRepository for PgUserRepository {
    async fn create(&self, user: User) -> anyhow::Result<()> {
        sqlx::query!(
          r#"INSERT INTO users (id, name, kana_name, email, phone_number, category) VALUES ($1, $2, $3, $4, $5, $6)"#,
          user.id.value(),
          user.name.value(),
          user.kana_name.value(),
          user.email.value(),
          user.phone_number.value(),
          UserCategoryRow::from(user.category) as UserCategoryRow,
        )
        .execute(&*self.db)
        .await
        .context("Failed to create user")?;

        Ok(())
    }

    async fn find_by_id(&self, id: UserId) -> anyhow::Result<Option<WithDate<User>>> {
        let user_row = sqlx::query_as!(
            UserRow,
            r#"SELECT id, name, kana_name, email, phone_number, category AS "category: UserCategoryRow", role AS "role: UserRoleRow", created_at, updated_at, deleted_at FROM users WHERE id = $1 AND deleted_at IS NULL"#,
            id.value(),
        )
        .fetch_optional(&*self.db)
        .await
        .context("Failed to fetch user by id")?;

        user_row.map(WithDate::try_from).transpose()
    }
}
